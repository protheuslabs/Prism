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

### BLK-011 — Policy profile engine and deterministic precedence
- **SRS Linkage**: PRISM-SRS-021
- **Priority**: P0
- **Scope**: policy resolution and `prism gate/plan` config layer
- **Definition**
  - Implement layered policy profiles for role/environment with deterministic precedence and inheritance.
  - Persist resolved policy fingerprint in gate and plan receipts.
- **Acceptance**
  - Changing profile source ordering produces predictable re-resolved output.
  - Strict mode rejects undefined or conflicting profile precedence.
  - Active profile is always exposed in command artifacts.
- **Tests**
  - Precedence determinism test
  - Strict-mode conflict rejection test

### BLK-012 — Evidence package export and compliance artifacting
- **SRS Linkage**: PRISM-SRS-022
- **Priority**: P0
- **Scope**: `prism evidence`, `prism gate --emit-evidence`
- **Definition**
  - Generate immutable evidence bundles containing score inputs, decision graph, and approvals.
  - Support JSON+signed envelope output compatible with external compliance tooling.
- **Acceptance**
  - Each gate run produces a single bundle reference with digest.
- **Tests**
  - Evidence bundle schema/manifest test
  - Bundle replay verification test

### BLK-013 — What-if simulation before execution
- **SRS Linkage**: PRISM-SRS-023
- **Priority**: P0
- **Scope**: `prism simulate`
- **Definition**
  - Add deterministic simulation mode for profile changes, deferrals, and blast-radius shifts.
  - No state mutation allowed; output includes projected risk reduction and rollback cost.
- **Acceptance**
  - Simulated output is stable for fixed inputs.
  - Simulation clearly marks no mutation and can be replayed.
- **Tests**
  - Simulation determinism test
  - Non-mutation enforcement test

### BLK-014 — Operator query API and orchestration interface
- **SRS Linkage**: PRISM-SRS-024
- **Priority**: P1
- **Scope**: `prism query`, optional local API server
- **Definition**
  - Add machine-readable endpoints/commands for top risks, blockers, receipts, and handoff packet retrieval.
  - Keep JSON schema versioned and compatible across CLI/API modes.
- **Acceptance**
  - Query responses include run hashes and deterministic ordering.
  - CLI and API produce equivalent payloads for shared queries.
- **Tests**
  - API/CLI parity test
  - Query ordering determinism test

### BLK-015 — Cross-domain dependency transfer risk modeling
- **SRS Linkage**: PRISM-SRS-025
- **Priority**: P1
- **Scope**: atlas scoring inputs and plan ordering
- **Definition**
  - Add dependency transfer risk model for coupled modules and ownership concentration paths.
  - Surface model outputs in priority explanations and plan rationale.
- **Acceptance**
  - Transfer risk is deterministic and additive in score explanations.
  - Migration blockers are flagged with explicit dependency links.
- **Tests**
  - Coupling graph determinism test
  - Blocker trace regression test

### BLK-016 — Incident-class playbooks and deterministic remediation sequences
- **SRS Linkage**: PRISM-SRS-026
- **Priority**: P1
- **Scope**: incident workflow engine
- **Definition**
  - Introduce deterministic playbook templates for high-risk classes with required evidence and transitions.
  - Ensure playbook execution updates incident state and produces closure checks.
- **Acceptance**
  - Known incident classes auto-suggest ordered remediation sequence.
  - Replay of the same incident sequence yields identical markers.
- **Tests**
  - Playbook selection and state-transition test
  - Closure check completeness test

### BLK-017 — Access controls and command authorization
- **SRS Linkage**: PRISM-SRS-027
- **Priority**: P1
- **Scope**: command executor and audit log
- **Definition**
  - Implement role-gated command tiers and enforce authorization on mutating paths.
  - Persist immutable authorization and audit events with command identity.
- **Acceptance**
  - Unauthorized mutating commands are rejected with auditable reason code.
  - Approval metadata appears in execution receipts.
- **Tests**
  - Unauthorized command rejection test
  - Audit trace integrity test

### BLK-018 — Deterministic replay and run lineage
- **SRS Linkage**: PRISM-SRS-028
- **Priority**: P2
- **Scope**: run registry, replay utility
- **Definition**
  - Store run lineage and support diff/replay by snapshot/config/command tuple.
  - Provide run-diff output for audit and debug.
- **Acceptance**
  - Replay produces byte-for-byte parity in deterministic mode.
  - Lineage chain can be queried and validated.
- **Tests**
  - Replay parity test
  - Lineage chain integrity test

### BLK-019 — Drift and health alerting
- **SRS Linkage**: PRISM-SRS-029
- **Priority**: P2
- **Scope**: health monitor, report generation
- **Definition**
  - Add drift detectors and health alerts for key governance/stability trends.
  - Provide throttled output with explicit emergency override for blocking risks.
- **Acceptance**
  - Alert signals are explainable with severity and remediation linkage.
  - Release-blocking signals cannot be fully suppressed.
- **Tests**
  - Drift detection and alert threshold test
  - Suppression policy override test

### BLK-020 — Controlled adaptive scoring calibration
- **SRS Linkage**: PRISM-SRS-030
- **Priority**: P2
- **Scope**: scoring config, calibration command
- **Definition**
  - Add bounded calibration updates from resolved incidents and operator feedback.
  - Make calibration changes explicit and revertible.
- **Acceptance**
  - Calibration updates include deltas, rationale, and versioned rollback marker.
  - Static mode remains default.
- **Tests**
  - Calibration diff and rollback test
  - Evidence recording test for coefficient update

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
11. BLK-011
12. BLK-012
13. BLK-013
14. BLK-014
15. BLK-015
16. BLK-016
17. BLK-017
18. BLK-018
19. BLK-019
20. BLK-020
