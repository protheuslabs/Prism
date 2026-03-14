# Prism Tech Stack and Implementation Plan

## Canonical Stack (Target)

Prism is implemented as an offline, deterministic, auditable Rust CLI with explicit evidence outputs.

- Runtime language: Rust 2021 (single primary implementation language)
- CLI framework: `clap` with subcommand decomposition
- Serialization: `serde` + canonical JSON (`serde_json`) and YAML policy support via `serde_yaml`
- Deterministic hashing and signatures: `sha2` now, moving to `blake3` for chained receipts/evidence digests
- Concurrency and async workflow: `tokio` for bounded parallelism and deterministic scheduling boundaries
- Local store: SQLite via `sqlx` (indexed run/task/state store) plus append-only `jsonl` ledgers in `.prism/`
- Structured audit: `tracing` for execution instrumentation, `tracing-subscriber` + JSON formatting
- Metrics and health: `prometheus`-style counters exported as JSON (or local endpoint in later versions)
- Security primitives: `ed25519-dalek` for signatures, `aes-gcm` for field encryption, abstraction for KMS/HSM providers
- External integrations: `reqwest` + typed adapters for GitHub/GitLab/Jira/ServiceNow/Slack/PagerDuty
- Optional API mode: `axum` HTTP server exposing the same query schema as CLI outputs

## Why this stack for Prism

- Determinism-first: Rust + stable hashing + canonical JSON produce byte-stable outputs for replay.
- Operational reliability: SQLite + file-ledger model gives both queryability and tamper-evidence.
- Single-maintainer practicality: strong static typing and explicit errors reduce hidden behavior drift.
- Audit-readiness: signed evidence, immutable receipts, and reproducible chain hashes support enterprise evidence collection.

## How it will be implemented

### 1) Foundation layer (immediately)

- Split CLI command handlers from policy/evidence engine modules.
- Add core domain model types: snapshot, signal, score, task, gate, run, receipt, and incident.
- Introduce deterministic command context object with:
  - input digest
  - profile hash
  - execution mode
  - source snapshot anchor
- Normalize all outputs to canonical schemas before writing to `.prism/receipts/` and `.prism/ledger/`.

This directly implements:
- BLK-001, BLK-002, BLK-007
- PRISM-SRS-001, PRISM-SRS-002, PRISM-SRS-008

### 2) Policy-first enforcement path

- Add strict/warn/off mode resolution with deterministic precedence rules.
- Bind all mutating commands behind the policy decision function:
  - `pass` -> execute
  - `warn` -> execute with advisory warning
  - `block` -> stop and emit blocker receipts
- Add emergency override token flow and immutable override metadata.

This directly implements:
- PRISM-SRS-033 and BLK-021
- PRISM-SRS-034 for signed enforcement artifacts

### 3) Evidence and audit as a first-class plane

- Add tamper-linked ledger format:
  - `prev_hash`, `entry_hash`, `entry_index`, `action_signature`, `run_hash`
- Add `prism audit verify` and deterministic verification failure reasons.
- Add command lineage and replay metadata on each gate/plan/incident/event command.
- Add redaction/encryption pass before persistence and export.

This directly implements:
- PRISM-SRS-034, PRISM-SRS-038
- BLK-023, BLK-027

### 4) Scale and resilience plane

- Add checkpointing and resumable snapshot states for 1M+/10x growth runs.
- Add state backup/restore with lineage checks and conflict reconciliation.
- Add lock/lease model for conflicting target sets before `prism do`.

This directly implements:
- PRISM-SRS-016, PRISM-SRS-028, PRISM-SRS-037, PRISM-SRS-040
- BLK-008, BLK-018, BLK-026, BLK-029

### 5) Enterprise connector and admission plane

- Add connector registry and schema-locked adapters.
- Emit machine events for:
  - gate result changes
  - risk deltas
  - remediation task creation/completion
- Add policy package fetch/cache with provenance and drift reports.

This directly implements:
- PRISM-SRS-035, PRISM-SRS-036, PRISM-SRS-031, PRISM-SRS-032, PRISM-SRS-039
- BLK-011, BLK-012, BLK-014, BLK-025, BLK-024, BLK-028

## Milestone order (recommended)

- Milestone A: P0 foundations
  - BLK-001, BLK-002, BLK-003, BLK-004, BLK-007, BLK-011, BLK-021, BLK-022
- Milestone B: Enforcement hardening and trust
  - BLK-023, BLK-024, BLK-027, BLK-028, BLK-029
- Milestone C: Enterprise integration
  - BLK-025, BLK-012, BLK-013, BLK-014
- Milestone D: Resilience and continuity
  - BLK-026, BLK-018, BLK-019, BLK-020

## Deterministic implementation rule

- Any command that mutates state or emits gate decisions must:
  1) compute deterministic input digest
  2) persist a receipt
  3) update the audit chain
  4) fail closed if required policy metadata/signature fields are missing in strict mode

## Risks and constraints

- New file additions in this repo must preserve existing style and avoid migration drift from docs-only signals.
- Determinism constraints must be tested via replay and snapshot-idempotence before any release scope command can be trusted.
- Enterprise-grade connectors are optional in observe mode and hard-fail in strict mode.
