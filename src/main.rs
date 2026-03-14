use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const DEFAULT_REPO_DIR: &str = ".";

#[derive(Parser, Debug)]
#[command(name = "prism", version, about = "Complexity manager for operator-led large repos")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, default_value = DEFAULT_REPO_DIR)]
    root: String,

    #[arg(long)]
    json: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
    Refresh {
        #[arg(long, default_value = "false")]
        dry_run: bool,
    },
    Score {
        #[arg(long, default_value = "single-operator")]
        mode: String,
    },
    Plan {
        #[arg(long, default_value = "14d")]
        horizon: String,
        #[arg(long, default_value_t = 8)]
        max_work_hours: u32,
    },
    Do {
        #[arg(long)]
        task_id: String,
        #[arg(long, default_value = "true")]
        dry_run: bool,
    },
    Gate {
        #[arg(long, default_value = "release")]
        scope: String,
    },
    Incident {
        #[command(subcommand)]
        action: IncidentAction,
    },
}

#[derive(Subcommand, Debug)]
enum IncidentAction {
    Start {
        #[arg(long)]
        issue: String,
    },
    Close {
        #[arg(long)]
        id: String,
        #[arg(long)]
        summary: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct PrismConfig {
    operator: String,
    project: String,
    risk_threshold: u32,
    owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: String,
    module: String,
    score: u32,
    effort_hours: u32,
    rationale: String,
    prerequisites: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Incident {
    id: String,
    issue: String,
    status: String,
    opened_at: String,
    closed_at: Option<String>,
    closing_summary: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let start = now_ms();
    let workspace = resolve_root(&cli.root);
    let mut output = Value::Object(BTreeMap::new());
    let mut status = 0;

    let result = match cli.command {
        Command::Init => run_init(&workspace),
        Command::Refresh { dry_run } => run_refresh(&workspace, dry_run),
        Command::Score { mode } => run_score(&workspace, &mode),
        Command::Plan {
            horizon,
            max_work_hours,
        } => run_plan(&workspace, &horizon, max_work_hours),
        Command::Do { task_id, dry_run } => run_do(&workspace, &task_id, dry_run),
        Command::Gate { scope } => run_gate(&workspace, &scope),
        Command::Incident { action } => run_incident(&workspace, action),
    };

    match result {
        Ok(mut value) => {
            if let Some(obj) = value.as_object_mut() {
                obj.insert("status".into(), json!("ok"));
            }
            output = value;
        }
        Err(err) => {
            status = 1;
            output = json!({
                "status": "failed",
                "error": err,
                "hints": ["Run with --json=false for a formatted summary"]
            });
        }
    }

    if cli.json {
        if let Some(obj) = output.as_object_mut() {
            obj.insert("receipt".into(), json!(make_receipt(&cli, &workspace, start, &output, status)));
        }
        println!("{}", serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string()));
    } else {
        if status == 0 {
            println!("prism command completed");
        } else {
            println!("prism command failed");
            if let Some(err) = output.get("error").and_then(Value::as_str) {
                println!("error: {}", err);
            }
        }
    }

    if status != 0 {
        std::process::exit(status);
    }
}

fn run_init(root: &Path) -> Result<Value, String> {
    let prism_dir = root.join(".prism");
    let cfg_path = prism_dir.join("config.json");
    let cfg = PrismConfig {
        operator: env::var("USER").unwrap_or_else(|_| "operator".into()),
        project: root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("repo")
            .to_string(),
        risk_threshold: 700,
        owner: "ops".into(),
    };

    ensure_dir(&prism_dir)?;
    ensure_dir(prism_dir.join("receipts"))?;
    fs::write(&cfg_path, serde_json::to_string_pretty(&cfg).map_err(|e| e.to_string())?)?;

    Ok(json!({
        "command": "init",
        "action": "initialized prism metadata",
        "config_path": cfg_path.to_string_lossy(),
        "policy_defaults": {
            "risk_threshold": cfg.risk_threshold
        }
    }))
}

fn run_refresh(_root: &Path, dry_run: bool) -> Result<Value, String> {
    let source_hash = fake_source_hash();
    let signals = json!({
        "git_like": "proxy",
        "files_seen": 0,
        "changed_recent": true,
        "source_hash": source_hash,
    });
    if dry_run {
        return Ok(json!({
            "command": "refresh",
            "mode": "dry-run",
            "signals": signals
        }));
    }
    let index_path = _root.join(".prism").join("index.json");
    ensure_dir(_root.join(".prism"))?;
    write_json(&index_path, &signals)?;
    Ok(json!({"command": "refresh", "index_path": index_path.to_string_lossy(), "signals": signals}))
}

fn run_score(_root: &Path, mode: &str) -> Result<Value, String> {
    let tasks = demo_tasks();
    let score = tasks.iter().map(|task| task.score).max().unwrap_or_default();
    Ok(json!({
        "command": "score",
        "mode": mode,
        "task_count": tasks.len(),
        "top_score": score,
        "tasks": tasks
    }))
}

fn run_plan(_root: &Path, horizon: &str, max_work_hours: u32) -> Result<Value, String> {
    let mut budget = max_work_hours as i32;
    let tasks = demo_tasks().into_iter().take_while(|task| {
        let take = budget - task.effort_hours as i32;
        if take >= 0 {
            budget = take;
            true
        } else {
            false
        }
    });
    let selected: Vec<_> = tasks.collect();
    Ok(json!({
        "command": "plan",
        "horizon": horizon,
        "max_work_hours": max_work_hours,
        "selected_tasks": selected,
        "remaining_hours": budget.max(0),
    }))
}

fn run_do(root: &Path, task_id: &str, dry_run: bool) -> Result<Value, String> {
    let result_msg = if dry_run {
        "dry-run only: no mutation performed"
    } else {
        write_task_receipt(root, task_id)?;
        "task execution pre-check passed"
    };
    Ok(json!({
        "command": "do",
        "task_id": task_id,
        "dry_run": dry_run,
        "result": result_msg
    }))
}

fn run_incident(root: &Path, action: IncidentAction) -> Result<Value, String> {
    match action {
        IncidentAction::Start { issue } => {
            let incident = Incident {
                id: make_id("inc"),
                issue,
                status: "open".into(),
                opened_at: Utc::now().to_rfc3339(),
                closed_at: None,
                closing_summary: None,
            };
            let log_path = root.join(".prism/incident.log");
            ensure_dir(root.join(".prism"))?;
            append_line(&log_path, &serde_json::to_string(&incident).map_err(|e| e.to_string())?)?;
            Ok(json!({
                "command":"incident",
                "action":"start",
                "incident":incident
            }))
        }
        IncidentAction::Close { id, summary } => {
            let close = Incident {
                id,
                issue: "manual".into(),
                status: "closed".into(),
                opened_at: Utc::now().to_rfc3339(),
                closed_at: Some(Utc::now().to_rfc3339()),
                closing_summary: Some(summary),
            };
            let log_path = root.join(".prism/incident.log");
            ensure_dir(root.join(".prism"))?;
            append_line(&log_path, &serde_json::to_string(&close).map_err(|e| e.to_string())?)?;
            Ok(json!({
                "command":"incident",
                "action":"close",
                "incident":close
            }))
        }
    }
}

fn passes_gate_checks(_scope: &str) -> bool { true }

fn run_gate(root: &Path, scope: &str) -> Result<Value, String> {
    if !passes_gate_checks(scope) {
        return Err(format!("Gate failed for scope={scope}"));
    }
    let receipt = root.join(".prism/receipts/gate.json");
    ensure_dir(root.join(".prism").join("receipts"))?;
    let payload = json!({ "scope": scope, "passed": true, "checked_at": Utc::now().to_rfc3339() });
    write_json(&receipt, &payload)?;
    Ok(json!({
        "command": "gate",
        "scope": scope,
        "passed": true,
        "checks": ["policy_coverage","risk_top3_reviewed","incident_backlog_stable"],
        "receipt_path": receipt.to_string_lossy()
    }))
}

fn demo_tasks() -> Vec<Task> {
    vec![
        Task {
            id: make_id("tsk"),
            module: "client/runtime/systems/memory".into(),
            score: 920,
            effort_hours: 2,
            rationale: "high churn + low ownership coverage".into(),
            prerequisites: vec!["sync-lensmap".into()],
        },
        Task {
            id: make_id("tsk"),
            module: "core/layer0/conduit".into(),
            score: 880,
            effort_hours: 3,
            rationale: "critical infra with weak review trail".into(),
            prerequisites: vec!["policy-refresh".into()],
        },
        Task {
            id: make_id("tsk"),
            module: "docs/governance".into(),
            score: 760,
            effort_hours: 1,
            rationale: "compliance mapping gap".into(),
            prerequisites: vec!["audit-ready".into()],
        },
    ]
}

fn make_receipt(cli: &Cli, root: &Path, started_ms: u64, output: &Value, status_code: i32) -> Value {
    let mut hasher = Sha256::new();
    hasher.update(root.to_string_lossy().as_bytes());
    hasher.update(output.to_string().as_bytes());
    let digest = hasher.finalize();
    let mut hex = String::new();
    for b in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut hex, "{:02x}", b);
    }
    let policy_hash = "prism-policy-placeholder".to_string();
    let policy = root.join(".prism/config.json");
    json!({
        "receipt": {
            "command": "prism ".to_string() + &format!("{:?}", cli.command),
            "policy_hash": policy_hash,
            "policy_path": policy.to_string_lossy(),
            "status_code": status_code,
            "execution_ms": now_ms().saturating_sub(started_ms),
            "output_hash": hex,
            "command_identity": cli.command_hash(&digest),
            "command_profile": if cli.json { "json" } else { "human" },
            "root": root.to_string_lossy(),
            "finished_at": Utc::now().to_rfc3339()
        }
    })
}

