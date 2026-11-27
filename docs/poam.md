# Plan of Action & Milestones (POAM)

Living document. Update the status and the next-actions table at the end of every working session.
Dates are left TBD on purpose: at about 10 hours a week with AI-assisted coding, the order of work matters more than the calendar. Add target dates once M2 gives us a velocity baseline.

**Status key:** ☐ not started · ◐ in progress · ☑ done · ⊘ blocked

## Milestones

| ID | Milestone | Exit criteria | WS | Status |
|---|---|---|---|---|
| M0 | Project setup | Repo scaffolded; ConOps baselined; spec template, grader agent, and kickoff prompts committed | WS-01, WS-10 | ☑ |
| M1 | Research & key ADRs | Research backlog (ConOps §12) closed; ADRs for the stack, content format, and setting licensing (OQ-1, OQ-2, OQ-5) | WS-01, WS-02, WS-08 | ☑ |
| M2 | Model prototypes | Python/MATLAB prototypes of: (a) fiscal + tax + bonds, (b) the R&D latent-truth/belief model, (c) faction/selectorate stability. Each is batch-run and shown stable over 1955–2060. | WS-03, WS-04, WS-05 | ◐ ((a) and (c) planned: `prototypes/economy/README.md`, `prototypes/governance/PLAN.md`) |
| M3 | Core specs approved | WS-02, 03, 04, and 05 specs with numbered requirements and acceptance criteria | WS-02–05 | ◐ (WS-02 approved v1.2; WS-03 v0.4 and WS-04 v0.4 drafts, not yet marked approved; WS-05 pending) |
| M4 | Sim-core skeleton | Tick loop, seeded RNG, content loading, breakdown/explain API, CLI batch runner. SYS-002, 003, 004, and 007 pass. | WS-02, WS-10 | ☐ |
| M5 | Core triad in Rust | Economy, governance, and R&D implemented to spec, with batch stability tests in CI | WS-03–05 | ☐ |
| M6 | Forces & thin stubs | Force design (templates, lifecycle, manpower, basing, munitions, strategic forces) per WS-11; diplomacy/AI, combat kernel, space graph, and setting data at MVP depth | WS-06–08, WS-11 | ☐ |
| M7 | UI thin client | Tauri client: budget/tax screen, R&D screen, faction screen, map (graph + 2525) | WS-09 | ☐ |
| M8 | Vertical slice | A full playthrough from 1955 to 2060 is possible as one nation-state (SYS-001) | All | ☐ |

## Next actions

