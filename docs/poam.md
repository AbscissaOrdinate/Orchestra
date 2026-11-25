# Plan of Action & Milestones (POAM)

Living document. Update the status and the next-actions table at the end of every working session.
Dates are left TBD on purpose: at about 10 hours a week with AI-assisted coding, the order of work matters more than the calendar. Add target dates once M2 gives us a velocity baseline.

**Status key:** ☐ not started · ◐ in progress · ☑ done · ⊘ blocked

## Milestones

| ID | Milestone | Exit criteria | WS | Status |
|---|---|---|---|---|
| M0 | Project setup | Repo scaffolded; ConOps baselined; spec template, grader agent, and kickoff prompts committed | WS-01, WS-10 | ☑ |
| M1 | Research & key ADRs | Research backlog (ConOps §12) closed; ADRs for the stack, content format, and setting licensing (OQ-1, OQ-2, OQ-5) | WS-01, WS-02, WS-08 | ☑ (OQ-4 pending Sean's test) |
| M2 | Model prototypes | Python/MATLAB prototypes of: (a) fiscal + tax + bonds, (b) the R&D latent-truth/belief model, (c) faction/selectorate stability. Each is batch-run and shown stable over 1955–2060. | WS-03, WS-04, WS-05 | ☐ |
| M3 | Core specs approved | WS-02, 03, 04, and 05 specs with numbered requirements and acceptance criteria | WS-02–05 | ◐ (WS-02 drafted, in review) |
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
| 6 | Test whether the claude.ai Project can sync docs from the repo (OQ-4) | Sean | ☐ |
| 7 | Start WS-08 setting/canon chat: 1955 back-history, expanded DATE polity list | New Project chat | ☐ |
| 5 | Run M1 research | `docs/research/m1-findings.md` | ☑ |
| 8 | Start WS-02 design chat with `prompts/WS-02.md` | New Project chat | ☑ (spec + ADR-0004–0009 drafted, PR open) |
| 10 | Review WS-02 spec and ADR-0004–0009; answer OQ-A1–A5; approve or request changes | Sean | ☐ |
| 11 | After WS-02 approval: write the WS-02 Phase B prompt; start M4 (the ConOps §7 edit for OQ-A5 is already in PR #3) | Project → repo | ☐ |
| 9 | Describe the target TNO economy "feel" at the WS-03 kickoff | Sean | ☐ |
| 12 | Resolve OQ-6 in the WS-02 review: add a `force` block (owner WS-11), its stage, and the combat-loss path; add force units to the ARC-090 bench scenario | WS-02 PR | ☑ (R5b/R6b, ARC-024, OQ-A6) |
| 13 | Start the WS-11 design chat with `prompts/WS-11.md` once the WS-05 spec is approved | New Project chat | ☐ |

## Change log

| Date | Change |
|---|---|
| 2026-11-22 | Initial scaffold, ConOps v0.1 |
| 2026-11-22 | ConOps v0.2 baselined; ADR-0001 (RON), ADR-0002 (Tauri), ADR-0003 (DATE + disclaimer) |
| 2026-11-22 | M1 research findings; prompt template + WS-02–05 kickoff prompts; M0 closed |
| 2026-11-22 | WS-02 architecture spec (Draft) and ADR-0004–0009 (Proposed); M3 in progress. M1 stays ☑ with OQ-4 pending |
| 2026-11-24 | ConOps v0.3: force design added (scope rows, P-10, §6.1 echelons, R-6, SYS-009, OQ-6); WS-11 created; WS-11 kickoff prompt |
