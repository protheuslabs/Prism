# Prism ⇄ LensMap Reference (Boundary Contract)

## Prism Scope

Prism owns complexity orchestration for large codebases.

In scope:
- Risk scoring and prioritization over code, modules, and change queues
- Work planning under operator constraints (time budget, risk tier, blast radius)
- Task execution scaffolds and gate evaluations (dry-run + receipt-backed actions)
- Incident recording, mitigation states, and evidence receipts
- Release-readiness readiness signals derived from policy and operational risk

Out of scope:
- Anchor format, note migration, or source-linked documentation editing
- Direct policy syntax authoring for annotations
- Source code refactor or merge-safety mechanics

## LensMap Inputs to Prism

Prism expects LensMap to provide deterministic governance signals:
- Anchor coverage and unresolved anchor/missing-link reports
- Owner/review/compliance metadata for high-criticality zones
- Stale/debt and annotation health signals
- Domain/classification context for cross-boundary risk weighting

## Hand-off Rule

LensMap exports evidence and governance truth.
Prism converts those signals into ranked operator plans and execution receipts.
