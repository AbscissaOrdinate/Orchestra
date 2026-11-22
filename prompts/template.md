# Workstream prompt template

Each workstream runs through three phases, each with its own prompt. Fill in the `<…>` placeholders.

---

## Phase A — Design kickoff (claude.ai Project chat)

```
Workstream <WS-NN — Name> for Orchestra (repo AbscissaOrdinate/Orchestra).

Read first: docs/conops.md, docs/poam.md, docs/research/m1-findings.md, docs/specs/README.md (template), <other specs/ADRs this depends on>.

Objective: produce docs/specs/<WS-NN-name>.md, an approved spec with numbered <PREFIX>- requirements, each with a testable acceptance criterion and a trace to SYS-/P- IDs.

MVP scope (from ConOps §3/§6): <in-scope bullets>
Out of scope: <deferred bullets>
Interfaces with other workstreams: <inputs/outputs to pin down>

Decisions to make in this session:
1. <decision>
2. <decision>

Working method:
- Before writing the spec, propose model options with their trade-offs (realism, legibility, compute cost, implementation effort), then recommend one.
- Ask me at most 5 questions, and only where a wrong guess would be costly. Otherwise assume, and state the assumption.
- Flag every parameter value as [V] sourced, [R] recalled, or [D] design choice.
- Keep it implementable by a solo dev with AI help: thin where the ConOps says thin.

Deliverables: the spec file committed on a branch with a PR; a prototype plan if the ConOps P-9 applies; POAM updated.
Done when: every requirement is testable, interfaces are agreed with the dependent workstreams, and open questions are listed.
```

## Phase B — Implementation (Claude Code session on the repo)

```
Implement <WS-NN> requirements <ID list or "all Approved"> from docs/specs/<WS-NN-name>.md.

Follow CLAUDE.md. Before coding:
1. Read the spec, ConOps §5 invariants, and the relevant ADRs.
2. Post a short plan: files to touch, the test for each requirement, any spec ambiguity. If there is an ambiguity, stop and ask. Don't guess.

Rules: tests named after requirement IDs; determinism and breakdown invariants respected; no scope beyond the listed IDs; branch + PR citing the IDs.
When done: run cargo fmt, clippy, and test; run the spec-grader agent on the diff; fix every FAIL; update docs/poam.md; report the grader table.
```

## Phase C — Grade and iterate (same or a new Claude Code session)

```
Run the spec-grader agent on branch <branch> against docs/specs/<WS-NN-name>.md, requirement IDs <list>. Return its table and its follow-on prompt verbatim.
```

Paste the grader's follow-on prompt into the implementation session. Repeat until everything passes, then merge.
