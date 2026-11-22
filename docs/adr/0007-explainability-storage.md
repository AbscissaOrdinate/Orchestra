# ADR-0007 — Explainability: inline breakdowns, snapshot ring, replay

Status: Proposed · Date: 2026-11-22 · Workstream: WS-02 · Supports: SYS-004, P-3

## Context
Every displayed variable must decompose into labeled terms that sum to its value (SYS-004). The UI must answer "why did X change?" for recent months. Estimate [D]: ~1,600 tracked scalars × ~6 terms ≈ 0.4 MB of breakdowns per tick.

## Options
| Option | For | Against |
|---|---|---|
| Keep every tick | Instant history | ≈ 0.5 GB over 1,260 ticks |
| Current tick only | Smallest | Can't explain last month |
| Recompute on demand always | No storage | Every query waits on a replay; current tick needs it anyway |
| **Inline current tick + ring of last 12 `World` snapshots + replay from autosave for older ticks** | Recent history instant; older history exact via determinism; memory bounded (≈ 12 × ~5 MB) | Replay latency for old ticks (≤ 11 ticks of sim time) |

## Decision
A displayed variable is a `Tracked { value, why: Breakdown }` with a private value, settable only with a `Breakdown`, so the invariant is enforced by type. Multiplicative models convert to additive terms by log share (exact sum). Terms can `link` to another variable for drill-down. `History` keeps the last 12 snapshots as `Arc<World>` and autosaves every 12 ticks; `explain_at` replays from the nearest autosave.

## Consequences
- Breakdowns are part of the saved and hashed state. They're deterministic, so this strengthens SYS-002 checks.
- Internal arrays (cohorts, income quantiles) needn't be `Tracked`; their displayed aggregates must be.
- Long-run charts use a separate value-only recorder, not breakdown history.
