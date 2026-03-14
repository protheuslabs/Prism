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

## Reference Architecture

The authoritative implementation stack and rollout sequencing are defined in:
- [`PRISM_TECH_STACK.md`](/Users/jay/.openclaw/workspace/apps/prism/docs/PRISM_TECH_STACK.md)
- [`PRISM_ARCHITECTURE.md`](/Users/jay/.openclaw/workspace/apps/prism/docs/PRISM_ARCHITECTURE.md)

This doc is normative for design decisions that are not fully covered in individual SRS requirements (especially implementation ordering, module decomposition, and deterministic execution guarantees).

## Architecture fit vs. mission

Prism is architected from day one as an execution-control tool rather than a source annotation tool. That makes it a better long-term fit for one-operator scale than LensMap's knowledge layer.

Ideal state for Prism requires:
- deterministic domain engine isolation from transport surfaces (CLI/API/connectors),
- policy and enforcement as a single pre-action gate,
- receipt-led execution with audit-chained lineage, and
- resumable checkpoints for large-scale reruns.

These are all explicit in `PRISM_ARCHITECTURE.md`; implementation progress against that document should be the primary architectural compliance check.

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

### PRISM-SRS-021 Adaptive Deterministic Policy Profiles
- Prism shall support environment and role-specific policy profiles that control strictness, risk cutoffs, and auto-accept thresholds.
- Profiles shall merge deterministically from:
  - global defaults,
  - repository policy,
  - and active mission profile (`explore`, `stability`, `incident`, `audit_preflight`).
- Profile resolution and precedence must be replayable and included in every gate and planning receipt.
- Missing profile fields must fail closed in strict mode and fail open only with explicit override and audit trace.

### PRISM-SRS-022 Integrity and Compliance Evidence Exports
- Prism shall emit audit-grade evidence packages per release cycle and per gate run.
- Each package shall include:
  - score inputs hash,
  - task decision graph,
  - gate decisions,
  - evidence completeness manifest,
  - and operator approvals/rejections.
- Evidence packages shall be exportable in JSON and signed metadata format for external compliance systems.
- Evidence artifacts shall be immutable by default and linked by deterministic `prism`-run identity.

### PRISM-SRS-023 Deterministic What-If Simulation
- Prism shall provide `prism simulate` to evaluate hypothetical operator actions before execution.
- Simulations shall model:
  - task deferrals,
  - profile changes,
  - risk-threshold changes,
  - and blast-radius escalation.
- Simulation outputs must include projected risk reduction, owner load impact, and rollback cost estimate.
- Prism shall guarantee no state mutations in simulation mode and emit a reproducible simulation receipt.

### PRISM-SRS-024 Operational API and Query Surface
- Prism shall expose a machine-readable query surface for external orchestrators (CLI JSON, optional local HTTP/stdio API).
- Required query operations include:
  - retrieve top risk units,
  - retrieve active blockers,
  - retrieve handoff packets,
  - retrieve gate status by scope,
  - and retrieve action receipts by id.
- Query responses must be deterministic and versioned for third-party tool integration.
- API and local command modes must emit the same schema and checksum anchors for parity.

### PRISM-SRS-025 Cross-Domain Dependency and Transfer Maps
- Prism shall detect critical cross-module dependencies and compute transfer risk for each module cluster.
- Dependency maps shall support:
  - critical-path amplification,
  - boundary ownership concentration,
  - migration dependency blocking,
  - and domain-to-domain coupling risk.
- Transfer maps shall be used in prioritization and shall be explainable in `priority_explain`.

### PRISM-SRS-026 Incident-Class Triage Playbooks
- Prism shall maintain deterministic operator playbooks for common high-risk classes:
  - critical policy violation,
  - silent score drift,
  - unresolved high-value stale debt,
  - and release-blocking evidence missing.
- Each playbook shall define:
  - required evidence,
  - approved first-respond actions,
  - escalation triggers,
  - and handoff template.
- Playbook execution shall produce deterministic state markers and closure checks.

### PRISM-SRS-027 Runtime Security and Access Controls
- Prism shall implement command-layer authorization profiles:
  - read-only,
  - planning-only,
  - execute,
  - and emergency override.
- Unauthorized state-changing commands shall be rejected with immutable audit logs.
- Critical mutating flows (`do`, `apply`, `handoff resolve`) shall require proof of operator authorization and immutable approval metadata.
- Access decisions and audit entries shall be included in gate receipts.

### PRISM-SRS-028 Data Lineage and Replayability
- Prism shall provide deterministic replay of any observed run from snapshot hash + config hash + command identity.
- Replay output must be byte-for-byte stable for deterministic modes.
- Run metadata shall include command lineage chain (parent run, derived run, rerun run).
- Operators shall be able to diff two deterministic runs and receive explainable deltas.

### PRISM-SRS-029 Health Governance and Drift Alerts
- Prism shall detect governance drift across cycles:
  - rising overdue counts,
  - declining owner response rates,
  - repeated deferment without impact reduction,
  - and policy bypass attempts.
