# WS-08a — Setting, Canon & Earth Map · Phase A kickoff

Prerequisites: WS-02 approved ✓ (content layout, setting/map separation, ARC-052). No other hard dependencies. Where start values depend on unwritten specs (WS-03 economy, WS-04 government, WS-11 forces), define the field names and qualitative targets, and leave the numbers for those workstreams.

```
Workstream WS-08a — Setting, Canon & Earth Map for Orchestra (repo AbscissaOrdinate/Orchestra).

Read first: docs/conops.md (§3, §4, SYS-007), docs/adr/0003-date-setting.md, docs/specs/WS-02-architecture.md (§3.6 content loading, §6 content/data, ARC-050..052), docs/specs/README.md.

Objective: produce docs/specs/WS-08a-setting-map.md (WLD- requirements, numbered WLD-001..049), plus a canon document docs/setting/date/canon.md. The game's default setting is an expanded DATE world on real Earth geography, 1955–2060.

Scope (MVP):
- Polity roster: the DATE polities plus the expansion needed for an interesting board (propose a count of ~20–40 polities with real-world analog roles; separate high-fidelity candidates from macro-tier polities)
- Back-history and canon: 1945–1955 to explain the start state; tag each item as DATE-adapted or Orchestra-original (ADR-0003 provenance rule)
- Start state per polity: government archetype, economic size and development, alignment and alliances, start tech level, rough force posture. Qualitative or ranged values now; WS-03, WS-04 and WS-11 set the numbers.
- Earth region graph: ~100 regions on real geography; terrain classes; edge kinds (land, strait, sea lane, air); region attributes needed by other workstreams (population share, resource tags, port/airfield suitability), as names only until those specs land
- Ownership map at the 1955 start; contested and colonial regions
- Extensibility check: adding, renaming or replacing a polity, or swapping the whole setting, with data only (SYS-007)
- Non-endorsement disclaimer placement (README, credits, any content file headers)

Out of scope: Solar System and resources (WS-08b); scripted event systems (maybe a short list of era-defining event hooks for later); polygon map art.

Decisions to make:
1. Polity count and which ones are high-fidelity candidates
2. How DATE's near-present framing is projected back to 1955 (decolonization, bloc formation, the nuclear era)
3. Region granularity: where to spend regions (contested zones) and where to merge (oceans, deserts)
4. Naming and lore conventions, and how adapted vs. original content is marked
5. The start-state schema fields that other workstreams must fill in

Working method: options → trade-offs → recommendation; at most 5 questions; tag facts [V]/[R]/[D]. Real-world history is inspiration only; polities are fictional.
Deliverables: spec + canon doc on a branch with a PR; draft content/settings/date/ and content/maps/earth/ RON files if the schema is stable; POAM updated.
Done when: every WLD- requirement is testable, the start-state schema is agreed, and a stub scenario loads under the WS-02 validator.
```
