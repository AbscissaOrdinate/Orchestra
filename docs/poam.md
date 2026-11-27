# Plan of Action & Milestones (POAM)

Living document. Update the status and the next-actions table at the end of every working session.
Dates are left TBD on purpose: at about 10 hours a week with AI-assisted coding, the order of work matters more than the calendar. Add target dates once M2 gives us a velocity baseline.

**Status key:** ☐ not started · ◐ in progress · ☑ done · ⊘ blocked

## Milestones

| ID | Milestone | Exit criteria | WS | Status |
|---|---|---|---|---|
| M0 | Project setup | Repo scaffolded; ConOps baselined; spec template, grader agent, and kickoff prompts committed | WS-01, WS-10 | ☑ |
| M1 | Research & key ADRs | Research backlog (ConOps §12) closed; ADRs for the stack, content format, and setting licensing (OQ-1, OQ-2, OQ-5) | WS-01, WS-02, WS-08 | ☑ |
| M2 | Model prototypes | Python/MATLAB prototypes of: (a) fiscal + tax + bonds, (b) the R&D latent-truth/belief model, (c) faction/selectorate stability. Each is batch-run and shown stable over 1955–2060. | WS-03, WS-04, WS-05 | ☐ |
| M3 | Core specs approved | WS-02, 03, 04, and 05 specs with numbered requirements and acceptance criteria | WS-02–05 | ◐ (WS-02 approved; WS-03 draft in PR; WS-04–05 pending; WS-08a/08b drafts in PR) |
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
| 15 | Build the M2(a) economy prototype per `prototypes/economy/README.md` (≈ 32 h, P1–P5) | `prototypes/economy/` | ☐ |
| 16 | Start the WS-04 design chat; it must conform to the WS-03 §2.4 contract (`EconKey`, `EconOutcomes`, `FactionEconBehavior`) | New Project chat | ☐ |
| 14 | Review the WS-08a spec, canon and draft content; answer OQ-W1–W6 and OQ-C1–C3; approve or request changes | Sean | ◐ (OQ-W1/W2/W4/W6, OQ-C2 accepted 2026-11-27; approval pending) |
| 15 | If OQ-W1 and OQ-W2 are accepted: edit WS-02 (ARC-090 bench to 47 polities and 132 regions; optional `RegionEdge.chokepoint` in §6) | Project → repo | ☑ (WS-02 v1.1, ConOps v0.4) |
| 16 | Verify the [R]-tagged DATE facts against ODIN (OQ-W3/OQ-C1): Sean saves the Internet Archive ODIN pages (PDF/HTML) and attaches them; Claude's web tools can't fetch the archive | Sean → Claude | ☐ |
| 17 | M4: port `prototypes/ws08a-content-check` types and `wld_*` tests into `sim-core`, then delete the prototype | Code session | ☐ |
| 18 | Start WS-08b (Solar System and resources) with `prompts/WS-08b.md` | New Project chat | ☑ (WS-08b spec Draft v0.1, draft Sol and resource content, Sol-graph prototype; PR open) |
| 19 | Review the WS-08b spec; answer OQ-S1–S5; accept or reject the proposed edits P-02a–P-CON (§8); approve or request changes | Sean | ◐ (OQ-S1–S5 accepted 2026-11-27; real-population small bodies and canonical minor bodies added in v0.2; §8 edits and approval pending) |
| 20 | After WS-08b approval: apply the accepted §8 edits to WS-02, WS-03, WS-04 (PR #8), WS-08a and the salvo note; carry the WS-05/06/07/11 items into their kickoffs | Project → repo | ☐ |
| 21 | Build M2(d), the rent-hook prototype (`prototypes/ws08b-sol/README.md` §2, ≈ 14 h), after M2(a) P1–P3 | `prototypes/economy/` | ☐ |

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
| 2026-11-27 | WS-08b spec Draft v0.1 (WLD-050–099): seeded Sol (28 template bodies + ≤ 16 generated), regime-node Δv graph, synodic windows, four-kind resource taxonomy, world commodity price block, rent hook into WS-03, prospecting, thin space ops. Draft content in `content/maps/sol/`, `content/maps/earth/resources.ron`, `content/core/resources/`; `prototypes/ws08b-sol/sol.py` checks (Mars windows match 1960–75 launches). Edits to other docs proposed (§8), not made |
| 2026-11-27 | WS-08b v0.2: OQ-S1–S5 closed. Generated small bodies follow real populations (vulcanoids, NEA groups, Mars/Jupiter trojans, belt composition gradient, Hildas); canonical Juno, Hygiea, Pluto, Charon added (32 template bodies, ≤ 50 total) |
| 2026-11-27 | OQ-W1/W2/W4/W6 and OQ-C2 accepted: ConOps v0.4 (~20–45 macro polities), WS-02 v1.1 (ARC-090 bench 47 polities/132 regions; optional `RegionEdge.chokepoint`). Low-fidelity 1955 map `docs/setting/date/map-1955.svg` plus renderer; Kalimantan centroid moved onto land |
