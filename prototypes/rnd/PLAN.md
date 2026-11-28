# M2(b) — R&D latent-truth / belief prototype plan

Workstream WS-05 · Spec: `docs/specs/WS-05-rnd.md` (v0.3) · Language: Python 3 + NumPy (+ matplotlib for plots) · Same conventions as M2(a) and M2(c): one function per stage, a leading `worlds` axis, PCG64 generators per (scenario, world, purpose), all parameters in one file.

## 1. Goal

Show over 1955–2060 (1,260 monthly ticks) that the WS-05 model:
1. **converges**: beliefs narrow toward truth as research increases and stay calibrated (SYS-005, RND-012/013);
2. **has the intended dead-end frequency**, soft (gated) outnumbering hard, and that both are recognized on a plausible timescale (RND-004–008, 025, 026);
3. **produces cost-overrun and schedule distributions that match the anchors** (GAO first, then the ACAT CV, RAND schedule) and a calibrated `estimate()` (RND-033/038);
4. **can't spiral** through civil tech effects (`M_s`, RND-104).

Then freeze every `[D]` constant into `params.ron` values for M4 (P-9). The prototype is a model sketch: no breakdown plumbing or save format, but the same equations and the same stub interfaces.

**Exit criteria** (POAM M2(b)):
- RND-004, 005, 006, 007, 008, 011, 012, 013, 033, 038 and 104 pass at full batch size with three base seeds.
- Plots committed in `RESULTS.md`: belief convergence fans, hard and soft dead-end frequency histograms, cost-growth and schedule-growth distributions (overall and by TRL), the P80 coverage chart, and the kill-threshold ROC.
- Every changed default written back to spec §4 with its new value (tag stays [D], note "tuned M2(b)").

## 2. Layout

```
prototypes/rnd/
  PLAN.md            this file
  make_estimate_points.py  writes content/core/rnd/estimate_points.ron (§4.5.7); calib_growth.py retired in v0.3 (S-8)
  params.py          every [D] constant from spec §4, one dict; scenarios override keys
  content_gen.py     synthetic default content: 75 lines + 20 singletons over 12 domains with the §6.3 node counts,
                     class mix (45 historical, 128 Established, 76 Promising, 36 Speculative, 15 Fringe), dates 1930–2055,
                     2–5 gate candidates per line (acyclic line graph), 12 repeatable lines
  truth.py           worldgen: line Beta, per-generation works, soft gates, perf mixture, κ, Boost edges, repeatable gains
  belief.py          ADR-0008 mixture (≤ 3 comps, Linear/Log scale): observe(), mean/σ, p_works(), binary_evidence()
  science.py         field effort split, m_sci, precision accrual, staggered pooled emission, publication, discovery
  projects.py        slots, funding response, G/S/phase draws, EVM/EAC, tests, Requires/Penalty gates, Boost, spec change, estimate()
  production.py      learning curve with flattening, rate, capacity fill, producibility, revisions, O&S
  market.py          thin: two sellers, quotes, policy acceptance, offers and grants, macro buy/license split, embargo
  contractors.py     contract types, capital memo account, distress and bailout, private ventures, auto_design
  stubs.py           WS-03 (funded lines, cost index, fill rate), WS-04 (rnd_morale), WS-06 (alignment, access, intel_q)
  scenarios.py       B1–B8 below; each returns a dict of metric arrays
  run_batch.py       CLI: python run_batch.py B2 --worlds 200 --seed 1 --out out/B2.npz
  report.py          plots + pass/fail table against the acceptance thresholds → RESULTS.md
  out/               results (only summary tables and plots are committed)
```

**Status (2026-11-27):** built and passing: `params.py`, `content_gen.py`, `truth.py`, `belief.py`, `projects.py` (growth, `estimate`, phase split), `scenarios.py` (B1, B2-lite, B3), `run_batch.py`, `report.py`, `test_rnd.py` (named after RND IDs), `RESULTS.md`. Not built: `science.py`, `production.py`, `market.py`, `contractors.py`, `stubs.py`, B2 full and B4–B9. B2-lite runs every non-historical tech through four science steps and a test loop in generation order (cross-generation rule active), without the time loop, publication, espionage or observing others. Findings S-1–S-8 are in `RESULTS.md`.