- Prism shall emit health alerts with severity, confidence, and actionable remediation IDs.
- Alert generation shall support throttling and alert suppression windows without hiding release-blocking conditions.

### PRISM-SRS-030 Self-Calibrating Model and Feedback Loop
- Prism shall update scoring coefficients through operator feedback loops from resolved incidents and closed tasks.
- Calibration updates shall be:
  - bounded,
  - audit-logged,
  - and explainable.
- Prism shall support two modes:
  - static coefficients (default),
  - controlled adaptation mode with manual enable and rollback.
- All calibration changes must be reflected in policy receipts and versioned config deltas.

### PRISM-SRS-031 Change Admission and Gate Integration
- Prism shall expose admission outputs for CI/PR workflows that consume enforcement results in machine format.
- Admission artifacts shall include:
  - changed file set hash,
  - enforce results by policy ID,
  - failed blockers with severity,
  - required remediation IDs,
  - and deterministic overall pass/fail.
- Admission mode shall fail closed on missing policy definition/version, signature, or mandatory evidence inputs.
- `prism gate` and `prism enforce` results shall share a canonical schema and stable exit semantics.

### PRISM-SRS-032 Enforcement Drift and Policy Evolution
- Prism shall detect policy drift between runs and require explicit re-baseline for active enforcement.
- Drift sources include:
  - profile/version edits,
  - scope boundary changes,
  - new high-risk rule additions,
  - and removed exception entries.
- On drift, Prism shall emit:
  - impact estimate,
  - stale/unsafe queue candidates,
  - and minimum required re-scan scope.
- Unreviewed policy drift in strict mode shall prevent release admission until acknowledged.

### PRISM-SRS-033 New Change Policy Enforcement
- Prism shall provide a policy-enforcement command mode (`prism enforce` or equivalent) for every proposed mutation input.
- New or modified assets shall be checked against active policy before `prism do` execution and before gate/release submission.
- Enforcement scope shall include:
  - path and module policy mapping,
  - required ownership/review evidence,
  - policy severity thresholds,
  - risk budget and blast-radius constraints,
  - and mandatory compliance tags.
- Enforcement result semantics:
  - `pass` when all checks succeed,
  - `warn` for advisory/non-blocking violations,
  - `block` for strict violations that prevent execution/release.
- Emergency override must require explicit signed token and produce an immutable override justification record.
- Enforcer results shall be receipt-backed and deterministic, including policy checksum and effective profile.

### PRISM-SRS-034 Audit Integrity and Non-Repudiation
- Prism shall maintain a tamper-evident audit ledger for all plan, gate, and execution actions.
- Each command run must emit:
  - chained hash link to the prior action,
  - previous/next ledger anchors,
  - optional signer identity,
  - and canonical serialized action payload.
- The ledger shall be cryptographically verifiable via a public `prism audit verify` workflow.
- Critical evidence and gate outcomes can be exported with an optional detached signature envelope for external compliance ingestion.

### PRISM-SRS-035 Fleet-Scale Policy Federation
- Prism shall support organization-level policy federation with:
  - signed policy package references,
  - deterministic policy cache refresh,
  - policy source provenance,
  - and explicit local override approval semantics.
- Policy resolution must be deterministic across repos running the same policy package and environment profile.
- Drift between local policy and fleet baseline shall be detected and flagged with required re-baseline tasks.

### PRISM-SRS-036 Enterprise Connector and Orchestration API
- Prism shall provide deterministic connectors for enterprise systems (GitHub, GitLab, Jira, ServiceNow, PagerDuty, Slack) through explicit schema-locked adapters.
- Connector actions must be:
  - policy-bound,
  - replayable,
  - auditable via Prism receipts.
- Prism shall support a machine-readable event bus to publish:
  - risk deltas,
  - gate outcomes,
  - and remediation events.
- Missing or unavailable connectors must fail closed in strict mode and degrade predictably in observe-only mode.

### PRISM-SRS-037 Resilience, DR, and Restore
- Prism shall support deterministic backup and restore of Prism state (`.prism/state`, receipts, snapshots, and policy cache).
- Recovery mode shall allow:
  - selective replay by run range,
  - conflict-detection for partially applied runs,
  - and state continuity verification against chain hashes.
- Restore outcomes must include a recovery report with deterministic missing/compensated commands.

### PRISM-SRS-038 Encryption, Secrets, and Sensitive Metadata Control
- Prism shall classify, redact, and optionally encrypt sensitive metadata fields before persistence and export.
- Required capabilities:
  - per-field redaction policy,
  - deterministic hash for encrypted payload verification,
  - and key-source abstraction compatible with KMS/HSM.
- Operator session secrets and signed override tokens must not be written to plaintext history or logs.

### PRISM-SRS-039 Service-Level Governance for the Tool
- Prism shall expose internal operational SLOs:
  - queue generation latency percentiles,
  - gate decision latency,
  - evidence freshness age,
  - and command success/failure ratio.
- `prism` shall emit self-health reports and alert when thresholds are violated.
- Health failures at warning threshold shall create advisory tasks; failures at critical threshold shall block release scope gates.

