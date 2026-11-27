# M2(c) — Governance & factions prototype plan

Workstream WS-04 · Spec: `docs/specs/WS-04-governance.md` (Draft v0.3) · Language: Python 3 + NumPy (+ matplotlib for plots) · Mirrors the M2(a) conventions in `prototypes/economy/README.md` (one function per stage, leading `runs` axis, PCG64 seeds, parameters in one file)

## 1. Goal

Show that the WS-04 model is **stable over 1955–2060** (1,260 monthly ticks), **produces the intended qualitative behavior** (selectorate trade-off, regime-dependent crisis types), and **hits the calibration bands** of spec §7.2. Then freeze every `[D]` constant into `params.ron` values for the Rust port (P-9). The prototype is a model sketch, not a port: no breakdown plumbing, no save format, but the same equations and the same stub interfaces.

M2(c) exit criteria (from the POAM): batch-run and shown stable over 1955–2060, plus:
1. GOV-035, 044, 057, 058, 063, 076 and 092 pass in the prototype at full batch counts.
2. The fiscal-squeeze scenario (S2) shows the predicted divergence between oligarchy and democracy.
3. Every changed default is written back to spec §4 with its new value and tag before the spec is approved.

## 2. Layout

```
prototypes/governance/
  PLAN.md          this file
  params.py        every [D] constant from spec §4, one dict; scenarios override keys
  presets.py       the 11 presets (§6.1), 9 institutions, 12 factions (§6.2), 8 axes and the policy map, as dicts
  stubs.py         econ / force / diplo stubs implementing the WS-03 §2.4 types and spec §3.3 fields
  gov.py           the model: one step(state, inputs, params, rng) -> state, vectorized over runs
  scenarios.py     S1–S9 below; each returns a dict of metric arrays
  run_batch.py     CLI: python run_batch.py S2 --runs 200 --seed 1 --out out/S2.npz
  report.py        plots + a pass/fail table against the acceptance thresholds
  out/             results (only summary tables are committed)
```

Conventions (CLAUDE.md): helper functions at the bottom of each script; vectorize with NumPy over a leading **runs** axis, so state arrays are `(runs, factions)`, `(runs, cells)` and so on, and one tick is one set of array operations for every run at once. Randomness uses `np.random.Generator(PCG64)` with one generator per (scenario, run, purpose) derived from the seed, mirroring the WS-02 keyed streams so results don't depend on draw order. (Bit-exact parity with the Rust ChaCha streams is not a goal.)

## 3. Stubs (shared interface with the Rust tests)

