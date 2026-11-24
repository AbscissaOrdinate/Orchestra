# Orchestra — Concept of Operations (ConOps)

| Field | Value |
|---|---|
| Status | Baseline v0.3 |
| Owner | Sean |
| Workstream | WS-01 |
| Last updated | 2026-11-24 |

This is the top-level design document. Workstream specs (`docs/specs/`) derive from it and trace back to the `SYS-` requirements in §9. If a spec conflicts with this document, this document wins until it is formally revised (see `CLAUDE.md`).

---

## 1. Purpose

Orchestra is a turn-based grand strategy sandbox. The player steers a polity through the space age, 1955–2060 in the MVP. The design priorities are realism through coupled, nondeterministic models, legibility of cause and effect, and achievable scope for a solo developer working about 10 hours a week with AI-assisted implementation.

**Player fantasy:** steer a state through the space age under uncertainty. You spend budget and political capital, bet on R&D whose outcome you can't know in advance, and live with the consequences.

**Differentiating triad**, which the MVP builds deep:
1. **Governance and internal politics.** Factions with rational interests, and government structures that can be redesigned.
2. **Economics.** Fiscal policy (detailed taxes, bonds, debt dynamics), monetary policy, demographics and defense economics.
3. **Uncertain R&D.** A nonlinear, nondeterministic tech progression, with craft design treated as R&D.

Every other system ships as a thin stub behind a stable interface.

## 2. Core loop

The game is turn-based with **monthly ticks**. It runs until the end date or until the player quits.