### PRISM-SRS-040 Multi-Operator Concurrency Control
- Prism shall prevent accidental conflicting actions on overlapping target sets in team/parallel usage.
- A deterministic lock model shall include:
  - lock scope labels (module/tree/task),
  - lease duration and renewal semantics,
  - conflict detection before `prism do` mutation,
  - and clean handoff release events.
- When conflict is detected, Prism shall propose a deterministic merge/serialize path and preserve operator continuity notes.

### PRISM-SRS-041 Policy Plane Federation Adapter
- Prism shall support a dedicated policy distribution and enforcement adapter for external policy systems.
- The adapter shall:
  - fetch signed policy bundles (organization, domain, repository profile),
  - resolve deterministic precedence and override policy,
  - and emit policy bundle fingerprints in all enforcement artifacts.
- Prism shall allow external repos to consume a shared policy cache while preserving explicit repo-local exceptions.
- All policy bundle verification failures in strict mode shall block release-scope execution.

### PRISM-SRS-042 Trust and Evidence Vault Integration
- Prism shall exchange trust and compliance evidence with a central evidence vault service for:
  - policy signatures,
  - release manifests,
  - SBOM and dependency attestations,
  - and connector event receipts.
- Prism shall export evidence bundle references in a vault-compatible envelope schema with deterministic versioning.
- Missing or mismatched trust references in strict mode shall block `prism gate --scope release`.

### PRISM-SRS-043 Incident Command and Recovery Plane
- Prism shall integrate with a deterministic incident control module that owns escalation workflow, severity routing, and post-incident audit trails.
- Incident state transitions emitted by Prism shall be mirrored into the incident plane with deterministic IDs and closure proofs.
- Incident plane updates shall include Prism task IDs, gate blockers, and affected policy set fingerprints.
- Repeat incidents with same signature inputs shall replay identically unless severity policy changes.

### PRISM-SRS-044 Observability and SLO Plane
- Prism shall provide first-class telemetry export to a deterministic monitoring plane for:
  - scoring latency,
  - gate decision outcomes,
  - evidence lag,
  - and task execution failure rates.
- Monitoring artifacts shall feed into Prism health tasks with warning/critical thresholds and remediation actions.
- SLO breaches in strict release mode shall emit deterministic blockers and evidence references.

### PRISM-SRS-045 Secrets and Credential Boundary Plane
- Prism shall support policy-bound access through an operator identity and secret-management abstraction.
- Required capabilities:
  - role/actor claims in command receipts,
  - short-lived token workflows for sensitive operations,
  - and encrypted-at-rest storage for session/override metadata.
- Secret-provider failures or policy mismatch in strict mode shall fail-closed for mutating commands.

### PRISM-SRS-046 Cross-Repo Knowledge Graph and Contract Registry
- Prism shall support a deterministic repository registry for domain contracts, ownership maps, and migration boundaries used for policy-weighted scoring.
- Registry entries shall include:
  - module ownership graph,
  - domain boundary definitions,
  - contract references,
  - and historical migration status.
- Prism scoring and planning shall consume registry links to prevent domain boundary blind spots.

### PRISM-SRS-047 AI Change Governance and Safety Guard
- Prism shall include a validation path for AI-assisted code-change proposals (patches, refactors, annotations) before execution.
- Validation shall cover:
  - policy conformance,
  - risky surface changes,
  - dependency/conflict checks,
  - and owner/load impact.
- High-risk AI-suggested actions shall be blocked by default in strict mode unless explicitly approved with signed override metadata.

### PRISM-SRS-048 Deliverable Integrity and Release Orchestration Plane
- Prism shall consume deterministic release/package descriptors from a release tooling plane (artifact names, signatures, install hashes).
- Prism shall produce release readiness artifacts that can be consumed by external orchestrators and CI gates.
- Inconsistent descriptor input or missing release envelope continuity shall fail strict release gates.

### PRISM-SRS-049 Communication and Orchestrator Hub
- Prism shall publish deterministic connector events for ticketing, collaboration, and PR systems from gate, plan, and incident workflows.
- Events shall include:
  - blocker IDs,
  - policy/score provenance,
  - and replay-idempotency markers.
- Connector failures in strict mode during release workflow shall surface as blocking issues and include remediation actions.

### PRISM-SRS-050 Shared Workspace and Multi-Tool API Contract
- Prism shall expose a stable API contract for partner tools (Dashboards, AI copilots, PM systems, risk engines).
- Contract requirements:
  - versioned schema per endpoint,
  - deterministic paging and ordering,
  - signed-like lineage metadata.
- Any schema/version violation from partner tools must be reported and block mutable actions when the partner is configured as hard-coupled.

## Non-Goals (v0.1)

- Not a replacement for IDE refactoring tools.
- Not a source-of-truth code formatter/linters.
- Not a policy authoring engine (uses external systems for policy governance).
- Not a replacement for LensMap source-linked knowledge capture; Prism only consumes governance signals.
