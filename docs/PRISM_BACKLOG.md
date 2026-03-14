# Prism Backlog (Executable Requirements)

This file translates `docs/PRISM_SRS.md` into implementation backlog.  
Items are ordered to support one-operator throughput while preserving determinism and recoverability.

## Backlog Execution Convention

- `P0`: must exist before any public release
- `P1`: required for one-operator 1M+ LOC target
- `P2`: operator efficiency improvement
- `P3`: hardening and observability enhancements
- `Status`: queued / in_progress / blocked / done (docs-only status is not done)
- `Scope`: CLI surface, data model, receipt/gate path, or reporting

### BLK-001 — Deterministic repository state and ownership ingest
- **SRS Linkage**: PRISM-SRS-001, PRISM-SRS-007
- **Priority**: P0
- **Scope**: `prism refresh` and `.prism/snapshots/*`
- **Definition**
  - Build a deterministic ingest pipeline for files, ownership hints, git metadata, and LensMap signals.
  - Persist an append-only snapshot containing file-set hash, signal envelope, and collection timestamp.
- **Acceptance**
  - Identical repository state + inputs produce identical snapshot hashes.
  - Snapshot format is machine-parsable and supports incremental updates.
  - Snapshot artifacts are written atomically and include a lineage reference to previous snapshot.
- **Tests**
  - Snapshot determinism test on repeated run
  - Snapshot integrity test for missing LensMap path (graceful fallback)

### BLK-002 — Deterministic scoring engine and score traceability
- **SRS Linkage**: PRISM-SRS-002, PRISM-SRS-012
- **Priority**: P0
- **Scope**: `prism score` and `score_signal` trace payload
- **Definition**
  - Implement module score model (0-1000) with deterministic formula and bounded feature weights.
  - Emit `priority_explain` with full formula inputs and output ordering rationale.
- **Acceptance**
  - Sorting is reproducible for the same snapshot/signature set.
  - Score output includes policy, churn, owner sparsity, stale annotation age, cross-domain flags, and review history.
  - Explanations are compact, machine-readable JSON and replayable.
- **Tests**
  - Deterministic sort replay test
  - Explainability schema regression test

### BLK-003 — Budget-aware task synthesis and cycle planning
- **SRS Linkage**: PRISM-SRS-003, PRISM-SRS-004, PRISM-SRS-011
- **Priority**: P0
- **Scope**: `prism plan`, output formats: Markdown + NDJSON
- **Definition**
  - Generate actionable plans bounded by work budget and operator throughput profile.
  - Include prerequisite/dependency graph, effort windows, impact estimates, and confidence.
- **Acceptance**
  - Preset profiles (`single_shift`, `focused`, `incident`) enforce task caps and effort caps.
  - Plan output includes top-N tasks, deferred backlog, and stop condition.
  - Export formats are stable and parseable by CI dashboards.
- **Tests**
  - Preset cap enforcement test
  - Plan export parse test

### BLK-004 — Command-level gate and release policy engine
- **SRS Linkage**: PRISM-SRS-005, PRISM-SRS-019
- **Priority**: P0
- **Scope**: `prism gate`, release scope semantics
- **Definition**
  - Implement gate checks for policy coherence, critical backlog, stale critical items, and evidence completeness.
  - Add release scope gate that blocks on unresolved escalated incidents and risk thresholds.
- **Acceptance**
  - Gate reports deterministic structured blockers with severity + remediation IDs.
  - Machine-readable pass/fail plus receipt linkage per run.
- **Tests**
  - Gate fail/pass deterministic regression
  - Release scope strict-mode test

### BLK-005 — Receipt-backed incident lifecycle with handoff trigger
- **SRS Linkage**: PRISM-SRS-006, PRISM-SRS-015, PRISM-SRS-017
- **Priority**: P0
- **Scope**: `prism incident`, state transitions, `prism handoff`
- **Definition**
  - Implement incident state model with reopen/close/rollback notes and affected path metadata.
  - Generate deterministic handoff packets when operator load/confidence thresholds are exceeded.
- **Acceptance**
  - Escalated incidents default-gate release and/or unsafe plan execution.
- **Tests**
  - State-transition audit test
  - Handoff packet schema + import roundtrip test

### BLK-006 — Bounded remediation and execution safety rails
- **SRS Linkage**: PRISM-SRS-013, PRISM-SRS-014
- **Priority**: P1
- **Scope**: `prism do`, execution profile, rollback checkpoints
- **Definition**
  - Implement default dry-run, explicit apply token/flag, and blast-radius classes.
  - Require mutation checkpoints and deterministic rollback metadata before action.
- **Acceptance**
  - No mutation without explicit operator confirmation unless in explicit trusted automation mode.
  - Snapshot delta and rollback metadata emitted for every `--apply`.
- **Tests**
  - Dry-run safety test
  - Rollback metadata retention test

### BLK-007 — Integrity envelope and evidence storage
- **SRS Linkage**: PRISM-SRS-008, PRISM-SRS-018
- **Priority**: P1
- **Scope**: `.prism/receipts/`, `prism inspect --receipt`
- **Definition**
  - Persist command identity, deterministic hash inputs, artifact list, and run metadata for all mutating commands.
  - Include immutable retention tags based on criticality.
- **Acceptance**
  - Receipt records are append-only and idempotent for repeated retries.
  - Strict mode fails closed when required integrity metadata is absent.
- **Tests**
  - Receipt schema validation
  - Strict-mode failure test for missing integrity metadata

### BLK-008 — Scale and resumability for very large codebases
- **SRS Linkage**: PRISM-SRS-016
- **Priority**: P1
- **Scope**: `prism refresh`, worker scheduling, checkpoint recovery
- **Definition**
  - Deliver bounded-parallel scoring with deterministic merge ordering.
  - Implement resumable checkpoints for 10x repository growth and crash recovery.
- **Acceptance**
  - No silent nondeterministic ordering in merged shards.
  - Checkpoint resume completes without losing score lineage.
- **Tests**
  - Large-set simulated growth test
  - Resume-from-checkpoint determinism test

### BLK-009 — LensMap interoperability contract and degraded operation mode
- **SRS Linkage**: PRISM-SRS-007, PRISM-SRS-010
- **Priority**: P2
- **Scope**: LensMap adapter + `prism import-lensmap`
- **Definition**
  - Consume LensMap metadata for owner, review, template class, compliance scope, and evidence flags.
  - Emit clear insufficient-signal states when source metadata is partial/missing.
- **Acceptance**
  - Deterministic mapping of LensMap notes to task rationale and priority deltas.
  - No hard dependency: missing LensMap inputs still produce conservative default scoring.
- **Tests**
  - Full LensMap import test
  - Missing-signal conservative-priority test

### BLK-010 — Mission KPIs and operator effectiveness loop
- **SRS Linkage**: PRISM-SRS-009, PRISM-SRS-020
- **Priority**: P2
- **Scope**: `prism report`, `prism kpi`
- **Definition**
  - Publish recurring posture KPIs: completion rate, confidence calibration, time-to-action, unresolved-risk half-life.
  - Distinguish machine-assured vs human-dependent actions in reporting.
- **Acceptance**
  - Quarterly/equivalent report can be generated from CI artifacts without manual recomputation.
  - KPIs are stable across reruns for unchanged snapshots.
- **Tests**
  - KPI determinism test
  - Report generation regression test

## Execution Order (Suggested)

1. BLK-001
2. BLK-002
3. BLK-003
4. BLK-004
5. BLK-005
6. BLK-007
7. BLK-006
8. BLK-008
9. BLK-009
10. BLK-010