**Deviation from this plan:** worldgen draws are a pure function of (seed, world, content ID, slot) using a splitmix64 hash (`truth.keyed_u`), not PCG64 per (scenario, world, purpose). That is the ARC-034 keyed-stream semantics, so RND-007's "adding content changes no other gate" is testable. Time-loop stages will use PCG64 per (scenario, world, purpose) as planned.

Conventions (CLAUDE.md): helpers at the bottom of each script; vectorize over a leading **worlds** axis (state arrays `(worlds, polities, techs)`, `(worlds, polities, projects)`), so one tick is one set of array operations for every world. Belief mixtures are stored as arrays `(…, 3)` of (w, μ, σ). Randomness uses `np.random.Generator(PCG64)` keyed per (scenario, world, purpose), mirroring the WS-02 keyed streams; bit parity with ChaCha8 is not a goal.

## 3. Stubs

| Stub | Fields | Model |
|---|---|---|
| WS-03 | paid amount and funded fraction per line; R&D cost index; defense/space capacity and fill rate; GDP (B5 coupled mode: GDP ∝ M_s, R&D ∝ GDP) | Archetype budgets as GDP shares (superpower 1.5% defense R&D + 0.5% civil R&D + space ramping 0 → 0.4% over 1957–67; middle power half; developing 0.1%) on GDP growing 3%/yr; cost index 1 + 0.01/yr real; fill rate 1 unless a scenario caps capacity |
| WS-04 | `rnd_morale` | 1.0 (B5 also runs 1.1) |
| WS-06 | alignment, access, `intel_q`, espionage events | Two blocs, fixed; access 1 within a bloc, 0.5 across; scripted embargo in B7 |

## 4. Scenarios

Default: 200 worlds × 1,260 ticks unless stated. "AI policy" = the spec defaults: science split by priority 1; start development when m_sci ≥ 0.67 and slots allow, highest p_works × perf first; budget at P50; cancel when p_works < θ_kill (default 0.15, swept in B4).

| ID | Scenario | Setup | Metrics | Pass criterion (spec ID) |
|---|---|---|---|---|
| B1 | **Truth calibration** | 10,000 worlds, worldgen only | Ungated/soft/hard shares per class; gate kinds and enabler counts; enabler date offsets; perf mode frequencies; κ moments; Boost edges per world; repeatable gain sums; hard and soft dead ends per world (all, and *reachable*) | RND-004–008; histograms of hard and soft dead ends (p5/p50/p95); soft/hard ≥ 2 |
| B2 | **Belief convergence** | 6 polities (2 superpowers, 2 middle, 2 developing), AI policy | Per tech: perf/cost σ and log error vs. cumulative effort; 80% interval coverage; `p_works` reliability diagram; time from first test to p_works < 0.10 for dead ends | RND-012, 013; fan charts of σ/σ₀ and |error| by decade 1955–2060 |
| B3 | **Cost and schedule overruns** | 10⁴ design projects and 10⁴ development projects drawn at the default start mix (16% mature, rest TRL_min ~ U[3, 5.5]); x = 1; then x ∈ {0.5, 2} | Realized/baseline cost and schedule: mean, CV, p50/p85/p95, by TRL; P50/P80 coverage; CPI change after 20% complete | RND-033, 038; CPI stability [R]: report the share of projects whose final CPI differs from CPI at 20% by > 10%, and tune the phase variance shares (0.6, 0.25, 0.15) to bring it down if most projects violate it |
| B4 | **Dead-end recognition, kill policy and gate response** | B2 setup; θ_kill ∈ {0.02, 0.05, 0.10, 0.15, 0.20, 0.30}; on a `Penalty` finding, AI pauses iff cost_mult > M_pause ∈ {2, 4, 8, ∞} | Money sunk in hard dead ends; working techs abandoned (Type II); months to kill; money spent paying gate penalties vs. years waited; techs proven by 2060 | ROC curve (sunk cost vs. false abandonment); pick the AI defaults θ_kill and M_pause; RND-025/026 |
| B5 | **1955–2060 portfolio** | B2 setup with publication, observing others and espionage (1 theft per bloc per year) on; run uncoupled and coupled (stub GDP ∝ M_s, R&D ∝ GDP) | Proof date vs. nominal date D; slots used; overruns by decade; dead-end spending share; fast-follower lag; repeatable levels reached; ln M_s per polity and its cross-polity sd | §7 batch-stability rows: finite values; ≥ 60% of non-dead-end techs with D ≤ 2040 proven by 2060; median proof within ±6 yr of D for funded Established techs; dead-end spending ≤ 25% of R&D; RND-104 (coupled) |
| B6 | **Production and learning** | Single lines per domain; quantity sweep 10–10⁴; rate ×0.5/×2; revision at N = 100 with χ ∈ {0.1, 0.4}; quantity cut 500 → 200 | Unit cost vs. N (plateau); lot cost; O&S share over 30 years; death-spiral ratio | RND-065, 066, 069, 070, 071 |
| B7 | **Market sanity** | 2 sellers (one per bloc), 10 macro buyers, 2 high-fidelity buyers; embargo by seller A on buyer X in 1975 | Price paths; seller N and unit cost (exports lower it); buyer dependency; halted lines and readiness funding after embargo | RND-080, 081, 084, 085, 088; qualitative: deals cheaper than spot; license cheapest per unit above ~50 units |
| B9 | **Contracting and ventures** | 10⁴ projects per contract type at u ∈ {0, 0.67, 1}; then B5 with contractors on (market polity with IB 1.5, planned polity with state bureaus) | Government cost, contractor capital paths, distress frequency, fixed-price refusals by u, venture starts per decade, adopted ventures | RND-040–044; qualitative: fixed price is refused or loses money at high u (KC-46 pattern); special access is fastest; commercial milestone is cheapest for eligible domains; ventures cluster in strong industrial bases |
| B8 | **Spec churn** | 10⁴ design projects with 0–3 `ChangeSpec`s at progress ∈ {0.2, 0.5, 0.8} | Cost and schedule growth vs. number and timing of changes | RND-037; report the growth premium per change pre- vs. post-freeze (teaching chart) |