| Stub | Fields (spec §3.2–§3.3) | Model |
|---|---|---|
| `econ` | `EconOutcomes` per `EconKey` (20 bins × WS-03's 5 sectors × roles Wage/Capital, plus Transfer and Public keys), with `wealth` from the WS-03 §3.5 household panel (or, until the M2(a) prototype exists, the v0.2 fallback: c = 0.9, role factors normalized within each bin, with the US marginals from `prototypes/economy/calib_wealth.py`); budget lines (WS-03 §3.6 list); corruption rents = 0.4·K × discretionary spending (WS-03 leakage rule); stress indicators; a linear `project()` | Lognormal body + Pareto tail (σ = 0.7, tail α = 2.0 above p90) [D]; real growth AR(1), mean 3%/yr, sd 2% [D]; inflation AR(1) around 4%; sector shares drift from agriculture to services over 1955–2060; progressive income tax + VAT; transfers to bins ≤ 4. Consumes `cbi`, `corruption`, `stat_bias` and `FactionEconBehavior` with the one-tick lag (invest_sentiment scales growth; labor_militancy adds strike losses). |
| `force` | personnel and readiness per service (army, navy, air, space, internal security), strategic share of defense spending | Personnel ∝ defense budget ÷ wage, service split from content; readiness 0.7 ± 0.1; space personnel 0 until 1965 |
| `diplo` | alignment[j], threat[j], war_burden | Scripted per scenario (S6 injects a war) |

When the M2(a) economy prototype exists, swap `stubs.econ` for it behind the same WS-03 §2.4 types; S2 and S5 are rerun as the first coupled test.

## 4. Scenarios

Default batch: 200 runs × 1,260 ticks per preset unless stated; neutral scripted policy = hold every instrument at its x⁰ and keep the starting coalition when valid.

| ID | Scenario | Setup | Metrics | Pass criterion (spec ID) |
|---|---|---|---|---|
| S1 | **Baseline per preset** | Each of the 11 presets, neutral policy | Coup attempts and revolutions per polity-year; coup success share; mean regime duration; share of months with R > 0.9; approval pinning | §7.2 bands and success share in [0.35, 0.65] (GOV-056); boundedness (GOV-057) |
| S2 | **Fiscal squeeze: oligarchy vs. democracy** | `oligarchic_republic` vs. `parliamentary_republic`, same economy. At 1965 a debt shock forces a 5-year consolidation of 4% of GDP. Three policy arms per regime: (a) cut public goods and transfers, (b) raise progressive taxes, (c) cut rents by lowering Spoils. Budgets go through `EnactBudget` and its institutional vetoes. | P(coalition turnover within 5 yr); P(coup); P(revolution); peak unrest; legitimacy trough; which arm the regime's own survival metric prefers | **Pass:** for every arm, democracy's turnover probability > oligarchy's, and its irregular-exit probability (coup + revolution) < oligarchy's, each by > 3 SE. **Expected, reported either way:** in the oligarchy, arm (c) raises coup risk most when coercive factions are in the coalition, arm (a) raises unrest most, and the oligarchs/legislature veto blocks arm (b) most often; in the democracy, arm (a) costs the most elections. |
| S3 | **Selectorate sweep** | ρ ∈ {0.01, 0.02, 0.05, 0.1, 0.2, 0.35, 0.5, 0.7} × S_share ∈ {0.01, 0.1, 0.65}; marginal 0.1% of GDP as corruption rents (via K) vs. public goods | Survival-support gain per unit spend (spec GOV-035 metric); crossover ρ* per S_share | Crossover exists and moves with S_share as (1 − ρ)/(β·ρ·S_share) ≈ 1 predicts (within one grid step); presets land on the sides GOV-035 states |
| S4 | **Democratization under veto players** | `oligarchic_republic` proposes universal franchise (D ≈ 1) with 0–3 reform veto players, and a smaller step (q ≥ 0.7, D ≈ 0.3) | Reform success rate; time to ratify; coup/revolution during transition; share of decree reforms when the player script falls back to decree after a veto | Success non-increasing in veto count and D (GOV-076); transition crises > baseline |
| S5 | **Inequality drift** | `oligarchic_republic` and `parliamentary_republic`; top-decile wealth share rises 0.30 → 0.55 over 1955–2000 (scripted in the stub; once M2(a) exists, driven by WS-03 v0.3 five-group dynamics through a falling estate and top tax rate) | Clout of top-quantile factions; Labor and Rural approval; unrest; revolution rate | Clout concentration rises only where w_w > 0 (GOV-013); oligarchy's revolution rate rises; democracy's doesn't by > 2× |
| S6 | **Military defeat** | `military_junta`, `one_party_state`, `parliamentary_republic`; a 24-month losing war in 1970 (war_burden ramps to 5 per 1,000) | Approval of service factions; coup probability; election loss; legitimacy | Junta's coup risk rises most; democracy's election-loss risk rises most (Argentina 1982 is the qualitative reference [R]) |
| S7 | **Tier round trip** | Each preset; demote at 1980, promote at 1990, compare to an always-high run | Conservation errors; post-promotion drift of Σ, L, R over 24 ticks | GOV-091 within 1e-6; macro vs. high-fidelity crisis rates within ×1.5 (GOV-092) |
| S9 | **Institutions and programs** | (i) Same economy, presets differing only in institution weights (e.g., `parliamentary_republic` vs. `presidential_republic`, junta security weight 0.25 vs. 0.45); (ii) Spoils ramp 0 → 0.8 over 20 years under `single_party`; (iii) Unity intensity 0–3 with free vs. censored press | Crisis-type mix; budget block rate; K path vs. floor; L and R response to Unity | Higher security weight → higher coup share of irregular exits; presidential λ 0.3 blocks more budgets than parliamentary; K never falls faster than the floor allows (GOV-052); Unity effect with censored press ≈ 3.3× free press (m = 1 vs. 0.3) |
| S8 | **Random constitutions** | 1,000 random valid `Institutions` (GOV-003 sampler), 120 ticks each, plus 50 of them for 1,260 ticks | Non-finite values; clout sum; time pinned at extremes | GOV-003 and GOV-057 hold for every sample; no parameter region produces runaway unrest under neutral policy (else: add damping and record it in the spec) |

S9 runs after S1–S3. Additional checks, run once rather than batched: GOV-044 (election loss vs. mean approval) and GOV-063 (veto players vs. acceptance) as quick sweeps inside S1's start states.

## 5. Tuning procedure

1. Implement `gov.py` with spec defaults; run S8 first (crash and boundedness), then S1.
2. Tune in this order, one block of constants at a time, recording each change: coup logit intercept and Λ weight (S1 junta band) → succession μ (regime durations) → revolution intercept and R weight (S1 revolution bands) → unrest τ and repression cap (GOV-057) → election k_e, b_e (incumbent-loss rate ~ 30–50% per election in democracies [R]) → reform v₀ and backlash (S4 success rates in a plausible range, e.g., 20–60% for D ≈ 1 with one veto player [D]).
3. Freeze. Rerun S1–S9 with 3 new base seeds; every pass criterion must hold for all three.
4. Write back: changed values into spec §4 (tag stays [D], note "tuned M2(c)"), and the final tables and plots into `prototypes/governance/RESULTS.md`.

## 6. Effort estimate [D]

About 20–30 hours at Sean's pace with AI help: stubs 4 h, `gov.py` 8–10 h, scenarios and report 6 h, tuning 4–8 h, write-back 2 h. S2 and S3 are the showcase plots and go first after S1.

## 7. Risks

| Risk | Mitigation |
|---|---|
| Too many [D] constants to tune jointly | Tune in the fixed order above; most constants only affect one band |
| Approval saturates (tanh pinned) | Check the U distribution in S1; adjust u_s before weights |
| Stub economy too smooth to stress politics | S2, S5 and S6 inject shocks explicitly; rerun with the WS-03 prototype when it exists |
| Python speed: 11 presets × 200 runs × 1,260 ticks | Vectorized over runs; expected < 1 min per preset batch [D]; parallelize presets with `multiprocessing` if not |
