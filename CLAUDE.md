# Orchestra — instructions for Claude sessions

Orchestra is a turn-based space-age grand strategy sandbox: a Rust simulation core with a Tauri/web UI.
Start every session by reading `docs/conops.md` (design baseline) and `docs/poam.md` (current status and next actions).

## Sources of truth (highest first)
1. `docs/conops.md`: vision, scope, principles, `SYS-` requirements
2. `docs/specs/WS-NN-*.md`: workstream requirements (`ECO-`, `GOV-`, `RND-`, etc.) with acceptance criteria
3. `docs/adr/`: architecture decisions
4. Code

If code and spec disagree, the spec wins. If a spec is wrong or ambiguous, **do not silently change it**. Stop, explain the problem, and propose an edit. Design changes are made in the claude.ai Project, then committed here.

## Workflow
- Spec-first. Implement only against approved requirement IDs, and cite those IDs in commit messages and PR descriptions.
- Every requirement you implement gets a test that maps to its acceptance criterion. Name tests after the requirement (e.g. `eco_014_bond_spread_at_high_debt`).
- After implementing, run the `spec-grader` agent and address every FAIL before declaring the work done.
- Update `docs/poam.md` (status, next actions, change log) at the end of each session.
- Work on a branch and open a PR. Never commit directly to `main`.

## Simulation invariants (ConOps §5)
- **Determinism:** all randomness goes through the sim's seeded RNG. No wall-clock time, no thread-order dependence, and no `HashMap` iteration in sim logic (use `BTreeMap`/`IndexMap` or sorted vectors).
- **Explainability:** every state variable that is updated produces a contribution breakdown.
- **Per-world truth, per-polity belief:** never let one polity's sampling change world truth.
- **Data-driven content:** no hard-coded polities, techs, components, or names in code.
- The core stays headless: no UI dependencies in `crates/sim-core`.

## Coding conventions
- Lazy senior developer: before writing code, check whether it's needed at all, then the standard library, then an existing dependency. Write the minimum code that works.
- Reusable, general functions; vectorize or iterate over slices rather than per-element special cases.
- Simple, intuitive names; readable code for a reviewer, not clever code.
- Rust: stable toolchain, no `unsafe`, `clippy` clean, `rustfmt`.
- Prototypes (`prototypes/`) may be Python or MATLAB. Put helper functions at the bottom of each script and vectorize with NumPy.

## Layout
```
docs/conops.md  docs/poam.md  docs/specs/  docs/adr/
prompts/        kickoff prompts per workstream
prototypes/     model sketches (Python/MATLAB)
crates/         sim-core, cli (created in M4)
app/            Tauri client (created in M7)
content/        data files (created in M4)
.claude/agents/spec-grader.md
```
