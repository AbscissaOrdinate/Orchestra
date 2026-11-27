# M2(a) Economy prototype plan (WS-03)

Status: Plan v0.3 · Spec: `docs/specs/WS-03-economy.md` (Draft v0.3) · Last updated: 2026-11-27

Goal: show that the WS-03 model is stable over 1955–2060 in batch Monte Carlo (spec §6.1), that every failure mode is reachable by policy on a plausible timescale (§6.2), and that it *feels* like TNO's slow pressure. The parameters this produces become the M4 content defaults.

## 1. Approach

- **Python 3 + NumPy only** (P-9). No pandas or SciPy unless a step needs them (Student-t and logistic are one-liners in NumPy).
- **Vectorize across runs.** Every state array has a leading `runs` axis (e.g. `K[runs, sectors]`, `pop[runs, sex, age]`). One batch of 1,000 runs costs about as much as one Python loop over 1,260 months. Helper functions go at the bottom of each script (repo convention).
- **Same structure as the Rust target.** One function per stage (`exogenous`, `production`, `finance`, `tier`) reading the stage-start state and returning new sub-blocks. Porting is then translation, not redesign.
- **Parameters in one file** (`params.toml`, tagged [V]/[R]/[D] in comments). M4 converts it to RON.
- **Deterministic seeds:** `numpy.random.Generator(PCG64(seed + run))`. Bit-compatibility with ChaCha8 is not a goal; statistical equivalence is.
- **WS-04/05/06/11 are exogenous scenario inputs** (stub time series for CBI, corruption, faction behavior, TFP multipliers, access matrix, upkeep demand). This keeps M2(a) independent of M2(b)/(c).

## 2. Files

| File | Contents |
|---|---|
| `params.toml` | All §3 defaults, per archetype |
| `model.py` | State containers and stage functions (§3.1–3.13) |
| `calib_wealth.py` | **Exists.** Fits the wealth marginals to Fed DFA / WIR shares and the joint panel to SCF 2022 (§3.5); reproduces the spec's calibration table |
| `panel_size.py` | **Exists.** Panel-size study (equal vs. oversampled) behind the 8,192 choice |
| `panel.py` | Household panel (oversampled lattice, weighted rank-cell marginals, roles, keys, shrinkage), tax engine, incidence (§3.5); runnable standalone for the Laffer, bracket-creep and ECO-048/049 checks |
| `wealth_groups.py` | Five-group wealth stock-flow, homeownership, q_H/q_E, pensions, offshore, generalized Pareto rebuild (§3.5 wealth dynamics); ECO-140–143, 147 |
| `demography.py` | Single-year two-sex cohort component, Preston/Brass mortality, TFR (§3.9) |
| `scenarios.py` | Scenario definitions (§3 below) as dicts of overrides plus stub input series |
| `batch.py` | `python batch.py --scenario S04 --runs 500 --seed 1` → `results/S04.npz` + summary CSV |
| `criteria.py` | Evaluates spec §6.1/§6.2 and selected ECO tests on the results; prints a pass/fail table and writes `results/criteria.md` |
| `feel.py` | Fan charts for the review (debt/GDP, spread, inflation, real growth, interest/revenue; median and 5–95% bands) |

## 3. Scenarios (all 1955-01 to 2060-12, monthly)

Archetypes: **A** advanced market democracy (high CBI, d_lim 1.6); **B** planned economy (CBI 0, price controls 0.6, state ownership 0.8, corruption 0.4); **C** developing (low K/Y, TFR ≈ 6, corruption 0.3, d_lim 0.7).

