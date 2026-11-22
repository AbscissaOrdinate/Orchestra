# Plan of Action & Milestones (POAM)

Living document. Update the status and the next-actions table at the end of every working session.
Dates are left TBD on purpose: at about 10 hours a week with AI-assisted coding, the order of work matters more than the calendar. Add target dates once M2 gives us a velocity baseline.

**Status key:** ☐ not started · ◐ in progress · ☑ done · ⊘ blocked

## Milestones

| ID | Milestone | Exit criteria | WS | Status |
|---|---|---|---|---|
| M0 | Project setup | Repo scaffolded; ConOps baselined; spec template, grader agent, and kickoff prompts committed | WS-01, WS-10 | ◐ |
| M1 | Research & key ADRs | Research backlog (ConOps §12) closed; ADRs for the stack, content format, and setting licensing (OQ-1, OQ-2, OQ-5) | WS-01, WS-02, WS-08 | ☐ |
| M2 | Model prototypes | Python/MATLAB prototypes of: (a) fiscal + tax + bonds, (b) the R&D latent-truth/belief model, (c) faction/selectorate stability. Each is batch-run and shown stable over 1955–2060. | WS-03, WS-04, WS-05 | ☐ |
| M3 | Core specs approved | WS-02, 03, 04, and 05 specs with numbered requirements and acceptance criteria | WS-02–05 | ☐ |
| M4 | Sim-core skeleton | Tick loop, seeded RNG, content loading, breakdown/explain API, CLI batch runner. SYS-002, 003, 004, and 007 pass. | WS-02, WS-10 | ☐ |
| M5 | Core triad in Rust | Economy, governance, and R&D implemented to spec, with batch stability tests in CI | WS-03–05 | ☐ |
| M6 | Thin stubs | Diplomacy/AI, combat kernel, space graph, and setting data at MVP depth | WS-06–08 | ☐ |
| M7 | UI thin client | Tauri client: budget/tax screen, R&D screen, faction screen, map (graph + 2525) | WS-09 | ☐ |
| M8 | Vertical slice | A full playthrough from 1955 to 2060 is possible as one nation-state (SYS-001) | All | ☐ |

## Next actions

| # | Action | Where | Status |
|---|---|---|---|
| 1 | Install the Claude GitHub App on the repo so cloud sessions can push | GitHub settings | ☐ |
| 2 | Review and approve ConOps v0.1 | This Project | ☐ |
| 3 | Draft the workstream kickoff prompt template plus prompts for WS-02 through WS-05 | Project chat → `prompts/` | ☐ |
| 4 | Resolve OQ-1 (DATE distribution status) | Research | ☐ |
| 5 | Run M1 research | Project chat | ☐ |

## Change log

| Date | Change |
|---|---|
| 2026-11-22 | Initial scaffold, ConOps v0.1 |
