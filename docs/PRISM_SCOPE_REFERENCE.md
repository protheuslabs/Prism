# Prism ⇄ LensMap Reference (Boundary Contract)

## Philosophy

Prism is the "operational leverage" half of the system.

It is designed so a single operator can govern extremely large codebases by making the smallest number of highest-leverage decisions.

Where LensMap turns code complexity into explainable context, Prism turns that context into a bounded, auditable action plan.
- Long leverage = one-person throughput with strict risk and scope controls.
- A small set of deterministic decisions each cycle beats broad manual firefighting.

## Prism Scope

Prism owns complexity orchestration for large codebases.

In scope:
- Risk scoring and prioritization over code, modules, and change queues
- Work planning under operator constraints (time budget, risk tier, blast radius)
- Task execution scaffolds and gate evaluations (dry-run + receipt-backed actions)
- Incident recording, mitigation states, and evidence receipts
- Release-readiness readiness signals derived from policy and operational risk

Primary mission target: one operator can govern and execute on 1M+ LOC systems with deterministic leverage, and the roadmap scales this path toward 30M LOC estates.

Out of scope:
- Anchor format, note migration, or source-linked documentation editing
- Direct policy syntax authoring for annotations
- Source code refactor or merge-safety mechanics

## Why this boundary exists

Without the boundary, orchestration can become a second source of source/annotation drift.

With this boundary:
- Prism stays execution-first and human-capacity-aware.
- LensMap stays source-truth-first and evidence-first.
- Both can evolve independently while sharing deterministic interfaces.

## LensMap Inputs to Prism

Prism expects LensMap to provide deterministic governance signals:
- Anchor coverage and unresolved anchor/missing-link reports
- Owner/review/compliance metadata for high-criticality zones
- Stale/debt and annotation health signals
- Domain/classification context for cross-boundary risk weighting

Operational sequencing and acceptance detail lives in [`PRISM_BACKLOG.md`](/Users/jay/.openclaw/workspace/apps/prism/docs/PRISM_BACKLOG.md).

## Hand-off Rule

LensMap exports evidence and governance truth.
Prism converts those signals into ranked operator plans and execution receipts.
