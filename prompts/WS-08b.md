# WS-08b — Solar System, Resources & Space Operations · Phase A kickoff

Prerequisites: WS-02 v1.1 ✓, WS-03 ✓ (merged), WS-08a ✓ (merged). WS-04 (PR #8) is not blocking: only its Space policy axis touches this workstream.

**What WS-03 settled, and what it left to WS-08b.** WS-03 chose five sectors. Agriculture & extraction is one sector. Energy is not a sector: it is a global commodity cost shock (`f_g`, WS-03 §3.3) plus a **resource-rent hook from WS-08** (WS-03 §3.1). WS-03 did **not** define a resource taxonomy, and WS-08a reserved `resource_tags` on regions with no vocabulary yet. WS-08b therefore owns the resource model, on Earth and off it, and has to design the rent hook against WS-03's existing interfaces.

```
Workstream WS-08b — Solar System, Resources & Space Operations for Orchestra (repo AbscissaOrdinate/Orchestra).

Read first:
- docs/conops.md (§3, §4, §5 P-1/P-2/P-10, §6 space row)
- docs/specs/WS-02-architecture.md (§3.2 truth/beliefs, §3.4 RNG/worldgen_stream, §3.3 stages, §6 content)
- docs/specs/WS-03-economy.md (§2.2–§2.3 interfaces, §3.1 sectors and the WS-08 rent hook, §3.3 global commodity factor f_g, §3.10 trade stub, §3.13 macro tier)
- docs/specs/WS-08a-setting-map.md (§3.2 reserved region attributes, WLD-047; numbering ends at WLD-049)
- docs/research/salvo-model.md (F-3 engagement generator inputs, target_class `space` + orbit_regime, spacecraft magazines and resupply)
- PR #8 docs/specs/WS-04-governance.md, the Space axis (instrument: space program/GDP) and the `rents` channel
- docs/specs/README.md

Objective: produce docs/specs/WS-08b-space.md (WLD-050..099), plus a Python prototype plan if world generation or the rent hook needs tuning.

Scope (MVP):
- Seeded Solar System variant: real bodies as the template, plus seeded hypothetical moons and minor bodies with plausible orbits; all sampled into Truth via worldgen_stream (P-1)
- Delta-v graph: Hohmann-transfer edges between nodes (surface, orbit regimes, Lagrange points as needed); synodic launch windows at monthly ticks; transfer times
- Orbit regimes: one enum shared with the salvo model's `orbit_regime`
- Resource model (owned here):
  - a small taxonomy covering Earth and off-Earth resources
  - the vocabulary for WS-08a `resource_tags`
  - deposit truth (type, grade, size, extraction difficulty) drawn from distributions
- Rent hook into WS-03 (design it; WS-03 confirms):
  - how deposits enter the agriculture & extraction sector (a capacity or TFP multiplier, or a natural-resource factor)
  - how resource rents flow to the treasury (royalties/SOE) and to WS-04 as a `rents`-channel instrument
  - how exporters and importers load differently on the global commodity factor f_g (resource curse, 1973-style shocks)
- Prospecting: surveys and missions emit Observations that narrow deposit beliefs (P-2); survey types trade cost against fidelity
- Space operations (thin):
  - missions, outposts and stations as entities
  - launch capacity from WS-11 bases, limited by WS-03 space-industry sector capacity
  - logistics as abstract delta-v × mass
  - the spacecraft resupply rule the salvo model defers to WS-08
- Engagement-generator inputs for WS-07: node adjacency, windows, co-location rules for orbital engagements
- Macro-tier reduced form for space activity and resource endowment

Out of scope: n-body or patched-conic trajectories; detailed station design; an environment or conservation model (future WS-04 axis hook only); bilateral commodity trade beyond WS-03's trade stub.

Decisions to make:
1. Node granularity per body (surface, LEO, high orbit, ...) and graph size within the WS-02 performance budget
2. Hypothetical-body generation rules (how many, where, which constraints keep them plausible)
3. The resource taxonomy (keep it small; every type must drive a mechanic) and the deposit distributions
4. The rent hook's functional form, and its effect on WS-03 stability criteria (WS-03 §6.1 must still pass)
5. How launch windows constrain missions with monthly ticks, and the minimum outpost/station model

Working method: options → trade-offs → recommendation; at most 5 questions; tag values [V]/[R]/[D]; cite sources for physical constants and real-body data. Propose (don't make) any edits to WS-03, WS-08a, WS-04 or the salvo note: for example, WS-08a §3.2 currently attributes the `resource_tags` vocabulary to WS-03.
Deliverables: spec on a branch with a PR; draft content/maps/sol/ and resource RON; POAM updated.
Done when: every WLD- requirement is testable (e.g., the same seed gives an identical Sol variant; a delta-v shortest path matches a hand calculation; the rent hook reproduces a positive f_g windfall for exporters); interface proposals are listed for WS-03, WS-04, WS-05, WS-07 and WS-11.
```
