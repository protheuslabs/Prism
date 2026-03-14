# PRISM SRS (Version 0.1)

## Purpose

Prism provides a practical complexity-management control plane for a single operator running a very large codebase. It converts repository signals into deterministic decisions and verifiable action plans.

## Scope (Initial Tranche)

1. Deterministic risk scoring for changed / critical modules.
2. Execution queue synthesis with budget-aware plans.
3. Gate evaluation for release and merge safety.
4. Receipted incident tracking.
5. LensMap interoperability contract (read-only in v0.1).

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

## Non-Goals (v0.1)

- Not a replacement for IDE refactoring tools.
- Not a source-of-truth code formatter/linters.
- Not a policy authoring engine (uses external systems for policy governance).