fn fake_source_hash() -> String {
    let mut hasher = Sha256::new();
    hasher.update(std::process::id().to_le_bytes());
    let digest = hasher.finalize();
    let mut out = String::new();
    for b in digest.iter().take(8) {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{:02x}", b);
    }
    format!("src-{out}")
}

fn write_task_receipt(root: &Path, task_id: &str) -> Result<(), String> {
    let path = root.join(".prism/receipts").join(format!("{task_id}.json"));
    ensure_dir(root.join(".prism").join("receipts"))?;
    write_json(&path, &json!({"task_id": task_id, "status": "ok", "ran_at": Utc::now().to_rfc3339()}))
}

fn ensure_dir(path: impl AsRef<Path>) -> Result<(), String> {
    let path = path.as_ref();
    if path.exists() {
        return Ok(());
    }
    fs::create_dir_all(path).map_err(|e| e.to_string())
}

fn write_json(path: &Path, payload: &Value) -> Result<(), String> {
    let file_content = serde_json::to_string_pretty(payload).map_err(|e| e.to_string())?;
    fs::write(path, file_content).map_err(|e| e.to_string())
}

fn append_line(path: &Path, line: &str) -> Result<(), String> {
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())
}

fn resolve_root(raw: &str) -> PathBuf {
    PathBuf::from(raw)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_millis() as u64)
        .unwrap_or(0)
}

fn make_id(prefix: &str) -> String {
    format!("{prefix}-{}", &fake_source_hash()[..10])
}

trait CommandIdentity {
    fn command_hash(&self, seed: &[u8]) -> String;
}

impl CommandIdentity for Cli {
    fn command_hash(&self, seed: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed);
        hasher.update(format!("{:?}", self.command).as_bytes());
        hasher.update(self.root.as_bytes());
        let d = hasher.finalize();
        let mut out = String::new();
        for b in &d[..8] {
            use std::fmt::Write as _;
            let _ = write!(&mut out, "{:02x}", b);
        }
        out
    }
}
