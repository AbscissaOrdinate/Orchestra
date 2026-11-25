# ADR-0007 — Explainability: inline polity-level breakdowns, lazy unit-level breakdowns, snapshot ring, replay

Status: Accepted · Date: 2026-11-22 (revised 2026-11-25: OQ-A2, OQ-A6) · Workstream: WS-02 · Supports: SYS-004, SYS-009, P-3

## Context
Every displayed variable must decompose into labeled terms that sum to its value (SYS-004), including each tracked unit's aggregates (SYS-009). The UI must answer "why did X change?" for recent months. Estimates [D]:
- Polity- and world-level: ~1,600 tracked scalars × ~6 terms ≈ 0.4 MB of breakdowns per tick.
- Unit-level (WS-11): ~1,800 units × ~8 stock variables × ~6 terms ≈ 3.5 MB per tick inline, against ≈ 0.1 MB as plain values.

## Options
| Option | For | Against |
|---|---|---|
| Keep every tick | Instant history | ≈ 0.5 GB over 1,260 ticks for polity-level alone |
| Current tick only | Smallest | Can't explain last month |
| Recompute on demand always | No storage | Every query waits on a replay |
| Inline everything + snapshot ring | Uniform | Unit breakdowns are ~9× the polity-level total and are rarely inspected; ring at 12 would exceed 100 MB from breakdowns alone |
| **Inline polity-level (`Tracked`); unit-level values only (`Lazy`), rebuilt by a one-tick capture replay; user-sized snapshot ring; replay from saves for older ticks** | Memory bounded; unit queries cost one tick of replay (≤ 100 ms); exactness guaranteed by determinism | Two types to choose between; capture plumbing in `TickCtx` |

## Decision
- `Tracked { value, why }` and `Lazy { value }` both have a private value set only by `set(Breakdown)`, so the invariant is enforced by type. `Lazy` discards the breakdown unless the tick context is capturing that key.
- Per-unit and other bulk state is `Lazy`; polity/world variables and unit rollups are `Tracked`.
- `explain_change` diffs two breakdowns, which answers "why did X change?" for rollups such as `military_power`.
- `History` keeps the current world plus `ring` previous snapshots. `ring` and `autosave_every` are user settings from {1, 2, 3, 6, 12, 18, 24}, default 12. The newest 3 autosaves are kept; user saves persist. Older ticks resolve via autosave → user save → scenario start.
- Budget: ≤ 8 MB per snapshot; ring memory = (ring + 1) × snapshot (≤ 104 MB at the default).

## Consequences
- Breakdowns of `Tracked` variables are saved and hashed; they're deterministic, so this strengthens SYS-002 checks. History settings are not world state and can't change results.
- `Lazy::set` must always build the breakdown (cheap, on the stack) so the value is identical in normal and capture runs; only storage is skipped. ARC-044 tests the equivalence.
- Long-run charts use a separate value-only recorder, not breakdown history.
