# WS-08b — Solar System, Resources & Space Operations
Status: Draft v0.3 · Parent: ConOps §3 (space map row), §4 (Solar System), §5 P-1/P-2/P-3/P-4/P-6/P-7/P-10, §6 space row, SYS-002, SYS-004, SYS-005, SYS-006, SYS-008 · Depends on: WS-02 v1.1, WS-03 Draft v0.3 (§2.2–2.4, §3.1, §3.3, §3.5, §3.6, §3.10, §3.13, §5.1), WS-08a Draft v0.2 (§3.2, WLD-047), WS-04 Draft v0.3 (§2.1 Space axis, §3.4 `gate_orders`, §4.4 private goods, §6.4 policy map), `docs/research/salvo-model.md` · Last updated: 2026-11-27

Tags: **[V]** checked against the cited source this session, or computed by `prototypes/ws08b-sol/sol.py` (marked "computed") · **[R]** recalled or secondary, not checked against a primary source · **[D]** design choice. Every [R] and [D] number is a calibration starting point that M2(d) (§7.2) confirms or replaces.
Requirement IDs WLD-050–099 (WS-08a ends at WLD-049). Nothing here edits another document; proposed edits are listed in §8.

**v0.3 (sync with merged WS-03 v0.3 and WS-04 v0.3, 2026-11-27):** private resource rents now reach households as business equity through WS-03's household panel and five-group wealth dynamics: WS-08b publishes a resource-equity value, so a price windfall shows up as asset-price gains for equity-heavy groups (§4.6, WLD-074, P-03j). WS-04's coup and revolution `capital_loss` shocks also hit deposit development (§4.6, WLD-088). WS-08b orders that commit budget money go through WS-04's `gate_orders` (§3.4, P-04d). §8 now proposes concrete WS-04 `policy_map.ron` rows (GOV-021) and answers WS-04's OQ-G8 revisit (P-04e). Proposed WS-03 scenarios were renumbered S22/S23, because WS-03 now uses S18–S21. §8.1 lists what each workstream needs to do or confirm.

