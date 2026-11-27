# WS-08b prototypes: Sol graph (done) and M2(d) rent hook (plan)

Status: Plan v0.2 · Spec: `docs/specs/WS-08b-space.md` (Draft v0.3) · Last updated: 2026-11-27

## 1. `sol.py`: Sol graph and worldgen checks (done)

`python sol.py [content_dir]` reads `content/maps/sol/*.ron`, `content/maps/earth/resources.ron` and `content/core/resources/body_deposits.ron`, builds the delta-v graph with the WS-08b §4.2 formulas, and prints the hand checks, the Earth–Mars windows against historical launches, 2,000-seed worldgen statistics and the belief-subject count. NumPy only; it runs in about 3 s. The results are recorded in spec §7.1 and back WLD-053–056, 061–064, 066–070, 072 and 079 (verification column **P**).

It is a sketch, not the implementation: its minimal RON reader handles only this repo's content, and it uses PCG64 rather than ChaCha8 (statistical equivalence only, as in M2(a)). M4 ports the formulas to `sim-core` and replaces these checks with `wld_*` tests.

## 2. M2(d): rent hook and commodity prices (plan)

**Goal:** show that the WS-08b §4.5–4.7 resource block leaves WS-03's §6.1 stability criteria intact (WLD-089), that `f_g` is a windfall for exporters and a loss for importers (WLD-085), and that world rents stay in a historically plausible band. The parameters it produces become the M4 content defaults in `content/core/resources/`.

**Approach:** extend the M2(a) economy prototype (`prototypes/economy/`; so far only the WS-03 v0.3 wealth calibration scripts exist) rather than duplicate it. The resource-equity line `V_res` (WS-08b §4.6) plugs into the five-group wealth dynamics there.

| File | Contents |
|---|---|
| `prototypes/economy/resources.py` | Deposit truth sampling (§4.4), price block (§4.5), production, rent split and resource equity `V_res` (§4.6), commodity net positions (§4.7). Vectorized over `runs × deposits`; one function per stage, reading stage-start state, as in M2(a) |
| `prototypes/economy/scenarios.py` (additions) | Archetype D (resource exporter: one supergiant or two giant deposits, high royalty or SOE share, corruption 0.4); S11 via p_k; S22; S23 (WS-03 uses S00–S21) |
| `prototypes/economy/criteria.py` (additions) | WLD-074, WLD-085, WLD-089 and the rows below |

**Scenarios** (1955-01 to 2060-12, monthly):

| ID | Scenario | Runs | Criterion |
|---|---|---|---|
| S00r | Shocks off, A/B/C/D | 1 | Identities: Σ split = rent; Σ_i n = 0; cum ≤ S |
| S01r–S03r | WS-03 baselines A, B, C with the default Earth tags | 1,000 each | Every WS-03 §6.1 row passes; 0 `ClampHit` |
| S04r | Mixed world, 13 polities incl. 2 × D, trade on | 200 | §6.1 per polity; world rents/GDP in [1%, 8%] for every year of 1970–2020 [R: WDI band]; median 1955 world oil output within ±30% of 5.6 Gboe/yr [R] |
| S11 | f_g +3σ in 1973 and 1979 | 200 | D: rent/GDP ≥ 1.5× its pre-shock level within 24 months; A: π +3 to +10 pp (WS-03 row kept) |
| S22 | f_g −2σ from 1986 for 3 years | 200 | D: budget deficit widens ≥ 3 pp of GDP within 2 years in ≥ 70% of runs; no debt spiral within 5 years in ≥ 80% (slow build) |
| S23 | Discovery windfall: an `unknown` giant discovered in 1969, developed by 1975 | 200 | GDP of the owner +2 to +8% by 1980 vs. no-discovery run; price dips while capacity exceeds demand |
| SWk | Windfall check (WLD-074, WLD-085) | 200 | Exporter rent, GDP, revenue, reserves and V_res up, and its T01 group's wealth share up through Gain_g; importer NX and reserves down, π up; 200/200 seeds |

**Sensitivity:** ε_k ∈ {0.15, 0.25, 0.4}; τ_p ∈ {6, 12, 24}; λ residual ∈ {0.2, 0.3, 0.5}; c_div ∈ {0.2, 0.4}. Report which criteria are fragile.

**Calibration targets [R]** (to verify against WDI and BP-style historical series during M2(d)): world oil output 1955 ≈ 5.6 Gboe/yr; world resource rents ≈ 1–2% of GDP around 1970, peaking ≈ 7–8% around 1979–80; Gulf exporters' rents above 30% of GDP in the 1970s.

**Order of work** (estimate [D]): P1 `resources.py` standalone with identity tests (3 h); P2 coupling to M2(a) stages (4 h); P3 baselines and S04r calibration (4 h); P4 S11/S18/S19 and sensitivity (3 h). **Total ≈ 14 h.** M2(d) depends on M2(a) P1–P3.
