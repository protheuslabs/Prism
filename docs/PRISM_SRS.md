# PRISM SRS (Version 0.1)

## Purpose

Prism provides a practical complexity-management control plane for a single operator running very large codebases.

The design philosophy is explicit: with deterministic signals, bounded plans, and receipts, one person should be able to govern a repo at 1M+ to 30M LOC without waiting for team-sized coordination overhead.
Prism is not code-first annotation tooling; it is a leverage engine for deciding what to do next with certainty.

## Scope (Initial Tranche)

1. Deterministic risk scoring for changed / critical modules.
2. Execution queue synthesis with budget-aware plans.
3. Gate evaluation for release and merge safety.
4. Receipted incident tracking.
5. LensMap interoperability contract (read-only in v0.1).

The long-horizon objective is one-operator scalability: one operator can keep a large estate manageable by applying small, high-confidence, auditable actions repeatedly.

See also: [`PRISM_BACKLOG.md`](/Users/jay/.openclaw/workspace/apps/prism/docs/PRISM_BACKLOG.md) for executable implementation requirements and command-level acceptance criteria.

## Requirements

### PRISM-SRS-001 Repository Ingest
- Prism shall index repository files, ownership hints, and git surface signals.
- Prism shall produce an append-only, timestamped snapshot with:
  - file set hash
  - changed-file ratio
  - metadata signals discovered
  - computed signal envelope

### PRISM-SRS-002 Deterministic Scoring
- Prism shall compute a deterministic score (0-1000) per module.
- Score inputs:
  - policy severity from associated notes (if available),
  - churn,
  - owner sparsity,
  - stale annotation age,
  - cross-domain surface flags,
  - review history.
- A full deterministic score run with unchanged inputs must generate identical outputs.

### PRISM-SRS-003 Task Synthesis
- Prism shall generate a queue where each task has:
  - task id
  - risk rationale
  - expected effort bounds
  - prerequisites/dependencies
  - rollback note
- Tasks should be exportable to CI and operator dashboards.

### PRISM-SRS-004 Plan and Capacity Management
- Prism shall produce a plan bounded by a work budget (hours).
- It shall prioritize by score, critical-path effect, and coupling constraints.
- Plans are emitible as:
  - Markdown report,
  - JSON/NDJSON tasks.

### PRISM-SRS-005 Gate and Verification
- Prism shall expose `prism gate` checks for:
  - policy coherence
  - top-risk open tasks
  - evidence completeness
  - stale critical items.
- Gate outputs include machine-readable pass/fail and a signed-ish deterministic receipt payload.

### PRISM-SRS-006 Incident Loop
- Prism shall track incidents with:
  - open date
  - impacted modules
  - mitigation steps
  - closure evidence
- It shall support reopen, close, and rollback notes.

### PRISM-SRS-007 LensMap Interop
- Prism shall optionally read `.lensmap` metadata and merge:
  - owner
  - template class
  - review status
  - compliance scope tags
- Missing LensMap inputs shall degrade gracefully to neutral evidence.

### PRISM-SRS-008 Evidence Envelope Compatibility
- Each major command that mutates state shall emit:
  - command identity,
  - deterministic hash inputs,
  - status code,
  - artifact list,
  - execution time.
- Receipts are stored in `.prism/receipts/`.

### PRISM-SRS-009 One-Operator Scalability Contract
- Prism shall provide an `observe → score → plan → act → verify` cycle optimized for a single operator.
- Prism shall support a bounded execution mode where each cycle outputs:
  - top-`N` highest-impact tasks,
  - estimated operator time windows,
  - expected risk reduction estimate,
  - and an explicit stop condition when confidence drops below configured thresholds.
- Prism shall expose a "workfront budget" that lets one operator cap scope per cycle by risk and blast-radius constraints.
- Prism shall maintain an "owner load map" so one operator can avoid overloading adjacent subdomains and can hand off only when a stable task packet is prepared.
- Prism shall make escalation explicit by generating a handoff packet when automation confidence is insufficient.

### PRISM-SRS-010 LensMap Synergy and Signal Integrity
- Prism shall accept LensMap as its primary deterministic governance signal source and shall not implement annotation format ownership.
- Any task synthesis input from LensMap shall include provenance fields so Prism can explain the originating policy, note, owner, and risk assumptions.
- Prism shall provide a deterministic mapping from LensMap signals to task rationale and priority so operator decisions remain auditable and replayable.
- Prism shall tolerate missing/partial LensMap data with conservative default priorities and explicit "insufficient signal" flags.

