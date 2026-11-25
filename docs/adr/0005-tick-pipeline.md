# ADR-0005 — Tick pipeline: ordered stages with stage-end commit

Status: Accepted · Date: 2026-11-22 · Workstream: WS-02

## Context
Each monthly tick runs Decide → Resolve → Inform (ConOps §2). Subsystems are coupled (P-8): tax revenue depends on this month's output, faction approval on this month's taxes. The order in which subsystems see each other's updates must be explicit and deterministic (P-4).

## Options
| Option | For | Against |
|---|---|---|
| In-place sequential updates | Simplest; no lag | Each subsystem's inputs depend silently on call order; reordering changes results; hard to review |
| Full double buffer (all read t−1, write t) | Order-independent; trivially parallel | One-month lag on every coupling, including ones that should be same-month (output → revenue) |
| **Ordered stages; read stage-start state, commit at stage end; one writer per block** | Couplings across stages are same-tick; couplings within a stage are order-independent; lags are visible in the stage table; borrow checker enforces it (`fn(&World) -> NewBlock`) | Subsystems must be placed in the right stage; a subsystem split across stages (economy production vs. finance) needs two functions |

## Decision
**Ordered stages:** D1 Orders, D2 AiDecide, R1 ApplyOrders, R2 Exogenous, R3 Production, R4 Finance, R5 Politics, R5b Forces, R6 External, R6b Losses, R7 Tier, I1 Observe, I2 Report. A subsystem is a plain function `fn(&World, &mut TickCtx) -> NewBlock(s)`, called from a static, hand-written stage function. No runtime registry. Each block has exactly one writing function per stage. Belief updates happen only in I1 from queued `Observation`s. Tier switches happen only in R7.

## Consequences
- Stage placement is a design decision each WS spec must state, with the lag it implies.
- AI decisions (D2) see last tick's committed state; that one-month information lag is intended.
- Per-polity work inside a subsystem can later be parallelized without changing results, because RNG streams are keyed (ADR-0006) and the stage reads immutable state.