| # | Action | Where | Status |
|---|---|---|---|
| 1 | Install the Claude GitHub App on the repo so cloud sessions can push | GitHub settings | ☑ |
| 2 | Review and approve ConOps | This Project | ☑ (v0.2) |
| 3 | Draft the workstream kickoff prompt template plus prompts for WS-02 through WS-05 | `prompts/` | ☑ |
| 4 | Resolve OQ-1, 2, 3, 5 | Project | ☑ (ADR-0001–0003) |
| 6 | Test whether the claude.ai Project can sync docs from the repo (OQ-4) | Sean | ☑ (2026-11-25; works for search, manual sync) |
| 7 | Start WS-08 setting/canon chat: 1955 back-history, expanded DATE polity list | New Project chat | ☑ (WS-08a spec, canon and draft content; PR open) |
| 5 | Run M1 research | `docs/research/m1-findings.md` | ☑ |
| 8 | Start WS-02 design chat with `prompts/WS-02.md` | New Project chat | ☑ (spec + ADR-0004–0009 drafted, PR open) |
| 10 | Review WS-02 spec and ADR-0004–0009; answer OQ-A1–A6; approve or request changes | Sean | ☑ (approved 2026-11-25, PR #3) |
| 11 | After WS-02 approval: write the WS-02 Phase B prompt; start M4 (the ConOps §7 edit for OQ-A5 is already in PR #3) | Project → repo | ☐ |
| 9 | Describe the target TNO economy "feel" at the WS-03 kickoff | Sean | ☑ (2026-11-26; spec §0) |
| 12 | Resolve OQ-6 in the WS-02 review: add a `force` block (owner WS-11), its stage, and the combat-loss path; add force units to the ARC-090 bench scenario | WS-02 PR | ☑ (R5b/R6b, ARC-024, OQ-A6) |
| 13 | Start the WS-11 design chat with `prompts/WS-11.md` once the WS-05 spec is approved | New Project chat | ☐ |
| 14 | Review WS-03 spec draft; answer OQ-E1–E6 (incl. the ConOps edits proposed in OQ-E5); approve or request changes | Sean | ☐ |
| 15 | Build the M2(a) economy prototype per `prototypes/economy/README.md` (≈ 38 h, P1–P5) | `prototypes/economy/` | ☐ |
| 16 | Start the WS-04 design chat; it must conform to the WS-03 §2.4 contract (`EconKey`, `EconOutcomes`, `FactionEconBehavior`) | New Project chat | ☑ (WS-04 v0.2 conforms; PR #8) |
| 14 | Review the WS-08a spec, canon and draft content; answer OQ-W1–W6 and OQ-C1–C3; approve or request changes | Sean | ◐ (OQ-W1/W2/W4/W6, OQ-C2 accepted 2026-11-27; approval pending) |
| 15 | If OQ-W1 and OQ-W2 are accepted: edit WS-02 (ARC-090 bench to 47 polities and 132 regions; optional `RegionEdge.chokepoint` in §6) | Project → repo | ☑ (WS-02 v1.1, ConOps v0.4) |
| 16 | Verify the [R]-tagged DATE facts against ODIN (OQ-W3/OQ-C1): Sean saves the Internet Archive ODIN pages (PDF/HTML) and attaches them; Claude's web tools can't fetch the archive | Sean → Claude | ☐ |
| 17 | M4: port `prototypes/ws08a-content-check` types and `wld_*` tests into `sim-core`, then delete the prototype | Code session | ☐ |
| 18 | Start WS-08b (Solar System and resources) with `prompts/WS-08b.md` | New Project chat | ☑ (WS-08b spec v0.4, draft Sol and resource content, Sol-graph prototype; new PR after the #14 revert) |
| 19 | Review WS-04 v0.2 (8 axes, 9 institutions, services split, WS-03 §2.4 conformance); answer OQ-G7–G9; approve or request changes | Sean (PR #8) | ◐ (OQ-G7–G9 accepted 2026-11-27; approval pending) |
| 20 | Review WS-03 v0.3 (household panel, five-group wealth dynamics, new housing/pension instruments, CBI ownership, WS-08a band mapping); OQ-E7/E8 | Sean (PR #13) | ☐ |
| 21 | WS-04: add `policy_map.ron` rows and channels for the four new WS-03 instruments (LTV cap, mortgage deductibility, pension funded share, social housing) so GOV-021 validation passes; optionally use the distribution summary's measured vs. true shares | WS-04 chat | ☑ (WS-04 v0.3 §6.4 default policy map, PR #15; measured-inequality use deferred as OQ-G10) |
| 20 | WS-03 v0.2: make the OQ-G7 edits (`wealth` in `KeyOutcome`; CBI owned by WS-04 gov type; rents never key income) | WS-03 → repo | ☑ (wealth, CBI ownership and rents note all done in WS-03 v0.2/v0.3) |
| 22 | Build the M2(c) governance prototype per `prototypes/governance/PLAN.md` (S8 → S1 → S2/S3 → S9); write tuned values back to WS-04 §4 | Claude Code session | ☐ |
| 23 | Review and approve WS-08b v0.4 (all four workstreams' feedback applied; §8.1 lists what remains) | Sean | ☐ |
| 24 | After WS-08b merges: WS-02 applies P-02a–g (v1.2); WS-08a drops `Region.resource_tags`, rewords WLD-047, adds WLD-017 and owns `deposit_start.ron`; WS-03 publishes `extraction_share` (WS-04 OQ-G11) | WS-02, WS-08a, WS-03 | ◐ (WS-08a part done: v0.3, PR #20; WS-02 part done: v1.2, PR ws02/v1.2-ws08b; WS-03 `extraction_share` open) |
| 25 | Build M2(d), the rent-hook prototype (`prototypes/ws08b-sol/README.md` §2, ≈ 14 h), after M2(a) P1–P3; propose `deposit_start.ron` values for WS-08a review | `prototypes/economy/` | ☐ |

## Change log

| Date | Change |
|---|---|
| 2026-11-22 | Initial scaffold, ConOps v0.1 |
| 2026-11-22 | ConOps v0.2 baselined; ADR-0001 (RON), ADR-0002 (Tauri), ADR-0003 (DATE + disclaimer) |
| 2026-11-22 | M1 research findings; prompt template + WS-02–05 kickoff prompts; M0 closed |
| 2026-11-22 | WS-02 architecture spec (Draft) and ADR-0004–0009 (Proposed); M3 in progress. M1 stays ☑ with OQ-4 pending |
| 2026-11-24 | ConOps v0.3: force design added (scope rows, P-10, §6.1 echelons, R-6, SYS-009, OQ-6); WS-11 created; WS-11 kickoff prompt |
| 2026-11-25 | WS-02 v0.2: OQ-A1–A6 closed (same-platform determinism; user-set history ring/autosave {1–24}, 3 autosaves kept; reference machine Ryzen 7 5800H; debt/treasury deferred to WS-03; `Lazy` unit-level breakdowns). Added ARC-044–046 and `explain_change`; ARC-021/024/031/042/043/062/090 revised; ADR-0006/0007/0009 revised |
| 2026-11-25 | WS-02 spec approved v1.0; ADR-0004–0009 accepted |
| 2026-11-25 | OQ-4 closed: Project GitHub sync works for search (manual "Sync now"; files not openable by name); POAM item 6 done |
| 2026-11-27 | WS-08a spec (Draft v0.1, WLD-001–049), canon v0.1, and draft Earth map (110 land + 22 sea regions, 366 edges) and DATE setting (47 polities) content; prototype content checker |
| 2026-11-27 | WS-08a grader fixes: one primary owner per start field (WLD-022); no cascading errors (ARC-051); data-driven disclaimer headers; region IDs and names made physical (23 renamed); 1945–55 back-history all [ORIG], DATE facts F-1..F-8 kept separate; 52 passing tests |
| 2026-11-26 | Salvo kernel research note `docs/research/salvo-model.md` (feeds WS-07, WS-11): framework F-1–F-8, gaps, munition lots option C, batteries as brigade templates; ConOps T-1/T-2 edits proposed, not made |
| 2026-11-26 | WS-03 economy spec Draft v0.1 (ECO-001–133) and M2(a) prototype plan; closes WS-02 OQ-A4 (adds `gov_debt`, `treasury` to `Aggregates`); ConOps edits proposed in OQ-E5, not made |
| 2026-11-27 | OQ-W1/W2/W4/W6 and OQ-C2 accepted: ConOps v0.4 (~20–45 macro polities), WS-02 v1.1 (ARC-090 bench 47 polities/132 regions; optional `RegionEdge.chokepoint`). Low-fidelity 1955 map `docs/setting/date/map-1955.svg` plus renderer; Kalimantan centroid moved onto land |
| 2026-11-27 | WS-04 governance spec Draft v0.2 and M2(c) prototype plan (PR #8). v0.1 (GOV-001–098) revised per Sean: 8 TI-inspired axes (Economy, Welfare, Knowledge, Government, Oppression, Military, Nuclear, Space) with 2–3 salient per faction; Unity as a program; 9 institutions (control points) with archetype weights replacing veto players and strength weights; Military + Navy/Aerospace split by size; conforms to WS-03 §2.4 (rents as private goods); presets for all 8 WS-08a `Government` variants. OQ-G1–G6 closed; WS-03 edits requested as OQ-G7 |
| 2026-11-27 | WS-03 amended (OQ-G7): `KeyOutcome.wealth` with a key-wealth rule (§3.5), Transfer/Public keys sectorless (≤ 240 keys), rents never key income; ECO-090 acceptance extended. OQ-G8 closed, OQ-G9 accepted |
| 2026-11-27 | WS-03 v0.2: household panel for income/wealth (calibrated to Fed DFA, WIR 2022, SCF 2022; `prototypes/economy/calib_wealth.py`), ECO-048/049 added, ECO-040/042/090 revised; CBI read only from WS-04 (closes WS-04 OQ-G7); WS-08a fields mapped (§5.1, WLD-022); OQ-E3 closed |
| 2026-11-27 | WS-03 v0.3: five-group wealth stock-flow (saving, returns, asset prices, taxes, estates, mobility) with measured/true/augmented concepts; mortgage, pension-funding and social-housing instruments; 8,192 oversampled panel (`panel_size.py`); ECO-140–147 (US/UK paths, NL vs. BE) |
| 2026-11-27 | WS-04 v0.3 (WS-03 v0.3 sync): §6.4 default policy map covering every WS-03/WS-04 instrument, including the LTV cap (Economy), mortgage-interest deductibility (Welfare −), pension funded share (Economy −) and social housing (Welfare +); wealth concept for oligarch control stated (true, offshore included); OQ-G10 added; prototype S5 to use WS-03 wealth dynamics once M2(a) exists |
| 2026-11-27 | WS-04 v0.4 (WS-08b P-04a–e): royalty, `resource_development` and `exploration` rows in §6.4 (exploration channel `none`, not `public`); `gate_orders` covers `Survey`/`DevelopDeposit` (PC 0); Landowners/Industrialists split agriculture-and-extraction owners by `extraction_share`; diverted rents labeled; OQ-G8 answer recorded; OQ-G11 asks WS-03/WS-08b for the rents split and `extraction_share` |
| 2026-11-27 | WS-03 v0.4: WS-08b P-03a–k applied; P-03j = separate resource-equity line (payout, not r_E); P-03f = separate non-commodity world factor f_w, λ_w 0.3 (corr 0.3 with f_g); archetype D, S22/S23 reserved for M2(d); S11 moved to M2(d); ECO-023, ECO-150–154 |
| 2026-11-27 | WS-08b v0.1–v0.3 (PR #14, merged then reverted so WS-03 v0.4 could land): seeded Sol (32 canonical bodies + ≤ 18 generated from real small-body populations), regime-node Δv graph with synodic windows, four-kind resource taxonomy, commodity price block, rent hook, prospecting, thin space ops; OQ-S1–S5 closed |
| 2026-11-27 | WS-08b v0.4 (new PR, rebuilt on main): marks P-03a–k and P-04a–e applied; `f_w` wording (λ_w 0.3); exploration channel `none`; extraction value added for `extraction_share` (WLD-058, WS-04 OQ-G11); WS-02 v1.2 items (delivery lag, per-regime `orbital_assets`, ≈ 0.18 MB snapshot); deposit IDs from tag text (SYS-007); `deposit_start.ron` header rule; OQ-S6 foreign concessions (non-blocking, default none) |
| 2026-11-27 | WS-08a v0.3 (WS-08b P-08a1/2): `Region.resource_tags` removed (WLD-047); new WLD-017 validates `settings/<s>/deposit_start.ron` against WS-08b deposit IDs; 58 passing tests. `deposit_start.ron` values wait on WS-08b M2(d) |
| 2026-11-27 | WS-02 v1.2: WS-08b P-02a–g applied (`Sol` node graph, `resources`/`space` blocks, WS-08 in R2/R5b/R6b, `Owner::{Deposit, Facility, Node}`, observation units in ADR-0008, test-setting bodies, `Aggregates.orbital_assets`); ARC-013/015/080/081/090 and §5 memory updated |