| ID | Scenario | Polities | Runs | Checks |
|---|---|---|---|---|
| S00 | Shocks off, A/B/C | 1 each | 1 | Identities: ECO-012, 053, 054, 080; determinism |
| S01 | Baseline A (AI fiscal rule on) | 1 | 1,000 | §6.1 rows for A |
| S02 | Baseline B | 1 | 1,000 | §6.1; shortage band frequency reported |
| S03 | Baseline C | 1 | 1,000 | §6.1; demographic transition timing |
| S04 | Mixed world: 3 high-fidelity (A, B, C) + 10 macro, trade on | 13 | 200 | §6.1 per polity; catch-up row; ECO-022 correlation |
| S05 | Profligate (A, primary deficit 8%, CBI 0.5) | 1 | 500 | §6.2 row 1 |
| S06 | Printing press (C, financing 10% GDP/yr, CBI 0) | 1 | 500 | §6.2 row 2; ECO-033 |
| S07 | Consolidation from d = 120% (A, surplus 4%) | 1 | 500 | §6.2 row 3 |
| S08 | Laffer sweep (top MTR 0–95% step 5) | 1 | 20 × 100 | §6.2 row 4; ECO-043 |
| S09 | Embargo at 1975 (C's largest partner) | 13 | 200 | ECO-070 |
| S10 | Price-control release in 1990 (B) | 1 | 500 | ECO-034 |
| S11 | Oil shock: scripted global factor +3σ in 1973 and 1979 | 13 | 200 | Inflation +3–10 pp in A, recession probability up; no clamp hits |
| S12 | War mobilization 1965–70 (A: defense 25% GDP, 8% LF drawn) | 1 | 500 | §6.2 row 8; ECO-016, 084 |
| S13 | Baby boom (TFR +1 for 1946–60 start cohorts) | 1 | 500 | Pension and health spending peak 2010–2030; debt pressure visible ≥ 10 years ahead in the projection |
| S14 | Tax regime swing: flat 20% vs. 1955-style 91% top rate (A) | 1 | 2 × 500 | Gini and top-share bounds; revenue difference; ETI response |
| S15 | Bailout path (C hits critical debt, lender available) | 1 | 500 | ECO-065, 067 continuity; CI path |
| S16 | Tier round trip (promote/demote every 10 years) | 13 | 200 | ECO-121, 122 |
| S17 | Projection accuracy (beliefs = truth vs. σ 5% vs. 30%) | 1 | 200 | ECO-102 |
| S18 | Inequality drift: advanced archetype, top tax rates cut 1980 and growth slowing (WS-04 S5 counterpart) | 1 | 500 | §6.1 wealth bounds; W_hh/Y rises; outputs feed WS-04 S5 as its econ input |
| S19 | US policy path 1955–2012 (spec §3.5 table) | 1 | 200 | ECO-144 (US) |
| S20 | UK policy path 1955–1975 | 1 | 200 | ECO-144 (UK), ECO-146 |
| S21 | NL-like vs. BE-like pair, same tax-transfer schedule, 40 years | 2 | 200 | ECO-145 |

Sensitivity: rerun S01, S05 and S06 with σ_CES ∈ {0.4, 0.6, 1.0}, ETI ∈ {0.1, 0.25, 0.5}, β_d ∈ {2, 3, 4} bp, k ∈ {0.03, 0.06, 0.1}. Report which criteria are fragile.

## 4. Order of work and estimate

| Phase | Work | Exit | Est. h [D] |
|---|---|---|---|
| P1 | `panel.py`, `wealth_groups.py`, `demography.py` standalone | ECO-040–044, 048, 049, 080–083, 140–143, 147 pass | 11 |
| P2 | Single-polity closed economy: production, IS gap, Phillips/Taylor/Cagan, budget, bonds | S00, S05–S07 pass | 8 |
| P3 | Distribution dynamics, corruption, shadow economy, planned settings, beliefs + projection | S02, S10, S17 pass | 6 |
| P4 | Multi-polity: trade, FX, diffusion, macro tier | S04, S09, S11, S16 pass | 6 |
| P5 | Batch criteria, calibration (incl. S19–S21), sensitivity, `feel.py` | All §6.1/§6.2 rows and ECO-144–146 pass; criteria.md committed | 7 |
| — | **Total** | | **≈ 38 h** (≈ 4 weeks at 10 h/week) |

## 5. Feel review (Sean, end of P5)

Look at the S01/S05/S13 fan charts and the projection for S13 and answer:
1. Is debt pressure visible years before it bites (spread and interest/revenue rising slowly)?
2. Does inflation visibly eat real growth in S06 and S11?
3. Does the embargo in S09 hurt enough to make trade relations matter?
4. Is the projection useful but wrong often enough to keep you honest?

Any "no" becomes a parameter or spec change request (CLAUDE.md: spec changes go through the Project).

## 6. Outputs to commit

`results/criteria.md` (pass/fail tables), calibrated `params.toml`, the feel-review plots (PNG, small), and a short `findings.md` listing spec changes the prototype requires. Raw `.npz` results are git-ignored.
