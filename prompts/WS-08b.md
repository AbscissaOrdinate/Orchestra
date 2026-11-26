# WS-08b — Solar System, Resources & Space Operations · Phase A kickoff

Prerequisites: WS-02 approved ✓ (`sol` graph, `Truth`/`worldgen_stream`, beliefs, ARC-011/013/034). Soft dependencies: WS-03 (how resources enter the economy) and WS-05 (propulsion tech that sets usable delta-v). Define those interfaces here as provisional, and have WS-03 and WS-05 confirm them.

```
Workstream WS-08b — Solar System, Resources & Space Operations for Orchestra (repo AbscissaOrdinate/Orchestra).

Read first: docs/conops.md (§3, §4, §5 P-1/P-2, §6 space row), docs/specs/WS-02-architecture.md (§3.2 truth/beliefs, §3.4 RNG/worldgen_stream, §6 content), docs/research/m1-findings.md, docs/specs/README.md.

Objective: produce docs/specs/WS-08b-space.md (WLD- requirements, numbered WLD-050..099) and a Python prototype plan if world generation needs tuning.

Scope (MVP):
- Seeded Solar System variant: the real bodies as a template, plus seeded hypothetical moons and minor bodies with plausible orbits; all sampled into Truth with worldgen_stream (P-1, ARC-034)
- Delta-v graph: Hohmann-transfer edges between nodes (surface, orbit, Lagrange points as needed); synodic launch windows; transfer times
- Resources: deposit truth per body (type, grade, size, extraction difficulty) drawn from distributions; Earth resources by region tag (from WS-08a)
- Prospecting: surveys and missions produce Observations that narrow beliefs about deposits (P-2); the cost and fidelity of survey types
- Space operations (thin): missions, outposts, and stations as entities; launch capacity from bases (WS-11); logistics as delta-v × mass, abstract
- Interfaces: resources → WS-03 production inputs (provisional); propulsion/vehicle tech → usable delta-v (WS-05, provisional); space bases and forces (WS-11); orbital engagements (WS-07)
- Macro-tier reduced form for space activity

Out of scope: n-body or patched-conic trajectories, continuous orbits, detailed station design.

Decisions to make:
1. Node granularity (per body: surface/LEO/high orbit?) and graph size within the WS-02 performance budget
2. Hypothetical-body generation rules (how many, where, which constraints keep it plausible)
3. Resource taxonomy (keep it small) and the deposit distributions
4. How launch windows constrain missions with monthly ticks
5. The minimum outpost/station model that supports space industrialization later

Working method: options → trade-offs → recommendation; at most 5 questions; tag values [V]/[R]/[D]; cite sources for physical constants and real-body data.
Deliverables: spec on a branch with a PR; draft content/maps/sol/ RON; POAM updated.
Done when: every WLD- requirement is testable (e.g., the same seed gives an identical Sol variant; a delta-v shortest path matches a hand calculation), and the provisional interfaces are listed for WS-03, WS-05, WS-07 and WS-11.
```