B1 and B3 run first (they need no time loop). B2 then B4 set the AI defaults that B5 and B9 use.

## 5. Tuning procedure

1. Implement `truth.py` and `belief.py`; run B1 and the RND-011 check.
2. B3: refit G and S with the belief σ that `science.py` actually produces at project start (spec §4.5.3 note). GAO governs: mature 1.03 ± 0.03, immature 1.35 ± 0.05; then fit the overall CV to 0.80 ± 0.10; report the overall mean (expected ≈ 1.30) without targeting ACAT's 1.36.
3. B2: tune info rates (viability 1.2, perf 30, cost 15) and test precisions so RND-013 holds without overconfidence (RND-012).
4. B4: choose θ_kill.
5. B5: tune κ_d, τ_p, `e_ref` and B_slot so the pacing rows pass; then B6–B8.
6. Freeze; rerun everything with 3 new base seeds; every pass criterion must hold for all three. Write back to spec §4 and `RESULTS.md`.

## 6. Effort estimate [D]

About 35–45 hours at Sean's pace with AI help: content generator and truth 5 h, belief mixture 4 h, science, discovery and gates 6 h, projects and EVM 8 h, production 3 h, market 4 h, contractors 4 h, scenarios and report 6 h, tuning 4–6 h. B1, B2 and B3 are the three required showcase outputs and come first (≈ 19 h).

## 7. Risks

| Risk | Mitigation |
|---|---|
| Mixture updates on Python arrays are slow over 200 × 6 × 300 × 3 subjects × 1,260 ticks | Vectorize over all subjects; pooled quarterly emission means ≈ 1/3 of subjects update per month; expected minutes per batch |
| Cross-generation heuristic breaks calibration | RND-012 checks it; if it fails, weaken the LR (3 → 2) before anything else |
| GAO and ACAT anchors can't both be met | Decided (OQ-R3): GAO governs; ACAT mean reported only |
| Soft gates make early eras feel blocked | B4/B5 report gate-penalty spending and waiting time; tune s_c and the enabler date offset Δ before the class mix |
| Dead ends either never matter or dominate | B4 and B5 report dead-end spending share; adjust the class mix in content, not the mechanics |
| Synthetic content differs from the eventual authored graph | Keep the generator's counts and class mix equal to spec §6.3; rerun B1/B5 once the real content exists |