1. **Decide.** Set the budget and tax schedule, monetary stance (if the central bank isn't independent), policies, R&D portfolio and design projects, diplomatic actions, and force posture.
2. **Resolve.** R&D and design projects advance stochastically. The economy, demographics and factions update. AI polities act. Treaties are checked. Engagements are auto-resolved.
3. **Inform.** Reports, events and intel updates. Every reported value can be decomposed into what contributed to it (SYS-004).

There are no win conditions. The game is a sandbox with optional retrospective scoring dashboards (post-MVP).

## 3. Scope

| Area | MVP (in) | Deferred (out) |
|---|---|---|
| Playable polity | Nation-state only. The polity data model stays generic. | Corporations, national unions, nonstate actors as playable |
| AI polities | Player + 3–5 high-fidelity AI + ~20–40 macro-level polities. Tiers can be promoted or demoted. | — |
| Corporations | AI-only, as defense and space contractors supplying the component and design market | Stock market, corporate bonds, corporate play |
| Nonstate actors | Macro-level AI only | Playable nonstate mechanics |
| Timeframe | 1955–2060 | Out to 2150 |
| Earth map | Abstract region graph of about 100 regions on real geography | Polygon or tile map |
| Space map | Solar System node graph with delta-v edges and launch windows | Detailed orbital mechanics |
| Governance | Parametric institutions, factions, and reform as a project | Freeform scripting |
| Economy | Sectoral production, quantile income distribution, full tax suite, bonds, monetary policy, cohort demographics, defense procurement | Detailed trade networks, finance sector |
| R&D | World-level latent truth, polity beliefs, stochastic projects, hidden cross-field enablers | — |
| Craft design | Hull presets + component slots + derived stats; preset 2D silhouette | Movable external components, cosmetic only (OQ-3) |
| Defense market | Buy, license, or arms-deal designs and components, with tracked provenance | — |
| Diplomacy | Alliances (overlapping, multiple types), clause-based treaties with probabilistic violation detection, a basic intel mechanic | Covert ops depth |
| Force design | Data-driven formation templates (sub-unit slots + equipment slots + personnel), era presets, research-unlocked slots and doctrine choices. Design from company level; **track** BCT/brigade, ship, squadron, SOF company, spacecraft/constellation (§6.1). Groupings: squadron/fleet, wing, joint task force. | Division/corps HQs as tracked echelons; generals and leaders; player-authored doctrine |
| Manpower & readiness | Recruitment pool from demographics; conscription vs. volunteer policy; active/reserve/cadre tiers; training lag; funded readiness with decay; one experience scalar | Detailed personnel management |
| Basing & infrastructure | Naval, air, army, space (launch sites, ground stations) and production (shipyards, plants) facilities with capacity; overseas access by treaty | Base defense layouts; detailed logistics network (MVP: abstract supply capacity along region edges) |
| Munitions | Stockpiles in a few aggregate classes, consumed by combat and refilled by production | Per-munition-type tracking |
| Strategic forces | Abstract: warhead stockpile, delivery systems, survivability and posture produce a deterrence-credibility value used by diplomacy and AI; nuclear use is a catastrophic escalation event | Nuclear exchange modeling |
| Combat | Auto-resolved operational engagements: salvo (naval, air, space) and Lanchester (ground), consuming unit aggregates only | Any tactical layer; cyber/EW beyond a modifier |
| UI | Text-first, SVG vector, MIL-STD-2525 symbols, ASCII fallback | 3D |

## 4. Setting

- **Default setting:** a DATE-style world. That means fictional polities on **real Earth geography**, modeled on the U.S. Army Decisive Action Training Environment, expanded so the board has enough actors. The 1955 back-history is written in-house at low fidelity.
- **Extensibility (SYS-007):** the setting is pure data. Polities, region ownership, start conditions, and names live in content files, separate from the map, which is a fixed region graph. Adding, removing, or renaming nations, or swapping in another setting on the same map, must need no code changes.
- **Solar System:** a variant of ours, generated from a seed. Hypothetical moons and minor bodies can exist, and off-Earth resources are sampled per world and revealed through prospecting.
- **DATE usage (OQ-1, ADR-0003):** DATE is Distribution A (approved for public release) and a U.S. Government work in the public domain, so it may be used in this GPL repository. The README and the game's credits must carry a non-endorsement disclaimer. The 1955 back-history and canon are written in WS-08.

## 5. Cross-cutting design principles

| ID | Principle | Consequence |
|---|---|---|
| P-1 | **Physics is per-world; knowledge is per-polity.** | At world generation, every tech, resource, and deposit gets a sampled latent truth. Each polity holds beliefs about it (estimate ± σ). Research, surveys, and espionage narrow those beliefs, and projects add execution noise. Dead ends are consistent across polities within a world. |
| P-2 | **One belief mechanic.** | The same (estimate, σ) structure serves tech uncertainty, intel and fog of war, treaty-violation detection, resource prospecting, and AI reasoning under uncertainty. |
| P-3 | **Explainability.** | Every computed state variable carries a contribution breakdown that the UI can show. The UI can always answer "why did X change?" |
| P-4 | **Determinism.** | A single seeded RNG stream (or streams derived from it) produces the same outputs from the same seed and the same inputs. No wall-clock time and no unordered-map iteration in the simulation. |
| P-5 | **Headless core.** | The simulation is a library with a CLI. The UI is a thin client. Batch Monte Carlo runs are the balancing and V&V tool. |
| P-6 | **Data-driven content.** | Techs, components, hulls, government parameters, polities, and events are content files, not code. |
| P-7 | **Fidelity tiers.** | High-fidelity polities run the full models. Macro polities run reduced-form equations on a few aggregates. Polities can move between tiers. |
| P-8 | **Coupling through shared objects.** | Economic interest groups *are* political factions. R&D projects *are* how designs come to exist. Avoid duplicate parallel models. |
| P-9 | **Prototype before porting.** | Models are first sketched in Python, MATLAB, or a spreadsheet (`prototypes/`) and tuned there, then implemented in Rust against a spec. |
| P-10 | **Compose upward with one slot pattern.** | Components → craft designs → formation templates all use the same slot/derived-stat mechanism. Each tracked unit exposes a few aggregates (offense, defense, staying power, detection, mobility, upkeep), and higher systems consume only those. |

## 6. System overview

Every model choice below is provisional until its workstream spec is approved.

| System | MVP depth | Baseline model | WS |
|---|---|---|---|
| Macroeconomy | Deep | A few sectors with CES/Cobb-Douglas production and a tech multiplier; capital accumulation; stochastic shocks; Markov regime states | WS-03 |
| Taxation | Deep | Lognormal-body/Pareto-tail income and wealth distributions in about 20 quantile bins. Any schedule can be applied: progressive income, wealth, corporate, VAT/sales, capital gains, estate, tariffs, subsidies. Behavioral response comes from an elasticity of taxable income. | WS-03 |
| Public finance | Deep | Deficits financed by bonds. Yield = f(debt/GDP, inflation, credibility, risk), so debt spirals can emerge (TNO-inspired). | WS-03 |
| Monetary | Medium | Central bank independence is a governance parameter. An independent bank follows a Taylor-type rule. Inflation comes from a simple Phillips curve. | WS-03 |
| Demographics | Medium | Cohort-component model in 5-year bands. Fertility and mortality depend on income, tech, and policy. | WS-03 |
| Governance | Deep | Selectorate theory (S, W, public vs. private goods) plus veto players. A government type is a parameter set. Reform is a coalition-gated project with crisis risk. | WS-04 |
| Factions | Deep | Interest vectors over policy space plus power weights. They drive stability and economic behavior (investment, R&D talent, labor). | WS-04 |
| R&D | Deep | P-1 latent truth. Lognormal cost and schedule, beta success probability, mixtures for multimodal outcomes. Hidden enabling edges. Funding-limited project slots with diminishing returns. | WS-05 |
| Craft design | Medium | A design is an R&D project. Spec changes slip the schedule. Components carry provenance. | WS-05 |
| Defense industry | Medium | Wright's-law learning, production-rate effects, operations and support lifecycle costs; a market for buying, licensing, and arms deals (War Footing-inspired) | WS-05 |
| Diplomacy & AI | Thin | Utility AI weighted by archetype × strength tier. Bargaining model for war/peace. Repeated-game reputation. Clause-based treaties. | WS-06 |
| Intelligence | Thin | P-2 belief updates | WS-06 |
| Force design | Medium | Formation templates (P-10); unit lifecycle (commission, refit/modernize, mothball, decommission, sell); deployment orders by theater/region with simple postures (defend, attack, patrol, blockade, strike, deter) | WS-11 |
| Manpower & readiness | Thin | Pool drawn from WS-03 cohorts; conscription is a WS-04 policy; readiness decays unless funded; O&S cost per unit | WS-11 |
| Basing & munitions | Thin | Facility capacity per region; access from WS-06 treaties; munitions stockpiles × production rate | WS-11 |
| Strategic forces | Thin | Stockpile + delivery + survivability → deterrence credibility (an input to WS-06 bargaining) | WS-11, WS-06 |
| ISR assets | Thin | Satellites, patrol aircraft and SOF reconnaissance feed P-2 belief updates | WS-11, WS-06 |
| Services | Thin | Army, navy, air and space services are WS-04 interest groups competing for budget share | WS-04, WS-11 |
| Combat | Thin | One stochastic attrition kernel: Armstrong stochastic salvo (naval, air, space; sequential-fire variant for first strikes), Lanchester square/linear (ground); consumes unit aggregates × readiness × supply × experience | WS-07 |
| Space | Thin | Hohmann delta-v graph, synodic windows, prospecting (P-2), outposts | WS-08 |
| UI | Thin | Tauri webview, SVG, MIL-STD-2525 symbology, text-first | WS-09 |

### 6.1 Force echelons

Design granularity is not tracking granularity. Templates are built from small building blocks, but only the tracked echelon exists as an entity on the map and receives orders. This keeps entity counts in the hundreds per polity, not thousands (R-6).

| Domain | Template building blocks | Tracked unit | Groupings |
|---|---|---|---|
| Army | Company → battalion → regiment (optional) → brigade / BCT | Brigade / BCT | Joint task force (divisions and corps post-MVP) |
| Special operations | Company | Company | Joint task force |
| Navy | Ship design (WS-05) | Ship | Squadron → fleet |
| Air | Aircraft design → squadron template | Squadron | Wing → air force |
| Space | Spacecraft design | Spacecraft or constellation | Orbital group |

Variants such as marines and amphibious forces, airborne, carrier air wings, and paramilitary or gendarmerie (relevant to WS-04 internal stability) are templates, not separate systems. Space is a separate service from about the 1960s; the setting decides when.

## 7. Architecture (provisional; to be confirmed in an ADR during WS-02)

- `crates/sim-core`: pure Rust simulation library (P-4, P-5).
- `crates/cli`: headless runner, batch Monte Carlo, scenario I/O.
- `app/`: Tauri shell with an HTML/CSS/JS front end consuming sim-core (ADR-0002). 2525 symbols likely via the `milsymbol` JS library (to be verified).
- `content/`: data files in RON, with prose in Markdown (ADR-0001).
- `prototypes/`: model sketches in Python or MATLAB.

## 8. Top risks

| ID | Risk | Mitigation |
|---|---|---|
| R-1 | Coupled stochastic systems are illegible, and outcomes feel arbitrary | P-3 explainability; show the σ on beliefs; telegraph uncertainty |
| R-2 | Runaway feedback loops (debt spirals, population blowups) | Damping terms; batch Monte Carlo stability tests in CI (WS-10) |
| R-3 | Content volume (techs, components, events across about 100 years) | P-6; procedural parameter sampling; MVP timeframe limit |
| R-4 | Solo developer bandwidth and Claude Pro usage limits | Spec-first with testable acceptance criteria; thin stubs; the grader loop limits rework |
| R-5 | DATE licensing in a public GPL repo | Resolved: Distribution A / public domain, with a non-endorsement disclaimer (ADR-0003) |
| R-6 | Force micromanagement and entity count | Tracked echelons fixed per domain (§6.1); templates applied in bulk; AI auto-organizes forces for macro polities |

## 9. System requirements (top level)

| ID | Requirement | Acceptance |
|---|---|---|
| SYS-001 | The player can play a nation-state from 1955 to 2060 in monthly turns. | A scripted full-length run completes without panics. |
| SYS-002 | The simulation is deterministic given a seed and an input log. | Two runs with the same seed and inputs produce byte-identical state hashes. |
| SYS-003 | The simulation runs headless through the CLI, including batch Monte Carlo. | `cli batch --runs N` emits summary statistics. |
| SYS-004 | Every displayed state variable exposes a contribution breakdown. | An API returns a breakdown for any listed variable; the breakdown terms sum to the value within tolerance. |
| SYS-005 | Technology truth is sampled per world; polities hold beliefs. | The same seed gives the same truth for every polity. Beliefs converge toward truth as research increases. |
| SYS-006 | Polities run at high or macro fidelity and can switch tiers. | A tier switch conserves the key aggregates (GDP, population, military power) within tolerance. |
| SYS-007 | The setting (polities, ownership, start state) is content data, separate from the map. | A test scenario adds a polity using data files only. |
| SYS-008 | Performance: one monthly tick with the MVP polity counts takes under 100 ms on mid-range hardware (provisional). | Benchmark run in CI, including ~300 tracked units per high-fidelity polity |
| SYS-009 | Forces are composed from data-driven formation templates. Only the tracked echelon per domain (§6.1) exists as a map entity, and each tracked unit exposes aggregates with breakdowns. | A new template defined only in content files can be built and deployed. A unit's aggregates equal the rollup of its sub-units and equipment, with breakdown terms that sum to the value. |

## 10. Workstreams

| WS | Name | Kind |
|---|---|---|
| WS-01 | ConOps, requirements, MVP, prompt design | Design (this Project) |
| WS-02 | Architecture and data model | Design → code |
| WS-03 | Macroeconomy, fiscal/monetary, demographics | Design → prototype → code |
| WS-04 | Governance and factions | Design → prototype → code |
| WS-05 | R&D uncertainty, craft design, defense industry and market | Design → prototype → code |
| WS-06 | Diplomacy, intelligence, AI | Design → code |
| WS-07 | Combat | Design → code |
| WS-08 | World generation, space map, resources, setting data | Design → code |
| WS-09 | UI/UX and symbology | Design → code |
| WS-10 | V&V: the spec-grader workflow, tests, batch stability | Cross-cutting |
| WS-11 | Force design, manpower, basing, munitions, strategic forces | Design → code |

Order: WS-01 → WS-02 → WS-03 and WS-04 in parallel → WS-05 → WS-11 → WS-07, then WS-06, WS-08 and WS-09. WS-10 runs throughout.

## 11. Open questions

| ID | Question | Owner |
|---|---|---|
| OQ-1 | What is DATE's distribution and release status, and can its names and text be used in a public GPL repo? | **Closed:** yes, with a disclaimer (ADR-0003) |
| OQ-2 | Content file format | **Closed:** RON + Markdown (ADR-0001) |
| OQ-3 | Do movable external craft components affect stats (e.g., signature, stability), or are they cosmetic only? | **Closed:** cosmetic only |
| OQ-4 | Can the claude.ai Project sync from this repo, avoiding duplicated docs? | Open: Sean to test |
| OQ-5 | Tauri vs. a pure-Rust UI (egui/iced) for a text-heavy interface | **Closed:** Tauri (ADR-0002) |
| OQ-6 | Where force state lives in the WS-02 world model: a `force` block, its stage placement, and how combat losses (WS-07, stage R6) get back to it under the one-writer-per-block rule | WS-02 review, before approval |

## 12. Research backlog

Closed 2026-11-22. See `docs/research/m1-findings.md`.


- How TNO's economy and budget mechanics work (currently known only loosely)
- War Footing's licensing and arms-deal system
- Terra Invicta's tech tree structure
- Victoria 3 interest groups as a precedent for P-8
- Formalizations of selectorate theory and veto players suited to games
- Distributions of defense cost and schedule growth, and Wright's-law parameters by domain
- Tauri, `milsymbol` (license, which 2525 revision it supports), deterministic RNG crates