### PRISM-SRS-011 Single-Operator Throughput Contract
- Prism shall define a deterministic operator throughput target profile with three presets:
  - `single_shift` (8 hours)
  - `focused` (2 hours)
  - `incident` (30 minutes)
- Each preset shall enforce:
  - maximum number of tasks produced,
  - maximum total expected effort,
  - and automatic reduction of low-confidence items to deferred status.
- Prism shall emit a session summary with planned tasks, confidence, estimated completion, and explicit deferred backlog on every run.

### PRISM-SRS-012 Explainable Priority Ordering
- Prism shall explain every task ranking as a weighted, deterministic formula breakdown.
- The formula shall include at least:
  - policy criticality,
  - recent change velocity,
  - failure/evidence risk,
  - owner risk,
  - cross-domain blast radius.
- Prism shall expose this explanation as a compact `priority_explain` object in task outputs.
- Operators shall be able to recompute the same ordering by replaying the stored score inputs.

### PRISM-SRS-013 Bounded Remediation Execution
- `prism do` shall default to dry-run and produce a deterministic action plan diff preview.
- In execution mode, `prism do` shall require one of:
  - explicit `--apply` confirmation,
  - or a signed operator token.
- Every executed change path shall be scoped to a declared blast-radius class:
  - `micro` (single file),
  - `local` (single module),
  - `wave` (cross-module bounded set),
  - `fleet` (explicitly gated by operator escalation).
- Prism shall store pre-state snapshot metadata before every mutating action to support precise rollback/revert.

### PRISM-SRS-014 Failure and Recovery Resilience
- Prism shall record all command-level failures with:
  - error code,
  - failed input signature,
  - affected evidence set,
  - and recommended recovery step.
- Recovery mode shall support:
  - replay from last successful checkpoint,
  - automatic checkpoint integrity checks,
  - and deterministic task reconstitution.
- Repeatable failures on the same snapshot and threshold set shall produce identical recovery guidance.

### PRISM-SRS-015 Deterministic Incident Governance
- Prism incidents shall include:
  - severity,
  - affected critical paths,
  - active mitigations,
  - residual risk,
  - operator burden impact estimate,
  - and closure evidence hash.
- Incident state transitions (open/in_progress/contained/closed/escalated) shall be auditable and receipt-backed.
- Escalated incidents with unresolved risk above threshold shall block `prism gate` by default.

### PRISM-SRS-016 Scaling and Performance Envelope
- Prism shall complete initial `refresh`+`score`+`plan` for a 1M+ LOC repo within a predictable envelope that is configurable by profile.
- For 10x growth in file count, Prism scoring shall degrade gracefully with checkpointed resumable execution rather than timeout failure.
- Prism shall process large monorepos with bounded parallelism and deterministic merge ordering for reproducible outputs.

### PRISM-SRS-017 Operator Handoff and Continuity
- Prism shall generate a standardized `handoff_packet.json` when confidence or ownership load exceeds safe limits.
- The handoff packet shall include:
  - current session state,
  - pending tasks,
  - unresolved assumptions,
  - evidence receipts,
  - and operator continuation checkpoints.
- Handoff packets shall support import by a second Prism session without re-running full scoring.

### PRISM-SRS-018 Security, Integrity, and Trust
- All command outputs that affect planning, execution, or gates shall include an integrity envelope with:
  - deterministic hash,
  - command identity,
  - input reference digest,
  - signature-ready metadata.
- Receipts shall support immutable storage policy and retention tagging by criticality.
- Prism shall fail closed if input integrity metadata is missing in strict mode.

### PRISM-SRS-019 Evidence-Driven Release Gate
- `prism gate --scope release` shall require:
  - top-risk backlog within policy threshold,
  - no unresolved escalated incidents,
  - and evidence receipts for critical items.
- Gate output shall emit a single pass/fail with deterministic blockers grouped by severity.
- Gate failures shall include exact remediation task IDs and reproducible command hints.

### PRISM-SRS-020 Mission Outcome KPIs
- Prism shall publish a quarterly (or equivalent) posture report with:
  - operator tasks completed per cycle,
  - average confidence calibration,
  - average time-to-action,
  - unresolved-risk half-life,
  - and handoff frequency.
- These KPIs shall be consumable by CI, leadership dashboards, and self-review.
- KPIs shall distinguish machine-certain actions from human-dependent actions.

## Non-Goals (v0.1)

- Not a replacement for IDE refactoring tools.
- Not a source-of-truth code formatter/linters.
- Not a policy authoring engine (uses external systems for policy governance).
- Not a replacement for LensMap source-linked knowledge capture; Prism only consumes governance signals.