**v0.2 (Sean's review, 2026-11-27):** OQ-S1–S5 closed as recommended. Generated small bodies now follow the real populations: vulcanoids, near-Earth asteroids by group, Mars and Jupiter trojans, main belt with its composition gradient, Hildas (§4.1 G-4). Canonical minor bodies are kept in every world: Ceres, Pallas, Juno, Vesta, Hygiea, Psyche, Eros, Pluto and Charon (§4.1 G-1a).

---

## 1. Purpose & scope

WS-08b owns the seeded Solar System, the resource model on and off Earth, the rent hook into the economy, prospecting, and a thin space-operations layer (missions, launch, logistics, outposts and stations). It gives WS-07 what its engagement generator needs for orbital combat.

**In (MVP):** a seeded Sol variant (32 canonical template bodies plus up to 18 generated bodies, sampled into `Truth`); a delta-v graph over orbit-regime nodes with Hohmann edges, synodic launch windows at monthly resolution and transit times; one `OrbitRegime` enum shared with the salvo model; a four-kind resource taxonomy, the `resource_tags` vocabulary and per-world deposit truth; a world commodity price block that loads on WS-03's `f_g`; the rent hook (production, depletion, rent, and its split into royalty, SOE take, private income and diverted rents); prospecting through the one belief mechanic; missions, launch capacity, delta-v × mass logistics, facilities, the spacecraft resupply rule, debris, and engagement inputs; a macro-tier reduced form.

**Out:** n-body and patched-conic trajectories (gravity assists, Lambert porkchops, low thrust); detailed station design (facilities are templates, not WS-05 slot designs); an environment or conservation model (a WS-04 axis hook is reserved, §8); bilateral commodity trade (commodities clear on one world market, which feeds WS-03's trade stub, §4.7); Venus and gas-giant surfaces; interplanetary interception (combat happens only at nodes).

## 2. Decisions

Each decision lists options, trade-offs and the recommendation. D1–D5 answer the kickoff's five questions; D6–D9 follow from them.

| # | Decision | Options | Trade-offs | Recommendation |
|---|---|---|---|---|
| D1 | Node granularity and graph size | (a) one node per body (~45); (b) **nodes per orbit regime where a regime drives a mechanic**; (c) fine altitude bands and every Lagrange point | (a) can't distinguish LEO from GEO, which ASAT reach, early warning and comms depend on. (c) multiplies nodes with no extra mechanic and needs n-body data for L-points. | **(b)** [D]. Earth: Surface, Low (300 km), Medium (20,200 km), Synchronous (35,786 km), High (Molniya-type ellipse) and one Lagrange point (EML1, the cislunar depot site). Other bodies: Surface (if landable) and Low; small bodies a single Surface node; giant planets a single High capture ellipse (periapsis 1.5 R, apoapsis 100 R) as hub. Template: 32 bodies, 53 nodes, 256 directed edges; worst case over 2,000 seeds 49 bodies, 70 nodes, 884 edges [V: computed]. That is inside the ARC-090 bench's 50 bodies. |
| D2 | Hypothetical bodies | (a) none; (b) **a sample of the notable, plausibly-undiscovered-in-1955 tail of each real small-body population, with that population's orbits, sizes and composition**; (c) freely generated planets and large moons | (a) leaves nothing to discover. (c) breaks plausibility: anything large would have been catalogued before 1955. A full census is impossible at this entity budget, so the counts are a gameplay sample. | **(b)** [D], accepted. Nine classes (mean, max): vulcanoids (0.5, 1), near-Earth asteroids (4.5, 6), Mars trojans (0.3, 1), main belt (3, 4), Hildas (0.5, 1), Jupiter trojans (1.2, 2), irregular moons of Jupiter and Saturn (0.7, 1 each), a Mars moonlet (0.3, 1). Mean 10.2 per world, max 17 [V: computed]. Canonical minor bodies are template bodies, present in every world. Small-body phases are sampled per world (OQ-S4, closed); trojans keep their ±60° lock to the host. |
| D3 | Resource taxonomy | (a) one generic "resource rent" scalar; (b) **four kinds, each driving a named mechanic**; (c) 8–12 kinds (coal, rare earths, He-3, arable land, ...) | (a) can't express that fissile ore matters for bombs and ice matters for propellant. (c) adds kinds with no mechanic (He-3 needs fusion; arable land would recalibrate WS-03 agriculture; rare earths need supply chains). | **(b)** [D]: `hydrocarbons` (rent, commodity factor), `metals` (rent, commodity factor; on and off Earth), `fissile` (rent, nuclear feed index), `volatiles` (in-space propellant only). Validation requires each kind to list ≥ 1 mechanic (WLD-075). Deposits: Earth, one per region tag (66 in the draft); off Earth, ≤ 63 candidates [V: computed]. |
| D4 | Rent hook form | (a) a TFP multiplier on agriculture & extraction; (b) a natural-resource factor inside the sector CES; (c) **an additive rent term outside the CES, computed per deposit as quantity × (price − unit cost)** | (a) splits rent by factor shares, so there is no royalty or SOE take and no rentier effect. (b) changes ECO-010's CES and its calibration. (c) is the World Bank rent definition [V: WDI method, "difference between the price of a commodity and the average cost of producing it"], leaves ECO-010/011 untouched, and splits cleanly by owner. | **(c)** [D]. Rent = Σ_d q_d·(p_k g_d − c_k δ_d). It enters agriculture & extraction value added as an additive term (normal factor returns are already in the CES), then splits into royalty, SOE take, private capital income and diverted rents (§4.6). WS-03 §6.1 stability is argued in §4.7 and tested in M2(d) (WLD-089). |
| D5 | Launch windows and the minimum facility | Windows: (a) none; (b) **closed-form optimal departure instants per heliocentric edge, a ±1-month open window, quadratic Δv penalty**; (c) porkchop tables. Facilities: (a) none; (b) **one `Facility` entity with template functions (habitat, extract, depot, resupply, sensor)**; (c) modular WS-05 designs | Windows (a) is unrealistic; (c) needs Lambert solvers and real ephemerides. Facilities (a) has nowhere to put ISRU or a lunar base; (c) is out of MVP scope. | Windows **(b)** [D]: with real J2000 phases, all seven historical Mars launch months 1960–75 fall inside an open window, the worst within 1.36 months of the optimum [V: computed, §7.1]. Planetocentric legs (Earth–Moon, moons of a planet) are always open and finish within the tick. Facilities **(b)** [D], with templates in content; WS-05 slot designs post-MVP. |
| D6 | Where space state lives, and stage placement | (a) WS-11 owns everything, including positions; (b) **WS-08 owns positions, Δv stocks, missions, facilities, launch-vehicle stock and debris; WS-11 owns everything else about a unit** | (a) forces WS-11 to implement orbital movement. (b) needs a clear split, but each block keeps one writer. | **(b)**. One world-level `space` block written by WS-08 in R5b (movement, logistics, resupply) and R6b (applying engagement results). Resources run in R2. WS-08 does nothing in R6, so WS-07 sees this tick's positions (WS-02 edit P-02c). |
| D7 | Belief subjects for deposits | (a) three subjects (size, grade, difficulty); (b) **size for every deposit, plus a cost-ratio subject off Earth only**; (c) size only | (a) costs ~450 subjects × 47 polities. (c) hides off-Earth economics that real missions do discover. | **(b)** [D]: ≤ 192 subjects [V: computed], ≈ 0.72 MB of beliefs (§4.12). Earth cost ratios become known to the owner at discovery (conventional geology), a stated simplification. |
| D8 | Logistics abstraction | (a) count missions only; (b) **rocket-equation mass ratio: delivering m needs m·exp(Δv/v_e) of LEO mass, with depots splitting the path**; (c) propellant-tracked trajectories | (a) makes ISRU worthless. (c) is out of scope. | **(b)** [D]: one formula, legible, and it gives volatiles and nuclear-thermal propulsion (higher v_e) their real value. |
| D9 | Commodity prices | (a) price = exp(η·f_g) only; (b) **one world market per kind: f_g loading plus a scarcity term (demand vs. capacity) with partial adjustment**; (c) bilateral trade with clearing | (a) can't produce depletion-driven price rises or a glut after big discoveries. (c) is out of scope. | **(b)** [D]. A linear-stable loop (spectral radius 1 − 2/τ_p = 0.83 at τ_p = 12 months, §4.7). |

## 3. Interfaces

### 3.1 Content layout

```
content/maps/sol/bodies.ron           [Body]            template bodies and their nodes (real data, cited)
content/maps/sol/transfers.ron        [Edge]            overrides and Lagrange edges; all other edges are derived
content/maps/sol/gen_rules.ron        GenRules          hypothetical-body classes
content/maps/earth/resources.ron      [(RegionId, [tag])]   Earth resource layer (OQ-S1, accepted)
content/core/resources/kinds.ron      [ResourceKind]    taxonomy, prices, costs, rates
content/core/resources/size_classes.ron  SizeClasses    deposit priors and qualifiers
content/core/resources/body_deposits.ron BodyDeposits   off-Earth candidates
content/core/resources/surveys.ron    [Survey]          prospecting survey types
content/core/space/facilities.ron     [FacilityTemplate]   (M4; schema in §5)
content/core/space/reach.ron          [ReachRule]       engagement reach vocabulary (WS-07 fills Pk; §4.9.8)
content/settings/<s>/deposit_start.ron [(DepositId, dev)]  1955 development levels (OQ-S5, accepted; setting data, values from M2(d))
```

Map files (sol, Earth resource layer) hold physical data only and never mention polities (ARC-052).

### 3.2 Data model (sketch; Rust types are the schema, ADR-0001)

```rust
pub enum OrbitRegime { Surface, Low, Medium, Synchronous, High, Lagrange, Transit }   // shared with salvo `orbit_regime`
// Transit is a position state only (in a heliocentric leg); no node has it and nothing at Transit can be engaged.
pub struct NodeId(pub u16);                     // dense; node IDs "node.<body stem>.<regime snake>[_<label>]"
pub struct SolNode { pub body: BodyId, pub regime: OrbitRegime, pub r_peri_km: f64, pub r_apo_km: f64, pub hub: bool }
pub struct Transfer { pub dv: f64 /* km/s */, pub dv_arrival: f64, pub aero_ok: bool, pub months: u16,
                      pub kind: TransferKind /* Ascent | Descent | OrbitChange | Moon | Helio | Override */ }
pub struct Sol { pub bodies: Vec<Body>, pub graph: Graph<SolNode, Transfer> }   // replaces WS-02 `Graph<Body, Transfer>` (P-02a)

// Truth (worldgen only; immutable, ARC-011)
pub struct BodyTruth { pub mean_lon0_rad: f64, pub generated: Option<GenOrbit> }       // phase; generated orbit and size
pub struct DepositTruth { pub size: f64, pub grade: f64, pub difficulty: f64 }          // size in kind units; barren = dry
// World state, WS-08 is the only writer
pub struct ResourceState {                       // R2
    pub price: Vec<Tracked>,                     // per kind, RC per unit
    pub scarcity: Vec<f64>,                      // ζ_k
    pub deposits: Vec<DepositState>,             // per deposit, dense DepositId
    pub by_polity: Vec<ResourceIncome>,          // per polity, this tick
}
pub struct DepositState { pub cum_produced: f64, pub dev: f64, pub output: Tracked, pub owner: Option<PolityId> }
pub struct ResourceIncome { pub rent: Tracked, pub royalty: f64, pub soe: f64, pub private: f64, pub diverted: f64, pub v_res: Tracked,
                            pub net_position: Vec<f64> /* per kind, units */, pub capex_due: f64 }
pub struct SpaceState {                          // R5b, R6b
    pub positions: BTreeMap<UnitId, SpacePos>,   // spacecraft only; absent = on the ground (WS-11)
    pub missions: Vec<Mission>, pub facilities: Vec<Facility>,
    pub lv_stock: BTreeMap<(BaseId, DesignId), u32>, pub debris: Vec<Tracked> /* per node */,
    pub hazards: Vec<HazardRecord>,              // this tick: debris hits, for WS-11 in R6b
    pub macro_assets: Vec<Option<MacroSpace>>,   // per polity, macro tier only
}
pub struct SpacePos { pub node: Option<NodeId> /* None = Transit */, pub dv_stock: Lazy, pub mission: Option<MissionId> }
```

### 3.3 Stage placement (WS-02 §3.3; edit P-02c)

| Stage | WS-08b function | Writes | Reads (stage-start) |
|---|---|---|---|
| R1 | `wld::apply_order` | `resources` order fields, `space` order fields | orders |
| R2 | `wld::resources` | `resources` (prices, production, rents, discoveries) | `econ.shock.f_g`, `agg.gdp`, FX, ownership, `rnd` capability flags (all one tick old) |
| R5b | `wld::space_ops` | `space` (launches, legs, arrivals, logistics, facility supply, resupply flows, debris hazards) | `force` (units, launch sites), `rnd` (deliveries, v_e), `resources` |
| R6b | `wld::space_losses` | `space` (Δv spent, facilities damaged or destroyed, debris added, dead units' positions dropped) | `engagements` |
| I1 | — | — | WS-08b pushes `Observation`s from R2 and R5b; I1 applies them |

Lags: R2 uses last month's `f_g`, GDP and FX, so commodity prices respond to the global factor one month late. WS-03 R3 sees this month's rents. A spacecraft arriving at a node in R5b can fight in R6 of the same tick.

### 3.4 Orders (WS-08b variants of `OrderKind`)

| Order | Effect | Rejected (event) when |
|---|---|---|
| `Survey { kind, target }` | Starts an Earth survey (geological, drilling) or an observatory campaign | Not owner and no WS-06 access; capability missing; already running |
| `DevelopDeposit { deposit }` | Schedules capex with WS-03, then ramps `dev` over `dev_months` | Not owner; no belief; offshore/arctic capability missing; already developed |
| `Launch { site, vehicle, payload, plan }` | Uses one vehicle from stock and one pad slot; creates a mission | No stock; pads exhausted this month; payload too heavy; plan infeasible (§4.9.2) |
| `BuildFacility { template, node }` | Creates a facility under construction; cargo missions deliver its mass | Node regime not allowed by template; extractor without a believed deposit, or one already held (OQ-S3) |
| `SetFacility { facility, assignment }` | Sets an extractor's output to depot or Earth return; depot sharing | Not owner |
| `Abandon { facility }` | Crew returns if a return vehicle is present, else is lost | — |

AI polities issue the same orders (WS-06). Macro polities issue none; their space activity is the §4.10 reduced form.

**Gating.** `Survey` and `DevelopDeposit` commit budget money (the `exploration` and `resource_development` lines), so they pass through WS-04's pure `gate_orders` in R1 like every WS-03 order (WS-04 GOV-064), and WS-08b adds only range validation (P-04d). Launches, facilities and missions are paid for from the existing space-program line, which the annual budget already gates; their orders are validated by WS-08b alone.

### 3.5 What flows to and from other workstreams

| Direction | Item | Notes |
|---|---|---|
| WS-03 → WS-08b | `f_g` (R2 draw), GDP, price level, FX `e_i`, royalty rate, `state_own_s` of agriculture & extraction (from WS-08a `economy.system`, WS-03 §5.1), corruption K, r_real + eq_premium | read one tick old |
| WS-08b → WS-03 | `ResourceIncome` per polity: rent (real), royalty, SOE take, private, diverted; private resource equity `V_res`; commodity net positions (units, value); capex schedule | P-03a–P-03i |
| WS-04 → WS-08b | `shocks.capital_loss` (coups, revolutions); `gate_orders` results for `Survey` and `DevelopDeposit` | WS-04 §3.4, WS-03 §2.3 |
| WS-08b → WS-04 | diverted resource rents, inside WS-03's corruption-rents amount (`rents` channel); policy-map rows for the royalty rate and two budget lines | P-04a–P-04c |
| WS-05 → WS-08b | capability flags (offshore, arctic, observatory tier, orbital spectrometry, lander, aerocapture, ISRU), best v_e, launch-vehicle and spacecraft design stats, deliveries | P-05a |
| WS-08b → WS-05/WS-11 | fissile self-sufficiency index per polity | P-05b |
| WS-06 ↔ WS-08b | survey access, facility access (allied resupply), extraction-rights clauses; discovery publication; observations of others' missions | P-06a |
| WS-08b → WS-07 | positions, co-location groups, reach rules, Δv stocks, adjacency with Δv; debris and facility losses come back via `engagements` | P-07a |
| WS-11 ↔ WS-08b | launch sites (pads, cadence, latitude), spacecraft units; resupply flows (shared pure function); debris hazards | P-11a–P-11d |

## 4. Model

### 4.1 World generation (P-1)

| Rule | Content |
|---|---|
| G-1 | Template bodies come from `bodies.ron` with cited real values (§10 sources). Every node and derived edge is a pure function of the template plus `BodyTruth`. |
| G-1a | Every template body is **canonical**: present in every world with its real values. The canonical minor bodies are Ceres, Pallas, Juno, Vesta, Hygiea, Psyche, Eros, Pluto and Charon. A template body is public at start unless `discovered` is later than the scenario start (Charon, 1978 [R]). Such a body starts unknown, is found by surveys like a generated body (§4.8), and is published to everyone no later than January of its `discovered` year [D], so the canonical timeline holds roughly. Pluto and Charon get no heliocentric edge within the 240-month cap, so they are isolated in the MVP. |
| G-2 | Planets keep their real J2000 mean longitude and rate [V: JPL approximate positions, Table 1]. Bodies with `mean_lon: None` (moons, asteroids) get a phase from `worldgen_stream(seed, body_id, P_PHASE)`. Moon phases don't matter at monthly resolution (G-7). |
| G-3 | Each `GenClass` draws a count ~ Poisson(count_mean), truncated at `max`, from `worldgen_stream(seed, class_id, P_COUNT)`. Body k uses `worldgen_stream(seed, "<class_id>#<k>", purpose)`. Adding a class never changes another class's bodies (WLD-052). |
| G-4 | Populations follow the real ones; only the counts are a sample. Orbit rules by class: **Vulcanoids**: a ∈ [0.08, 0.21] AU, inside the stable zone (limit ≈ 0.21 AU [V: Stern & Durda]) and outside the evaporation zone (≈ 0.07–0.08 AU [V: Steffl et al.]); D ≤ 5.7 km, because a STEREO search rules out larger vulcanoids at 3σ [V: Steffl et al.]; near-circular. **NEAs**: group drawn by the debiased shares Amor 39.4%, Apollo 54.4%, Aten 3.5%, Atira 1.2% plus Vatira 0.3% (folded into Atira) [V: Granvik et al. 2018]; (a, e) drawn inside the group's definition (Amor 1.017 < q ≤ 1.3 AU; Apollo a > 1, q < 1.017; Aten a < 1, Q > 0.983; Atira Q < 0.983) and inside the NEO definition q < 1.3 AU, a < 4.2 AU [V: Granvik et al.]. **Trojans** (Mars, Jupiter): a = host's a, mean longitude = host's ± 60° plus a libration offset; Jupiter L4:L5 = 1.6:1 [V: Szabó et al. 2007, via Li et al. 2023]. **Main belt**: a ∈ [2.1, 3.3] AU outside the 3:1, 5:2, 7:3 and 2:1 Kirkwood gaps [R]; composition follows the gradient in which S-types dominate the inner belt and C/D-types the outer [V: arXiv:2506.02721] (P(stony) 0.7 → 0.2 across the belt, P(metallic) 0.1 [D]). **Hildas**: a ∈ [3.7, 4.2] AU, e < 0.3, i < 20° [R: secondary], primitive composition. **Moons**: in [lo, hi] × the parent's Hill radius r_H = a·(μ_p/3μ_☉)^(1/3), outside `roche_factor` × 2.44 R_p (fluid Roche limit), and ≥ `min_rel_spacing` from every other moon of the parent in semimajor axis. Sizes follow a truncated power law N(>D) ∝ D^−slope within the class's undiscovered-in-1955 range (Jupiter trojans up to 80 km, since only 14 were known even in 1961 [V: Wikipedia, secondary]). Draws violating a rule are rejected and redrawn (≤ 100 tries; a failure is a content error). Centaurs and the Kuiper belt are not generated in the MVP. 2,000 seeds: 0 violations; NEA groups 40.2/55.2/3.4/1.2%; Jupiter-trojan L4 share 0.627 (target 0.615); belt stony share 0.62 inside 2.5 AU vs. 0.29 beyond 2.82 AU [V: computed]. |
| G-5 | Generated bodies have one Surface hub node, a composition drawn from the class rule and GM from D and density. They start undiscovered by every polity (WLD-057). Eccentricity is drawn and kept in truth for the plausibility rules, but the Δv model ignores it (G-6). |
| G-6 | Heliocentric orbits are treated as circular at a, with inclination only entering the plane-change term (§4.2). Eccentricity is ignored [D]. This is the largest physics simplification and is why windows are approximate (§7.1). |
| G-7 | Planetocentric legs ignore moon phase and inclination (Triton's retrograde orbit included) [D]. |
| G-8 | Deposits: one per Earth tag (§4.4) and one per matching off-Earth rule. Truth comes from `worldgen_stream(seed, deposit_id, purpose)`. Deposit IDs are `deposit.<region stem>.<tag index>` and `deposit.<body stem>.<kind stem>`. |

### 4.2 Delta-v edges

All μ in km³/s², radii in km, results in km/s. vis-viva: v(r, a) = √(μ(2/r − 1/a)).

| Edge | Formula | Notes |
|---|---|---|
| Surface ↔ hub | ascent = override, else v_c(r_hub)·(1 + loss_atm); descent = override, else 0.1·v_c if aerocapture, else ascent | loss_atm: airless 0.14 (fits Moon 1.87 [R]), thin 0.16, thick 0.60 [D]. Earth 9.4 up / 0.1 down; Mars 4.1 / 0.7 [R: Wikipedia Δv budget] |
| Orbit change, same body | circular ↔ circular: Hohmann; circular ↔ ellipse with the same periapsis: one periapsis burn | LEO→GEO 3.8926 = hand calculation [V: computed] |
| Moon ↔ parent hub | one burn at the hub apse that puts the other apse at the moon's a_m; v∞ = \|v_transfer(a_m) − √(μ_p/a_m)\|; capture into the moon's hub by an Oberth burn (or v∞ itself for a small body) | LEO→LLO = 3.106 TLI + 0.821 LOI = 3.928 vs 4.04 in the literature [V: computed; R: Wikipedia] |
| Heliocentric hub ↔ hub | Hohmann v∞ at each end; each end Δv = √(v∞² + 2μ/r_p) − v_hub(r_p) (Oberth), or v∞ for a small body | LEO→LMO 3.590 + 2.091 = 5.681 [V: computed] |
| Plane change (helio) | Δi_eff = max(0, \|i_a − i_b\| − 2°); add v_☉(r)·Δi_eff in quadrature to v∞ at whichever end is cheaper | 2° free: launch timing absorbs it [D]. Pallas (34.8°) costs 16.0 km/s from LEO [V: computed] |
| Aerocapture | where the target body has an atmosphere and the polity has `cap.aerocapture`, the arrival part `dv_arrival` is replaced by 0.15 km/s of trim [D] | LEO→LMO 3.74 with aerocapture |
| Override | `transfers.ron` entries replace or add an edge (both directions unless `one_way`) | EML1 edges [R/V] |
| Pruning | heliocentric edges with T_H > 240 months are not built [D] | Neptune unreachable in the MVP (gravity assists are post-MVP) |
| Launch-site latitude | going to a Synchronous node from a Surface launch at latitude φ adds √(v_a² + v_s² − 2 v_a v_s cos φ) − (v_s − v_a) (combined plane change at apogee) | 0° +0, 5.2° +0.014, 28.5° +0.363, 45.9° +0.804, 62.9° +1.278 [V: computed]. Equatorial sites matter, as they did historically. |

**Path query.** `plan_path(from, to, tech) -> Path` runs Dijkstra on the cost dv + w_t·months (w_t = 0.02 km/s per month [D]) over edges the polity may use (aerocapture by capability). Ties break by node ID. With w_t = 0, LEO→Ceres goes via a Jupiter capture orbit (9.71 km/s, 81 months); with w_t = 0.02 it goes direct (10.16 km/s, 16 months) [V: computed]. `shortest_dv(from, to)` (w_t = 0) is the ARC-015 query.

### 4.3 Windows and transit time

For heliocentric edge A→B: n_X = √(μ_☉/a_X³); T_H = π√(((a_A + a_B)/2)³/μ_☉); required phase φ_req = π − n_B·T_H; phase φ(t) = L_B(t) − L_A(t). Optimal departures t*_j are the roots of φ(t) − φ_req (mod 2π) where the relative phase falls through zero, found in closed form from the linear L(t) (the prototype root-finds at 0.1-month steps). Ticks are mean Gregorian months from 1955-01-01 (JD 2435108.5).

- Month m is **open** for A→B if \|m + 0.5 − t*_j\| ≤ 1.5 for some j (the optimum month ± 1).
- Δv in an open month = edge Δv · (1 + 0.15·((m + 0.5 − t*_j)/1.5)²) [D]. Closed months: departure is not allowed (the mission waits).
- Transit time = ⌈T_H in months⌉: Earth→Mars T_H = 8.51 → 9 months; synodic period 25.62 months [V: computed].
- Planetocentric and override edges are always open and complete within the tick (`months = 0`); a chain of such legs completes in one R5b.
- Windows are a pure function of `Truth` and the tick: computed on demand, never stored.

### 4.4 Resources: taxonomy, tags, deposits

| Kind | On / off Earth | Mechanics | Unit |
|---|---|---|---|
| `res.hydrocarbons` | on | Rent; CommodityFactor | Gboe |
| `res.metals` | on, off | Rent; CommodityFactor | Mt Cu-eq |
| `res.fissile` | on, off | Rent; NuclearFeed (self-sufficiency = owned production ÷ program demand, to WS-05/WS-11) | kt U |
| `res.volatiles` | off | Propellant (ISRU: depot stock, §4.9.4) | Mt |

**Tag grammar** (the vocabulary of WS-08a's reserved `resource_tags`): `<kind>.<size class>[.<qualifier>]*`, kind without the `res.` prefix. Size classes come from `size_classes.ron` per kind (hydrocarbons: minor, major, giant, supergiant; metals and fissile: minor, major, giant). Qualifiers: location (`offshore`, `arctic`, `easy`), which set the base difficulty and the WS-05 capability needed; and knowledge (`suspected`, `unknown`; default known), which sets the 1955 public belief (§4.8). Unknown kinds, size classes or qualifiers are load errors. Offshore fields attach to the coastal land region, because sea zones are never owned (WLD-010).

**Deposit truth.** With probability p_dry the deposit is barren (size = 0.001 × median); otherwise ln S ~ N(ln median, σ). Grade g ~ LogNormal(0, grade_sigma). Difficulty δ = Π qualifier difficulty × LogNormal(0, 0.2). Off-Earth: exists with p_exists, S as above (generated bodies scale the median by D³), and the cost ratio ln(δ/g) ~ N(ln cost_ratio, 0.5).

### 4.5 World commodity price (per kind with `CommodityFactor` or `Rent`)

```
D_k(t)  = θ_k · exp(−h_k·years) · Σ_i ι_i Y_i(t−1) · (p_k(t−1)/p̄_k)^(−ε_k)      demand; θ_k set at worldgen so D_k(0) = S_k(0)
Ŝ_k(t)  = Σ_d q̂_d                                                                  capacity (u = 1), deposits with an owner
ζ*_k    = (1/ε_k) · ln(D_k / Ŝ_k)
ζ_k(t)  = ζ_k(t−1) + (ζ*_k − ζ_k(t−1)) / τ_p                                       τ_p = 12 months [D]
ln p_k  = ln p̄_k + η_k · f_g(t−1) + 2·tanh(ζ_k / 2)                                soft bound: scarcity moves p by at most ×e²
```

Breakdown of p_k: base, global factor, scarcity. ι_i is a content intensity multiplier per archetype (default 1). Scarcity is measured against capacity, not realized output, so shut-ins (low u) don't feed back into the price.

### 4.6 Production and the rent hook

Per deposit d of kind k with owner i (the region owner on Earth; the extractor's owner off Earth), monthly:

```
q̂_d = (r_k/12) · (S_d − cum_d) · dev_d                        capacity; off Earth also ≤ the extractor's rate
m_d = p_k · g_d − c_k · δ_d − c_ret,d                           unit margin; c_ret = return-logistics cost off Earth (§4.9.4)
u_d = σ(m_d / (0.25 · c_k · δ_d))                               smooth shut-in (logistic)
q_d = u_d · q̂_d;   cum_d += q_d                                 depletion: cum_d ≤ S_d always
ρ_d = q_d · m_d                                                 rent (RC); breakdown: volume × margin per deposit
ρ_i = Σ_{d∈i} ρ_d · e_i / P_i                                   real LCU for WS-03
```

**Split** (τ = royalty rate, a WS-03 tax; σ = `state_ownership` of agriculture & extraction; K = corruption; c_div = 0.4 [D], the same as WS-03's c_leak):

| Flow | Amount | Goes to |
|---|---|---|
| Royalty | ρ·τ·(1 − c_div K) | WS-03 revenue line `resource_royalty` |
| SOE take | ρ·(1 − τ)·σ·(1 − c_div K) | WS-03 SOE revenue line |
| Diverted | ρ·(τ + (1 − τ)σ)·c_div K | WS-03 corruption-rents amount → WS-04 private goods (`rents`) |
| Private | ρ·(1 − τ)(1 − σ) | business-equity income of households: WS-03 adds it to the capital income of the panel households holding equity in agriculture & extraction (then taxes it per household), and to the equity return of the five wealth groups (P-03j) |

The four flows sum to ρ exactly. In WS-03, agriculture & extraction output becomes Y_ext = Y*_CES,ext·(1 + gap) + ρ_i (only the above-normal return is added; unit costs pay the factors already in the CES). Effects: exporters gain GDP, revenue and FX reserves when f_g > 0; in autocracies diverted rents buy coalition loyalty without leaking public spending (the rentier effect, through WS-04's existing selectorate mechanics); revenue becomes procyclical with f_g (a resource-curse channel); a price collapse turns into deficits and, through WS-03 §3.7, into slowly building debt pressure.

**Resource equity (WS-03 v0.3 hook).** Private rent is a return on business equity, so its capitalized value belongs in household wealth. Per deposit, V_d = max(0, (1 − τ)(1 − σ) · 12 ρ_d) · a(T_d, r), with annuity factor a(T, r) = (1 − e^(−rT))/r, r = WS-03's r_real + eq_premium, and T_d = (S_d − cum_d)/(12 q_d) years, capped at 50 [D]. WS-08b publishes V_res,i = Σ_{d∈i} V_d (real LCU, with a per-deposit breakdown). Proposed for WS-03 (P-03j): add V_res to W_hh as an equity line, held through the groups' equity shares e_g, with ΔV_res counted in Gain_g, and subject to the same offshore share φ_off. Then an `f_g` windfall makes top, equity-heavy groups richer through asset-price gains, and resource oligarchs gain clout through WS-04's wealth-weighted institutions, with no new WS-04 rule. Unowned or state-owned rents add nothing (σ = 1 → V = 0).

**Development.** `DevelopDeposit` charges capex = capex_months × (believed monthly output at u = 1) × p̄_k, priced on the polity's **belief** of S, not the truth, so a bad estimate wastes money (P-1). WS-03 spreads it over `dev_months`: the state share through budget line `resource_development`, the private share as agriculture & extraction investment. Then dev ramps linearly to 1 over `dev_months`. Start levels come from the setting (`deposit_start.ron`, OQ-S5; default 0). Conquest transfers a deposit with its `dev` (WS-07 facility damage may lower it). A WS-04 `capital_loss` shock in polity i multiplies the `dev` of every deposit i owns by (1 − capital_loss), in the same R2 tick that WS-03 destroys that share of sector capital: extraction capacity is capital too.

### 4.7 Commodity channels to every polity, and stability

Each polity's consumption is allocated from actual world supply: C_k,i = S_k · D_k,i / D_k. Net position n_k,i = q_k,i − C_k,i (Σ_i n = 0 exactly). WS-03 receives n·p_k as commodity exports or imports. They enter NX (the §3.2 gap) and FX reserves (§3.10), and the import price enters the §3.4 import-price term with weight equal to the commodity share of imports. So an `f_g` shock reaches:

- **exporters:** rents (GDP, revenue, private income, diverted rents), NX and reserves up;
- **importers:** NX down (demand), import-price pass-through (cost push), reserves down.

The current WS-03 reduced form (ε = λ f_g + √(1 − λ²) ε_idio on ε_π and ε_d, λ = 0.5) would count these effects twice. Proposed (P-03f, OQ-S2): keep it, but read it as the **non-commodity** world shock with default λ = 0.3, to be tuned in M2(d). ECO-022's form is unchanged.

**Stability (R-2).** (1) Price loop: with capacity independent of price, ζ* = c − ζ, so ζ(t+1) = ζ(t)(1 − 2/τ_p) + const. That is monotone and stable for any τ_p ≥ 2 (0.833 at 12), and the tanh bound only lowers the gain. Using realized output instead would put the logistic supply response inside the loop, with gains around −26 near shut-in, which oscillates unless τ_p > 13. Capacity is used for that reason. (2) Rents are bounded by capacity and depletion; the price by the soft bound. (3) The new WS-03 terms are additive and bounded, so ECO-012's Solow identity and the §3.14 damping still hold. (4) The residual risk is debt spirals in exporters after a price collapse. That is intended behavior (WS-03 §0), but it must stay inside §6.1, so M2(d) adds exporter-archetype rows (P-03h). WLD-089 requires every WS-03 §6.1 row to pass with resources on.

### 4.8 Prospecting (P-2)

**Subjects.** One `ln S` subject (scale `Log`) per deposit; one `ln(δ/g)` subject (scale `Linear`) per off-Earth deposit. `SubjectIdx` is assigned at worldgen in deposit-ID order.

**Prior.** A two-component mixture: the barren component (weight p_dry or 1 − p_exists, at the dry size) and the size distribution. It is the same distribution truth is drawn from, so beliefs are calibrated.

**Start (tick 0).** Known Earth deposits: every polity gets the prior plus one public observation (σ = 0.3, noise from `worldgen_stream(seed, deposit_id, P_PUBLIC)`, the same for all). Suspected: prior only. Unknown: `None` for everyone. Off Earth: `None`.

**Surveys** (`surveys.ron`). Each completed survey pushes one `Observation` per covered subject: value = truth × exp(N(0, σ)) in natural units, noise σ on the belief's scale (P-02e asks WS-02 to state this convention). Noise draws use stream `{tag: "wld", entity: polity << 16 | deposit, purpose}`.

| Survey | Covers | σ size / cost | Discovery | Prerequisite |
|---|---|---|---|---|
| Geological | every deposit in a region | 0.8 / — | p 0.5 per unknown deposit | own region or WS-06 access; offshore/arctic need the capability |
| Drilling | one deposit | 0.3 / — | — | belief exists |
| Telescopic | the sky, per year | — | generated bodies: p = 1 − exp(−(D/D50)²), D50 by observatory tier and class [D] | `cap.observatory` |
| Orbital | every candidate on a body | 0.9 / 0.8 | p 0.8 per candidate (creates the belief) | a spacecraft with a survey sensor at the body's hub for 6 consecutive months |
| In situ | one deposit | 0.4 / 0.3 | — | a lander at the Surface node for 3 months |
| Sample | one deposit | 0.15 / 0.1 | — | an outpost on the deposit, or a sample-return mission |

Space surveys run automatically while a qualifying spacecraft stays at the node: no order is needed. Discovery of a body is published to every polity after 12 months [D]; deposit beliefs stay private. Nothing here writes `Truth` (ARC-011).

### 4.9 Space operations (thin)

**4.9.1 Launch.** Launch sites are WS-11 facilities with `pads`, `cadence` (launches per pad per month) and a latitude (the region centroid). Launch vehicles are WS-05 designs (payload to LEO, reliability, unit cost) delivered into `lv_stock` at a site. A `Launch` uses one vehicle and one pad slot: monthly launches per site ≤ pads × cadence, and ≤ stock. The space-industry capacity limit (WS-03 ECO-015) bites through vehicle production, so it is not counted twice here.

**4.9.2 Missions.** A mission carries a payload: a spacecraft (a WS-11 unit with design Δv capacity and mass) or cargo (mass for supply, construction, propellant or munitions). The plan is a path from `plan_path`. Feasibility: a spacecraft's path Δv beyond LEO, including the latitude penalty, must not exceed its Δv stock (refuelled at depots on the path); cargo must not exceed the delivered-mass capacity (§4.9.3). State machine: `Planned(depart) → Underway{leg, arrive} → Arrived | Lost`. Each heliocentric leg waits for an open month. Each launch, interplanetary departure, capture and landing draws success with the design's reliability (stream `{tag: "spc", entity: mission, purpose: leg}`). Arrival sets `positions[u].node`.

**4.9.3 Logistics (Δv × mass).** Delivering cargo of mass m to node n needs LEO mass M = m·exp(Δv/v_e)/(1 − f_tug), with Δv measured from the nearest propellant source on the path, v_e the polity's best propulsion (WS-05: about 3.1 storable, 4.4 hydrolox, 8–9 nuclear thermal km/s [R]) and f_tug = 0.1 [D]. LEO is a source with unlimited propellant, fed by launches. A depot with stock is also a source: the part of the path beyond it draws propellant m·(exp(Δv_beyond/v_e) − 1) from depot stock.

**4.9.4 Facilities.** A facility is a template instance at a node: functions `Habitat{crew}`, `Extract{rate}`, `Depot{capacity}`, `Resupply`, `Sensor{survey_class}`. It is built when delivered construction mass ≥ the template mass. Monthly supply need = 0.5% of mass + 0.1 t per crew [D/R]. It is met first by local ISRU and then by cargo; `supply_fraction` (Tracked) is the EMA of met ÷ need, and output and readiness scale with it. Below 0.25 for 6 months the facility is abandoned (event; crew returns if a return vehicle is present, otherwise lost). Extractors produce volatiles into the depot at their node (propellant = 0.9 × water mass [D]) or metals for return to Earth (c_ret = return-logistics cost per unit via §4.9.3, with Earth aerobraking).

**4.9.5 Ownership off Earth.** A deposit is held by the first extractor established on it; later extractor orders for the same deposit are rejected, and the holder loses it only by destruction or by treaty (WS-06) (OQ-S3).

**4.9.6 Spacecraft resupply (closes the salvo note's deferral, §7.2/F-6).** A spacecraft refills its magazine and its Δv stock only while it is at the same node as a facility with the `Resupply` function that is owned by its polity, or by an ally with an access clause (WS-06), and that holds stock. Per tick it refills min(`reload_per_tick`, depot stock). Depots are stocked by cargo missions (munitions, propellant) or by ISRU (propellant). One pure function, `resupply_flows(world) -> Vec<Flow>`, computes the flows from stage-start state. WS-11 applies them to magazines and WS-08 to depot stocks and Δv, both in R5b, so each block keeps its one writer. Spacecraft with no such facility never refill (the salvo default).

**4.9.7 Debris.** Per node: D_n(t+1) = D_n·exp(−1/τ_r) + Σ_kills k_debris·mass_class. Hazard per unit per tick p = 1 − exp(−β_r·D_n), drawn per unit (stream `"spc"`, entity unit, purpose DEBRIS). Hits go to `space.hazards`, which WS-11 applies in R6b like engagement losses. τ_Low = 240 months; Medium, Synchronous and High = 6,000 months (effectively permanent) [D]. A Kessler-style ramp can emerge from repeated ASAT use in Low orbit.

**4.9.8 Engagement-generator inputs (to WS-07, salvo F-3).**
1. **Co-location:** engagements happen only between entities at the same node in the same tick; `Transit` entities can't be engaged.
2. **Reach rules** (`reach.ron`): `SameNode`; `SurfaceToOrbit { body, max_regime }` (direct-ascent ASAT from a ground battery: Earth Surface→Low early, →Medium as tech allows); `Adjacent { max_dv }` (co-orbital kill vehicle against a node one edge away, spending the attacker's Δv stock). WS-07 owns legality and Pk per munition class; WS-08b owns the vocabulary and adjacency.
3. **Maneuver as defense** (salvo G-7): the soft layer spends `dv_stock` (recorded in `engagements`; WS-08 applies it in R6b).
4. **Windows** don't constrain engagements: every planetocentric edge is always open.
5. **Facilities** are targets of class `facility` with the node's `orbit_regime`; damage reduces mass and functions, applied in R6b.

### 4.10 Macro tier (P-7, SYS-006)

Deposits, prices and rents run per deposit regardless of tier (they are world state), so resource endowment needs no reduced form. Missions and facilities of a demoted polity persist and run deterministically, but macro polities issue no new space orders. Earth-orbit activity is reduced to `MacroSpace { assets: [f64; 4] /* Low, Medium, Synchronous, High */ }`:

```
launches = space_program_real · φ_launch / c_launch(tech)
assets_r(t+1) = assets_r · (1 − 1/(12 L_r)) + launches · w_r(era)        L_r: Low 5 yr, others 10 yr [D]
```

Promotion: WS-11 creates stub constellation units per regime whose strength fractions sum to `assets_r`; WS-08 places them at `node.earth.<regime>`. Demotion: assets_r = Σ s_u over the polity's spacecraft at Earth-orbit nodes of that regime. Both are conserved within 1e-6 (ARC-081). Proposed: `Aggregates.orbital_assets` (Tracked) so WS-06 can compare space presence across tiers (P-02g).

### 4.11 Explainability and RNG

- `Tracked`: prices (world), world demand and capacity (world), debris per node (world), deposit output (`Owner::Deposit`), rent per polity with per-kind and per-deposit terms, facility supply fraction (`Owner::Facility`), macro assets. `Lazy`: spacecraft Δv stock (`Owner::Unit`). New `Owner` variants are proposed in P-02d.
- Worldgen streams: `worldgen_stream(seed, id, purpose)` with purposes `P_PHASE, P_COUNT, P_ORBIT, P_SIZE, P_COMP, P_TRUTH_SIZE, P_TRUTH_GRADE, P_TRUTH_DIFF, P_PUBLIC`. Runtime: tag `"wld"` (discoveries, survey noise), tag `"spc"` (launch and leg outcomes, debris hits). Every key includes the entity and, through the factory, the tick.

### 4.12 Performance and memory (SYS-008, ARC-091)

WS-08's budget is 5 ms per tick (WS-02 §5, shared with WS-08a events).

| Work | Estimate [D] |
|---|---|
| R2 resources: ≤ 129 deposits × ~30 flops, 3 price kinds × 47 polities | ≪ 0.1 ms |
| R5b: ≤ 200 missions, ≤ 100 facilities, ≤ 500 positions; `plan_path` memoized per (from, to, capability set) per tick; Dijkstra on ≤ 70 nodes / 884 edges ≈ 10 µs | ≤ 1.5 ms |
| R6b: engagement records touching space | ≪ 0.1 ms |
| **Total** | **≤ 2 ms**, leaving ≥ 3 ms for WS-08a events |

Memory: Sol graph ≈ 30 kB; deposit truth and state ≈ 15 kB; beliefs ≤ 192 subjects × 47 polities × ~80 B ≈ 0.72 MB. That is about 20% of the 3.6 MB WS-02 allots to beliefs, and about 19% of its 1,000-subjects-per-polity estimate.

## 5. Requirements

Verification: **P** = prototype check in `prototypes/ws08b-sol/sol.py` (now); **M2** = M2(d) batch run (§7.2); **M4** = `sim-core` test named `wld_NNN_*`; **R** = grader review.

| ID | Requirement | Acceptance criterion (testable) | Trace | Ver. |
|---|---|---|---|---|
| **Sol generation** |||||
| WLD-050 | Sol content loads and validates: every parent exists; gm > 0, radius > 0, a > 0; exactly one hub per body with nodes; node regimes are legal for the body (no Surface without a landable body; no `Transit` node); overrides reference existing nodes. | The default content loads. A fixture with a missing parent, two hubs on Mars, a `Transit` node and an override to `node.mars.medium` reports exactly those 4 errors (ARC-051). | ARC-050, ARC-051 | M4 |
| WLD-051 | Template values equal their cited sources. | Golden test: Earth gm = 398600.436, Mars a = 1.52371034 AU and L0 = −4.55343205°, Moon gm = 4902.800, Callisto a = 1,882,700 km, Ceres i = 10.59°, Pluto radius = 1188.3 km (exact). The canonical minor bodies (G-1a) are all present in every world. Every `bodies.ron` line with a number has a source tag in its comment. | ConOps §4 | M4, R |
| WLD-052 | The same seed gives an identical Sol variant, independent of unrelated content. | Two worldgens with seed 42: blake3 of the serialized Sol truth and graph equal. Adding a gen class `gen.test` leaves every other generated body, phase and deposit bit-identical (ARC-034 pattern). | P-1, SYS-002 | M4 |
| WLD-053 | Generated-body counts follow content. | Over 10⁴ seeds, each class's mean count is within 3 standard errors of E[min(Poisson(λ), max)], and no world exceeds Σ max = 18. Prototype (2,000 seeds): mean 10.21, max 17. | §4.1 G-3 | P, M4 |
| WLD-054 | Generated moons are plausible. | 10⁴ seeds: every generated moon has a within [lo, hi] × r_H, a > roche_factor × 2.44 R_p, and \|a − a_j\|/a_j ≥ min_rel_spacing for every other moon of its parent. Zero violations. | §4.1 G-4 | P, M4 |
| WLD-055 | Generated asteroids are plausible and follow the real populations. | 10⁴ seeds, zero violations of: every NEA lies in its drawn group's definition and has q < 1.3, a < 4.2 AU; vulcanoids have a ∈ [0.08, 0.21] AU and D ≤ 5.7 km; belt bodies lie in [2.1, 3.3] AU outside every gap band; Hildas have a ∈ [3.7, 4.2], e < 0.3, i ≤ 20°; trojans' mean longitude is within libration_deg of host ± 60°; every diameter lies in its class range. Distributions: NEA group shares within 0.02 of content; Jupiter-trojan L4 share within 0.02 of 0.615; belt stony share inside 2.5 AU exceeds that beyond 2.82 AU by ≥ 0.2. | §4.1 G-4 | P, M4 |
| WLD-056 | Graph size stays within the bench. | For 2,000 seeds: bodies ≤ 50, nodes ≤ 80, directed edges ≤ 1,000 (prototype: 49, 70, 884). | ARC-090, SYS-008 | P, M4 |
| WLD-057 | Generated bodies, and template bodies discovered after the start, begin unknown; other template bodies are public. | At tick 0, every polity's view lists every template body except Charon and no generated body. A generated body appears in a polity's view only after a telescopic discovery or the publication lag. With no survey at all, Charon appears in every view in 1978-01. | P-1, P-2, G-1a | M4 |
| **Graph and windows** |||||
| WLD-060 | One `OrbitRegime` enum (Surface, Low, Medium, Synchronous, High, Lagrange, Transit) serves nodes, positions and the salvo `orbit_regime`; node IDs follow `node.<body>.<regime>[_<label>]`. | Compile: WS-07's target type uses `OrbitRegime`. Test: Earth's nodes are exactly {surface, low, medium, synchronous, high, lagrange_eml1}; no node has `Transit`. | salvo §8.1 | M4 |
| WLD-061 | Orbit-change edges are Hohmann (circular) or one periapsis burn (ellipse). | LEO(300 km)→GEO = 3.8926 ± 0.0005 km/s, equal to the closed-form Hohmann value; LEO→HEO equals the single-burn formula within 1e-9. | §4.2 | P, M4 |
| WLD-062 | Heliocentric edges use Hohmann v∞ with Oberth burns at both ends. | LEO→LMO = 5.681 ± 0.002 km/s (3.590 departure + 2.091 capture); Earth→Mars months = 9. | §4.2 | P, M4 |
| WLD-063 | Moon edges follow §4.2. | LEO→LLO = 3.928 ± 0.002 km/s (TLI 3.106, LOI 0.821), within 5% of the 4.04 literature value. | §4.2 | P, M4 |
| WLD-064 | Surface edges use the override or the loss formula. | Earth surface→LEO = 9.4 (override); Moon surface→LLO = 1.862 ± 0.002 (formula), within 2% of 1.87; with an override removed, the edge uses the formula. | §4.2 | P, M4 |
| WLD-065 | Overrides replace or add edges. | EML1 edges equal the `transfers.ron` values in both directions; an override on a derived edge replaces it, and a `one_way` override leaves the reverse derived. | §4.2 | M4 |
| WLD-066 | `shortest_dv` matches a hand calculation and is deterministic. | Earth surface→Moon surface = 9.4 + 3.106 + 0.821 + 1.862 = 15.190 ± 0.003 km/s via earth.low and moon.low; Earth surface→Mars surface = 15.781 ± 0.003 (descent override 0.7). Results are identical when edge input order is shuffled. | ARC-015 | P, M4 |
| WLD-067 | The planner weighs time, and aerocapture by capability. | `plan_path(LEO, Ceres)`: w_t = 0 → via jupiter.high (9.706 km/s, 81 months); w_t = 0.02 → direct (10.158, 16). With `cap.aerocapture`, LEO→LMO = 3.740 ± 0.002; without it, 5.681. | §4.2 | P, M4 |
| WLD-068 | Relative inclination above 2° costs Δv at the cheaper end. | Earth↔Mars (1.85°): no plane-change term. LEO→Pallas ≥ 15 km/s (prototype 16.03). The term equals v_☉·Δi_eff added in quadrature within 1e-9 in a unit test. | §4.2 | P, M4 |
| WLD-069 | The launch-site latitude penalty applies to Synchronous targets only. | Penalty at 0° = 0 (within 1e-12); 28.5° = 0.363 ± 0.002; 45.9° = 0.804 ± 0.002 km/s; a Surface→Low→Medium plan gets no penalty. | §4.2 | P, M4 |
| WLD-070 | Windows reproduce the real Earth–Mars cadence. | With template phases, every month in {1960-10, 1962-11, 1964-11, 1969-02, 1971-05, 1973-07, 1975-08} [V: NASA/JPL launch dates; R: 1960-10 and 1973-07] is an open Earth→Mars month, and the optimum lies within ±1.5 months of each. The mean gap between optima = the synodic period 25.62 ± 0.05 months. | §4.3 | P, M4 |
| WLD-071 | The window penalty is quadratic, and departures wait outside windows. | Unit: Δv(m) equals the §4.3 formula within 1e-12 in the 3 open months; a mission planned in a closed month departs in the next open month, and its arrival = departure + months. | §4.3 | M4 |
| WLD-072 | Transit time is ⌈T_H⌉ months; planetocentric legs finish in the tick. | Earth→Mars 9, Earth→Jupiter 33 months; a LEO→LLO→Moon surface mission launched at tick t has `Arrived` at t. | §4.3 | P, M4 |
| WLD-073 | Windows are a pure function of truth and tick. | `window(edge, t)` evaluated twice, and after a save/load, returns identical values; no window data is stored in `World`. | P-4, ARC-061 | M4 |
| WLD-074 | Private resource equity is published per polity and matches the annuity formula. | Unit: V_d equals max(0, (1 − τ)(1 − σ)·12ρ_d)·a(T_d, r) within 1e-12, with T capped at 50; σ = 1 → V_res = 0; V_res,i = Σ V_d within 1e-9 rel (ARC-041 breakdown). Two-polity exporter test (WLD-085 setup): f_g +2σ held 24 months raises the exporter's V_res vs. f_g = 0. | §4.6, WS-03 §3.5 | M4 |
| **Resources** |||||
| WLD-075 | The taxonomy has exactly the four kinds, each with ≥ 1 mechanic, and the code names no kind. | Default content: 4 kinds. A fixture kind with `mechanics: []` is rejected. CI grep finds no `res.` ID in `crates/*/src/` (ARC-010). | D3, P-6 | M4 |
| WLD-076 | Resource tags follow the grammar. | The 66 default tags load. A fixture with `oil.giant`, `metals.supergiant` and `hydrocarbons.major.underwater` reports exactly 3 errors naming kind, size class and qualifier. A tag on a sea zone is rejected (WLD-010). | §4.4 | M4 |
| WLD-077 | Earth deposit truth follows the prior. | 10⁴ seeds, `hydrocarbons.giant`: barren share within p_dry ± 0.01; among the rest, mean ln S within 0.02 and σ within 3% of content. Deposit truth depends only on (seed, deposit ID) (ARC-034 pattern). | P-1, SYS-005 | M4 |
| WLD-078 | Off-Earth candidates follow `body_deposits.ron`. | Every real-body rule creates one candidate; a generated Metallic body gets a metals candidate whose median scales with D³; candidates ≤ max_candidates (85). | §4.4 | M4 |
| WLD-079 | Belief subjects stay within budget. | Default content, any seed: deposits ≤ 129 and subjects ≤ 192 (66 + 2 × 63). | §4.12, WS-02 §5 | P, M4 |
| WLD-080 | Start beliefs follow knowledge qualifiers. | At tick 0: a `known` deposit has `Some` for all 47 polities with an identical belief; a `suspected` one has the prior for all; an `unknown` one (West Siberia) is `None` for all; every off-Earth subject is `None`. | §4.8 | M4 |
| **Price and rent hook** |||||
| WLD-081 | The price follows §4.5, with breakdown base + global factor + scarcity summing exactly. | Unit: given inputs, ln p within 1e-12; ARC-041 sum check. f_g = 0 and D = Ŝ held: p = p̄ for 120 months. Spectral check: a step in D converges monotonically with ratio 1 − 2/τ_p ± 1e-9 per month (linear regime). | §4.5, P-3 | M4 |
| WLD-082 | Production obeys capacity, shut-in and depletion. | Unit: q = σ(m/(0.25cδ))·q̂ within 1e-12; over a 1,260-tick run, cum_d ≤ S_d and S_d − cum_d never negative; with m = 0, u = 0.5. | §4.6 | M4 |
| WLD-083 | Rent per deposit and per polity is q·margin, with breakdowns. | ρ_i = Σ ρ_d · e_i/P_i within 1e-9 rel; the polity rent breakdown has one term per owned producing deposit and sums exactly (ARC-041). | §4.6, SYS-004 | M4 |
| WLD-084 | The rent split is exact. | 10⁴ random (τ, σ, K): royalty + SOE + diverted + private = ρ within 1e-12 rel; K = 0 → diverted = 0; σ = 1, τ = 0 → private = 0. | §4.6 | M4 |
| WLD-085 | A positive f_g is a windfall for exporters and a loss for importers. | Two-polity world (exporter: one `hydrocarbons.giant.easy` at dev 1; importer: none), shocks off except f_g held at +2σ for 24 months vs f_g = 0, same seed: the exporter's rent, GDP, treasury revenue and FX reserves are all higher; the importer's NX and reserves are lower and its π is higher. Signs hold in 200/200 seeds with shocks on. | §4.7, WS-03 §3.3 | M2, M4 |
| WLD-086 | Commodity flows conserve and reach WS-03. | Σ_i n_k,i = 0 within 1e-9 rel every month; WS-03's NX and reserve breakdowns contain a commodity term equal to n·p per polity. | §4.7 | M4 |
| WLD-087 | Development ramps after capex, priced on belief, gated by capability. | `DevelopDeposit` on an offshore deposit without `cap.offshore` is rejected; with it, dev = 0 until capex completes, then rises linearly to 1 over `dev_months`; capex equals the formula using the belief mean of S (not truth) within 1e-9. | §4.6, P-1 | M4 |
| WLD-088 | Ownership changes and capital-loss shocks move rents and capacity. | A region transferred at tick t: from t + 1 its deposits' rents go to the new owner, with `dev` unchanged. Unowned regions produce 0. A WS-04 `capital_loss` of 0.3 at tick t gives every deposit of that polity `dev` × 0.7 at t (within 1e-12), and no other polity's deposits change. | §4.6, WS-04 §3.3 | M4 |
| WLD-089 | WS-03 stability holds with resources on. | M2(d): every WS-03 §6.1 row passes for archetypes A, B, C with the default Earth tags, plus the proposed exporter archetype D rows (P-03h); zero `ClampHit`. | R-2, ECO-130 | M2 |
| **Prospecting** |||||
| WLD-090 | Surveys push observations with the content σ, and beliefs converge. | A drilling survey pushes one observation with noise_sigma = 0.3. 20 successive drilling surveys on one deposit: belief σ(ln S) < 0.1 and \|mean − ln S_true\| < 3σ in ≥ 95% of 500 seeds. A barren deposit's dry-component weight exceeds 0.9 after 3 drilling surveys in ≥ 90% of seeds. | P-2, ARC-013 | M4 |
| WLD-091 | Discovery creates beliefs and never touches truth. | A geological survey of the British Isles with `cap.offshore` discovers its unknown deposit with frequency 0.5 ± 0.03 over 2,000 seeds; without the capability, 0. `blake3(truth)` is identical before and after (ARC-011). | P-1 | M4 |
| WLD-092 | Space surveys run by presence. | A spacecraft with an orbital sensor at `node.moon.low` for 6 consecutive months creates Moon candidate beliefs (p 0.8 each) and pushes observations with σ (0.9, 0.8); leaving after 5 months pushes none. | §4.8 | M4 |
| **Space operations** |||||
| WLD-093 | Launches are limited by pads, cadence and stock. | A site with 2 pads × 1/month and 5 vehicles in stock: the 3rd launch order in a month is rejected with an event; after 3 months the stock is 0 and further orders are rejected. | §4.9.1 | M4 |
| WLD-094 | Missions are feasible, wait for windows, and draw failures. | A spacecraft with Δv stock 5.0 ordered to LMO (needs 5.681) is rejected; with 6.0 it launches, departs in an open month and arrives 9 months later, with stock reduced by the path Δv. With reliability 0.9 per maneuver, the success frequency of a 3-maneuver mission over 10⁴ seeds = 0.729 ± 0.01. | §4.9.2 | M4 |
| WLD-095 | Logistics follows the mass ratio, and depots split the path. | Cargo of 1 t to LLO with v_e = 4.4 on the direct path: LEO mass = exp(3.928/4.4)/0.9 within 1e-9 rel; on the path via a stocked depot at EML1, LEO mass = exp(3.77/4.4)/0.9 and the depot loses exp(0.64/4.4) − 1 t of propellant. | §4.9.3, D8 | M4 |
| WLD-096 | Facilities build, need supply, and are abandoned when unsupplied. | A template of 20 t completes the tick delivered mass reaches 20 t. With 0 deliveries and no ISRU, supply_fraction falls below 0.25 and the facility is abandoned exactly 6 ticks later, with an event. An extractor on a volatiles deposit adds 0.9 × output to its depot each month. | §4.9.4 | M4 |
| WLD-097 | The spacecraft resupply rule holds, with one writer per block. | A spacecraft at a node with an allied `Resupply` depot (access clause) refills min(reload_per_tick, stock) and the depot loses the same amount within 1e-12. With no access clause, or the depot at another node, nothing moves. WS-11's magazine delta equals WS-08's depot delta. | §4.9.6, salvo §7.2 | M4 |
| WLD-098 | Engagement inputs, debris and the macro tier follow §4.9.8, §4.9.7 and §4.10. | (a) Two hostile spacecraft at `node.earth.low` form one co-location group; a Transit unit forms none; a ground DA-ASAT battery gets `node.earth.low` in reach and not `node.earth.synchronous` without the tech. (b) With D_n set so that β·D = 0.1, the debris-loss frequency over 10⁴ unit-ticks = 1 − e^(−0.1) ± 0.01; D in Low halves in 240·ln 2 months with no kills. (c) Promote then demote a macro polity: assets per regime conserved within 1e-6 (ARC-081). | §4.9, P-7, SYS-006 | M4 |
| WLD-099 | WS-08b is deterministic, explainable and within budget. | Two 1,260-tick runs with the same seed and inputs give identical hashes (ARC-060). Every WS-08b `Tracked`/`Lazy` passes the ARC-041 sum check over a 120-tick run. `bench`: R2 + R5b + R6b WS-08b median ≤ 2 ms on the reference machine. | SYS-002, SYS-004, SYS-008 | M4 |

WLD-058 and WLD-059 are reserved.

## 6. Content/data

| File | Schema (`deny_unknown_fields`) | Draft entries |
|---|---|---|
| `maps/sol/bodies.ron` | `Body { id, name, parent: Option, class: Star\|Planet\|DwarfPlanet\|Moon\|Asteroid, gm, radius_km, orbit: Option<Orbit { a: Au(f64)\|Km(f64), inc_deg, mean_lon: Option<(f64, f64)> }>, composition: Rocky\|Icy\|Gas\|Carbonaceous\|Stony\|Metallic, atmosphere: Airless\|Thin\|Thick, aerocapture: bool = false, homeworld: bool = false, discovered: Option<i32> = None, nodes: [Node { regime, alt_km = 0, apo_alt_km: Option, hub = false, label: Option, ascent_dv: Option, descent_dv: Option }] }` | 32 bodies, 53 nodes |
| `maps/sol/transfers.ron` | `Edge { from, to, dv, months, source, one_way = false }` | 3 (EML1) |
| `maps/sol/gen_rules.ron` | `GenRules { classes: [GenClass { id, max, count_mean, parent, class, orbit: Belt { a_au, gaps_au, ecc_rayleigh, ecc_max?, inc_max_deg? } \| NeoGroups([(group, share)]) \| Trojan { of, l4_share, libration_deg } \| Moon { hill_fraction, roche_factor, min_rel_spacing }, inc_rayleigh_deg, diameter_km, size_slope, composition: Fixed([(Composition, w)]) \| Gradient { a_au, stony, metallic } }], densities_g_cm3 }` | 9 classes |
| `maps/earth/resources.ron` | `[(RegionId, [tag])]` | 46 regions, 66 tags |
| `core/resources/kinds.ron` | `ResourceKind { id, name, unit, mechanics, on_earth, off_earth, price_ref, cost_ref, fg_loading, demand_elasticity, intensity_decline, extraction_rate, dev_months, capex_months }` | 4 |
| `core/resources/size_classes.ron` | `SizeClasses { classes: [SizeClass { kind, id, median, sigma_ln, p_dry, grade_sigma }], dry_fraction, qualifiers: [Qualifier { id, difficulty, requires }], knowledge }` | 10 classes, 3 location qualifiers |
| `core/resources/body_deposits.ron` | `BodyDeposits { by_body, by_composition, composition_scale, max_candidates }` | 27 + 6 rules |
| `core/resources/surveys.ron` | `Survey { id, target: Region\|Deposit\|Sky\|Body, requires, months, cost, sigma_size, sigma_cost, p_discover, note }` | 6 |
| `core/space/facilities.ron` (M4) | `FacilityTemplate { id, regimes, mass_t, crew, functions, upkeep_frac }` | — |
| `core/space/reach.ron` (M4) | `ReachRule { munition_class, rule: SameNode\|SurfaceToOrbit{body, max_regime}\|Adjacent{max_dv} }` | — |
| `settings/<s>/deposit_start.ron` | `[(DepositId, dev)]` | — (OQ-S5) |

All §4 defaults live in content; the code holds formulas only (ARC-010).

## 7. Verification

### 7.1 Prototype results (`python prototypes/ws08b-sol/sol.py`, 2026-11-27) [V: computed]

| Check | Result |
|---|---|
| Template graph | 32 bodies, 53 nodes, 256 directed edges |
| LEO→GEO | 3.8926 (equals the hand Hohmann value) |
| LEO→LLO / Moon surface→LLO | 3.928 (TLI 3.106 + LOI 0.821) / 1.862 |
| Earth surface→Moon surface | 15.190 |
| LEO→LMO | 5.681 (3.590 + 2.091); Earth–Mars T_H 8.51 months, synodic 25.62 |
| LEO→Ceres | direct 10.158 / 16 mo; via Jupiter 9.706 / 81 mo |
| Latitude penalty (0, 5.2, 28.5, 45.9, 62.9°) | 0, 0.014, 0.363, 0.804, 1.278 |
| Mars windows vs. 7 launches 1960–75 | all inside open months; max offset 1.36 months (Mars 1960A) |
| 2,000 seeds | generated mean 10.21, max 17; 0 violations; max 49 bodies, 70 nodes, 884 edges; NEA groups 40.2/55.2/3.4/1.2%; Jupiter-trojan L4 share 0.627; belt stony share 0.62 (inner) vs. 0.29 (outer) |
| Canonical | Pluto–Charon edges internal only (isolated); LEO→a sample Jupiter trojan 11.97 km/s, 33 months |
| Subjects | Earth 66 + off Earth ≤ 2 × 63 = 192 |

The prototype uses PCG64; the Rust port must match the formulas exactly and the distributions statistically (the same convention as WS-03's M2(a)). The window check uses circular orbits (G-6), so the real 1–1.4-month offsets come from eccentricity and from launch periods that spread across several weeks.

### 7.2 M2(d) plan: rent hook and price block

`prototypes/ws08b-sol/README.md`. In short: add `resources.py` to the M2(a) economy prototype (vectorized over runs), run WS-03 §6.1 with resources on, add exporter archetype D, and run three scenarios: S11 (oil shocks 1973 and 1979, now via p_k), S22 (price collapse 1986) and S23 (discovery windfall: North Sea type). Exit: WLD-085 and WLD-089 pass, and world rents/GDP stay inside the World Bank historical band of ≈ 1–8% [R]. Estimate 14 h [D].

### 7.3 M4

Tests named `wld_0NN_*` in `sim-core`, one or more per requirement. The test setting (WS-02 §6) needs ≥ 1 moon and 1 small body for WLD-063/066; proposed in P-02f.

## 8. Proposed edits to other documents (not made)

| ID | Document | Edit | Why |
|---|---|---|---|
| P-02a | WS-02 §3.2 | `sol: Graph<Body, Transfer>` → `sol: Sol { bodies, graph: Graph<SolNode, Transfer> }`; add `NodeId` | Nodes are (body, regime), not bodies (D1) |
| P-02b | WS-02 §3.2 | `World` gains `resources: ResourceState` and `space: SpaceState` (world-level, WS-08 writer) | D6 |
| P-02c | WS-02 §3.3 | R2 adds WS-08 `resources`; R5b and R6b add WS-08 `space`; R6 drops "space ops (WS-08)". Budget row unchanged | Combat sees this tick's positions (D6) |
| P-02d | WS-02 §3.5 | `Owner` gains `Deposit(DepositId)`, `Facility(FacilityId)`, `Node(NodeId)` | §4.11 |
| P-02e | WS-02 §3.2 / ADR-0008 | State that `Observation.value` is in natural units and `noise_sigma` is on the belief's scale | §4.8 relies on it |
| P-02f | WS-02 §6 | Add the §3.1 content files; the test setting's 4 bodies include a moon and a small body | WLD-063/066 |
| P-02g | WS-02 §3.8 | Optional `Aggregates.orbital_assets` (Tracked) | Cross-tier comparison of space presence (§4.10) |
| P-03a | WS-03 §3.1 | Y_ext gains an additive rent term ρ_i with per-kind breakdown terms | D4 |
| P-03b | WS-03 §3.5, §5 | `TaxKind resource_royalty` with base `ResourceRent`; SOE take uses `state_ownership` of agriculture & extraction | §4.6 split |
| P-03c | WS-03 §3.6 | Revenue lines: royalty, SOE resource take; spending lines `resource_development` (capex) and `exploration` (surveys) | §3.4, §4.6 |
| P-03d | WS-03 §3.6, §2.2 | Diverted resource rents are added to the corruption-rents amount exported to WS-04, never to key income | Rentier channel, no double count |
| P-03e | WS-03 §3.10, §3.4 | Commodity net positions n·p enter NX and FX reserves; the import-price term weights commodity prices by their import share | §4.7 |
| P-03f | WS-03 §3.3 | `f_g` stays a WS-03 draw; the λ loading becomes the non-commodity world shock, default 0.3 (was 0.5), tuned in M2(d) | Avoids double counting (OQ-S2) |
| P-03g | WS-03 §3.13 | The macro ln y equation applies to non-resource output; y = y_nr + rent; pb includes the state's take | Macro rents |
| P-03h | WS-03 §6.1, §6.2 | Exporter archetype D rows; S11 via p_k; new S22 (1986 price collapse: exporter deficit widens ≥ 3 pp of GDP within 2 years in ≥ 70% of runs) and S23 (discovery windfall) | WLD-085, WLD-089 |
| P-03i | WS-03 §2.3 | Inputs table gains the WS-08 row (`ResourceIncome`, `V_res`, net positions, capex) | Interface completeness |
| P-03j | WS-03 §3.5 | Add WS-08b's `V_res` to W_hh as an equity line, held through the groups' equity shares e_g, with ΔV_res in Gain_g and the private rent flow in the panel's capital income for agriculture & extraction equity holders; φ_off applies. Alternative if WS-03 prefers: fold the rent into the agriculture & extraction MPK_net that drives q_E (simpler, but it then raises every sector's equity price) | §4.6; private rents reach the v0.3 wealth model |
| P-03k | WS-03 §3.5 | `resource_royalty` is a firm-level tax on the rent base, outside the per-household panel (like the SOE line); the monthly discretion band (WS-04 GOV-066) applies to it like any tax | v0.3 taxes are per household |
| P-04a | WS-04 §6.4, `policy_map.ron` | Three rows (GOV-021): royalty rate (tax) → `incidence`, Economy +● (a higher state take is the cooperative pole, like state ownership); `resource_development` line → `incidence`, Economy +●; `exploration` line → `public (v 0.5)`, Economy +● (survey results are a public good to the owner). Missions, launches and facilities add no instrument: they are paid from the existing space-program line (Space +●●●) | GOV-021 validation |
| P-04b | WS-04 §4.4 | The private-goods breakdown labels the resource-diversion part of Rents separately | Legibility (P-3) |
| P-04c | WS-04 §6.2 | Industrialists' membership includes Capital keys of agriculture & extraction (today: industry, defense, space) | Resource owners, P-8 |
| P-04d | WS-04 §3.4 | `gate_orders` also covers WS-08b's `Survey` and `DevelopDeposit` (budget-committing; PC cost 0 by default [D]) | One gate (GOV-064) |
| P-04e | WS-04 §7, OQ-G8 | OQ-G8 asked to revisit the Space axis when WS-08b adds resources. WS-08b's answer: keep **earthbound / expansionist**. WS-08b adds rents but no environment state, so an extraction/ecocentric axis would still have nothing to act on. Resource politics already reaches factions through incidence (rents, royalty), Spoils (diverted rents) and wealth clout (V_res). Revisit with a future environment model | Closes the revisit |
| P-05a | WS-05 | Capability flags (`cap.offshore`, `cap.arctic`, `cap.observatory` tiers, `cap.orbital_spectrometry`, `cap.lander`, `cap.aerocapture`, `cap.isru`); propulsion v_e by tech; launch-vehicle and spacecraft design stats (payload to LEO, reliability, Δv capacity, dry mass, survey sensor class); deliveries to sites | §4.8, §4.9 |
| P-05b | WS-05 / WS-11 | Use the fissile self-sufficiency index as a nuclear-program cost multiplier (form chosen there) | NuclearFeed mechanic |
| P-06a | WS-06 | Treaty clauses: survey access, facility access (allied resupply), off-Earth extraction rights (an Outer Space Treaty-style hook); discovery publication; intel observations of others' missions and facilities | §4.8, §4.9 |
| P-07a | WS-07 | The engagement generator uses `space.positions`, the co-location rule, `reach.ron` and adjacency Δv; Transit is never engaged; maneuver spends Δv stock; kills report a mass class for debris; space facilities are `facility` targets with `orbit_regime` | §4.9.8, salvo F-3, G-7 |
| P-11a | WS-11 | Launch sites expose pads, cadence, max vehicle class and latitude; ground stations | §4.9.1 |
| P-11b | WS-11 | A spacecraft's position and Δv stock are WS-08 state; WS-11 owns the rest of the unit; a delivered spacecraft sits at its launch site | D6 |
| P-11c | WS-11 | Spacecraft `reload: at_depot` per §4.9.6, through the shared `resupply_flows` | Salvo §7.2 |
| P-11d | WS-11 | Macro promotion creates stub constellations conserving `MacroSpace.assets`; R6b applies `space.hazards` like engagement losses | §4.9.7, §4.10 |
| P-08a1 | WS-08a §3.2, WLD-047 | `resource_tags`: "WS-03 vocabulary" → "WS-08b vocabulary (WS-08b §4.4)". If OQ-S1 is accepted, remove the reserved field in favor of `maps/earth/resources.ron` | The vocabulary lives here |
| P-08a2 | WS-08a content | Add `settings/date/deposit_start.ron` (1955 development levels of known deposits) | OQ-S5 |
| P-SAL | salvo note §8.1, §7.2, §11 | `orbit_regime` = WS-08b `OrbitRegime`; spacecraft refill = WS-08b §4.9.6; G-7 closed by §4.9.7–4.9.8 | Shared vocabulary |
| P-CON | ConOps §6 space row | "Hohmann Δv graph over orbit-regime nodes, synodic windows, world commodity prices and resource rents (WS-03 hook), prospecting (P-2), outposts and stations" | Reflects D1, D4, D9 |

### 8.1 What each workstream needs to do or confirm (as of WS-03 v0.3, WS-04 v0.3)

| Workstream | Needed | Blocking? |
|---|---|---|
| WS-02 | Accept and apply P-02a–P-02g: `Sol` node graph type, the `resources` and `space` world blocks, WS-08 writers in R2, R5b and R6b (none in R6), `Owner::{Deposit, Facility, Node}`, the `Observation` units convention, test-setting bodies, optional `orbital_assets`. These change WS-02's approved v1.1, so they need your sign-off | Yes for M4; not for approving WS-08b |
| WS-03 | Accept P-03a–P-03k. Two need its judgment: **P-03j** (explicit resource-equity line vs. folding the rent into q_E) and **P-03f** (residual λ 0.5 → 0.3, confirmed in M2(d)). It should also confirm that S22/S23 and archetype D are free, and that royalty sits outside the panel (P-03k) | P-03j decides how WLD-074 is consumed; the rest can follow approval |
| WS-04 | Add the three P-04a policy-map rows (GOV-021 fails without them once WS-08b content loads), extend `gate_orders` (P-04d), add Industrialists membership (P-04c), the breakdown label (P-04b), and record the OQ-G8 answer (P-04e) | P-04a and P-04d before M4 integration |
| WS-08a | Remove the reserved `Region.resource_tags` field (OQ-S1, P-08a1) and own `settings/date/deposit_start.ron` (OQ-S5, P-08a2; values drafted in M2(d)) | No |
| WS-05, WS-06, WS-07, WS-11 | Carry P-05a/b, P-06a, P-07a and P-11a–d into their kickoffs | No |

Nothing from another workstream is needed to approve WS-08b itself: every cross-workstream item is a proposal, and approving the spec means accepting §8 for each owner to apply.

## 9. Open questions

All five closed 2026-11-27 (Sean: accept the recommendations).

| ID | Question | Resolution |
|---|---|---|
| OQ-S1 | Where should Earth resource tags live: a separate layer `maps/earth/resources.ron` (drafted), or WS-08a's reserved `Region.resource_tags`? | **Closed: Separate layer.** Geology can be swapped or tuned without touching the region file, and WS-08a's file stays as approved. Drop the reserved field (P-08a1). |
| OQ-S2 | How should `f_g` reach economies: explicit commodity channels plus a smaller residual λ (§4.7), or keep WS-03's uniform loading and add rents on top? | **Closed: Explicit channels, λ 0.3 residual.** The alternative double counts and gives exporters the wrong-signed demand shock. |
| OQ-S3 | Off-Earth extraction rights: first extractor holds the deposit (contested by force or treaty), or a claims registry? | **Closed: First extractor**; WS-06 treaty clauses can override it. |
| OQ-S4 | Small-body phases: sample per world (real a and i, seeded phase), or real ephemerides for every body? | **Closed: Sample.** It makes asteroid timing part of the variant; planets keep real phases, so Mars and Venus windows stay historical. |
| OQ-S5 | 1955 development of known deposits: a setting file (`settings/date/deposit_start.ron`, WS-08a data), or one default level for every known deposit? | **Closed: Setting file.** Gulf production in 1955 was a small fraction of its later capacity; one default would make 1955 output about 3× too high or too low by region. M2(d) drafts the values. |

**Defaults taken (no question needed):** aerocapture trim 0.15 km/s; the launch-site latitude penalty is in the MVP; Neptune is unreachable (the 240-month cap; gravity assists are post-MVP); Venus has no surface node; Earth cost ratios are known at discovery (D7); facilities are templates, not designs; macro polities run no deep-space missions.

## 10. Sources

- JPL SSD, *Approximate Positions of the Planets*, Table 1 (1800–2050): planet a, I, L, L-rate. https://ssd.jpl.nasa.gov/planets/approx_pos.html [V]
- JPL SSD, *Planetary Physical Parameters*: radii, masses (GM = mass × G, CODATA 2018, as NSSDC notes). https://ssd.jpl.nasa.gov/planets/phys_par.html [V]
- JPL SSD, *Planetary Satellite Physical Parameters*: satellite GM and radius; planet GM. https://ssd.jpl.nasa.gov/sats/phys_par [V]
- JPL SSD, *Planetary Satellite Mean Elements*: satellite a and i. https://ssd.jpl.nasa.gov/sats/elem/sep.html [V]
- NASA NSSDC, *Asteroid Fact Sheet*: Ceres, Pallas, Vesta a, i, mass, dimensions. https://nssdc.gsfc.nasa.gov/planetary/factsheet/asteroidfact.html [V]
- IMCCE, *Orbital parameters of the brightest asteroids*: Psyche a, i. https://promenade.imcce.fr/en/pages5/570.html [V]
- PDS Small Bodies Node, 433 Eros: a, e, i, mass. https://arcnav.psi.edu/urn:nasa:pds:context:target:asteroid.433_eros [V]
- Wikipedia, *Delta-v budget*: Earth–Moon high-thrust table; Earth and Mars ascent. https://en.wikipedia.org/wiki/Delta-v_budget [R, secondary]
- Rodrigues et al., *Connecting Earth and Moon via the L1 Lagrangian point*, arXiv:2502.11694: EML1→LLO 0.648 km/s. https://arxiv.org/pdf/2502.11694 [V]
- NASA JPL, *NASA's legacy of Mars exploration*: Mars 1 (1962-11-01), Mariner 4 (1964-11-28), Mariner 9 (1971-05-30). https://www.jpl.nasa.gov/news/nasas-legacy-of-mars-exploration/ [V]
- NASA, Viking 1 launch 1975-08-20. https://www.nasa.gov/?p=168404 [V]; NASA science, Mariner 6 launch 1969-02-25. https://science.nasa.gov/mission/mariner-program [V]
- Steffl et al., *A Search for Vulcanoids with the STEREO Heliospheric Imager*, arXiv:1301.3804: no vulcanoids > 5.7 km (3σ). https://arxiv.org/pdf/1301.3804 [V]
- Stern & Durda, *Collisional evolution in the Vulcanoid region*, arXiv:astro-ph/9911249: stability limit near 0.21 AU. https://arxiv.org/pdf/astro-ph/9911249 [V]
- Granvik et al., *Debiased orbit and absolute-magnitude distributions for near-Earth objects*, Icarus 312 (2018), arXiv:1804.10265: NEO group shares, NEO definition. https://ar5iv.labs.arxiv.org/html/1804.10265 [V]
- Li et al., *Asymmetry in the number of L4 and L5 Jupiter Trojans driven by jumping Jupiter*, A&A 2023 (citing Szabó et al. 2007, N4/N5 = 1.6 ± 0.1). https://www.aanda.org/10.1051/0004-6361/202244443 [V]
- *Clues for Solar System Formation from Meteorites and their Parent Bodies*, arXiv:2506.02721: S/E-types dominate the inner belt, C/D-types further out. https://arxiv.org/pdf/2506.02721 [V]
- Wikipedia, *Jupiter trojan* (14 known in 1961) and *Hilda asteroid* (a 3.7–4.2 AU, e < 0.3, i < 20°). https://Wikipedia.com/wiki/Jupiter_trojan, https://Www.Wikipedia.org/wiki/Hilda_group [R, secondary]
- World Bank WDI, *Total natural resources rents (% of GDP)*, method: price minus average cost, times quantity. https://data.worldbank.org/indicator/NY.GDP.TOTL.RT.ZS; method text via Our World in Data, https://archive.ourworldindata.org/20260304-094028/grapher/natural-resource-rents.html [V]
- Recalled, to verify in M2(d) [R]: Pluto mean elements (older JPL table); Charon a 19,591 km and discovery 1978; Hygiea mass and radius; GM_☉ 1.32712440018e11; CODATA 2018 G; mid-1950s Gulf posted crude ≈ $1.9/bbl; copper ≈ $0.43/lb; 1955 world oil output ≈ 5.6 Gboe/yr; North Sea discovery 1969 and first oil 1975; lunar north-pole ice ≈ 6e8 t; Molniya apogee 39,700 km; NTR v_e 8–9 km/s; Kirkwood gap locations; the CNEOS NEO definition.
