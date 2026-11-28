# WS-11 — Force Design, Manpower, Basing, Munitions & Strategic Forces
Status: Draft v0.1 · Parent: ConOps §3 (force design, manpower & readiness, basing, munitions, strategic forces rows), §5 (P-2, P-3, P-4, P-6, P-7, P-8, P-10), §6 and §6.1, R-6, SYS-002, SYS-004, SYS-006, SYS-007, SYS-008, SYS-009 · Depends on: WS-02 v1.3 (§3.2, §3.3 R5b/R6b, §3.5 `Lazy`, §3.8, ARC-024, ARC-044, ARC-081, ARC-090), WS-03 Draft v0.4 (§2.2–2.3, §3.6, §3.9, §3.13), WS-04 Draft v0.4 (§3.1, §3.3, §4.2, §6.3), WS-05 Draft v0.4 (§3.6, §4.6–4.8, §4.10; P-11a/b), WS-08a Draft v0.3 (§3.2, §3.5), WS-08b Draft v0.4 (§3.2–3.5, §4.9–4.10; P-11a–d), salvo note v0.3 (§7–§8, §11–§12) · Last updated: 2026-11-28

Tags: **[V]** verified (source linked, or computed and reproducible) · **[R]** recalled, not checked · **[D]** design choice (to be tuned in the M2(e) prototype, §7).

Requirement prefix: `FRC`. Real-world orders of battle are inspiration for presets only; every template in the setting is fictional (ADR-0003).

---

## 0. Design intent: what the system should teach

The player runs a defense ministry, not a battle. The lessons the model must produce without scripts:

1. **Forces are built years before they fight.** Equipment comes from WS-05 production lines with multi-year lead times, pilots take two years to train, and reserves take months to mobilize. "When the invasion begins, the army you built is the army you fight with" is War Footing's premise [V: [War Footing store page](https://store.steampowered.com/app/4727860/)], and it is ours.
2. **Upkeep is the budget.** O&S and personnel cost more than procurement over a unit's life (WS-05 §4.7.4). An unfunded force decays into a paper force: units still on the map, readiness falling month by month.
3. **Manpower is labor.** Every soldier is withdrawn from WS-03's labor force. Conscription is cheap in pay and expensive in output; a volunteer force is the reverse and has no deep reserve.
4. **Structure is a choice.** Templates are customizable, research unlocks new slots and doctrines, and a refit costs experience. There is no best template, only trade-offs among firepower, upkeep, supply need and mobility.
5. **Deterrence is a perception.** Second-strike capability matters through what rivals believe about it (P-2), so ISR and secrecy matter.

## 1. Purpose & scope

WS-11 owns the `force` block (WS-02 §3.2): formation templates and their rollup into unit aggregates, tracked units and their lifecycle, groupings and deployment, manpower, readiness, bases, munitions stockpiles, supply capacity, strategic forces and ISR assets, for both fidelity tiers.

**In (MVP):** everything in the ConOps §3 force rows; the WS-07 per-unit input contract (salvo note §8); start-state mapping for the WS-08a `forces` and `tech.nuclear` fields.

**Out:** tactical combat and engagement resolution (WS-07); generals and leaders; tracked division and corps HQs; detailed logistics networks (supply is capacity along region edges, §4.9); nuclear exchange modeling (use is an event, §4.11); cyber and EW beyond the `ew` modifier; player-authored doctrine (doctrines are content options, §4.2.4); unit-level loyalty during coups (OQ-F6); orbital movement and Δv (WS-08b owns them).

## 2. Decisions

| # | Decision | Recommendation |
|---|---|---|
| 1 | Template schema and rollup | **Templates are trees of content element types (company level) and sub-templates, with equipment slots keyed by role (WS-05 hull class or a kit class).** A unit holds one fill per role (design, authorized count, on-hand count). Aggregates are a pure, cached function of (template version, fills, design stats, doctrine), with one breakdown term per role. Five field operators only: `sum`, `max`, `min`, `mean` (count-weighted) and `merge` (mounts and layers by key). |
| 2 | OQ-6 (force block placement, loss path) | **Already closed in WS-02 v1.0** (§3.3, ARC-024): WS-11 writes `force` in R5b and R6b only; WS-07 writes `engagements` only. This spec adds the R6b rules (§4.8) and the engagement record fields WS-11 needs (P-07b). |
| 3 | Aggregate set for WS-07 | **`UnitCombatView`**, the salvo §8.1–8.3 fields plus `GroundStats` (Lanchester), multipliers kept separate (`s`, readiness, supply, experience). The six P-10 headline scalars (offense, defense, staying power, detection, mobility, upkeep) are summaries of the same rollup for UI and AI. ConOps edit P-CON-F1 records this (salvo T-2). |
| 4 | Manpower and mobilization | **Polity-level pools (pipeline, unassigned, conscripts by month of service, trained reserve) per personnel class; units hold counts and a conscript share.** Four unit statuses (Active, Reserve, Cadre, Mothballed) with readiness caps and lags. Age and sex enter through content profiles, not per-soldier state. |
| 5 | Era presets | **~30 template lines in 7 services, each with 1–5 versions across five eras (≈ 75 versions),** plus ≈ 40 element types and ≈ 12 doctrines. Research unlocks versions, slots and doctrines through `requires` lists; the player may also define custom templates from unlocked elements (§4.2.3, §6.4). |

### 2.1 Options and trade-offs (condensed)

**D1 — Template structure.**
(a) Flat HOI4-style: a brigade is a list of battalion types. Simple, but "design from company level" fails, and the battalion is the only lever.
(b) **Tree of element types and sub-templates, roles at the leaves.** Company-level building blocks, battalion and regiment sub-templates, brigade root. Matches ConOps §6.1, and legibility is kept by rolling up by role rather than by tree node.
(c) Equipment-level free composition (each vehicle placed by the player). Maximum freedom; thousands of decisions; R-6 fails.
→ (b). A tank company is "14 of role `hull.ground.mbt` + 70 technical personnel"; a battalion is "3 tank companies + 1 HQ company". The tree defines authorized counts; after flattening, the unit holds one fill per role (≈ 6–10 per brigade), so the state and the breakdown stay small.

**D1b — What a slot references.** A slot names a **role** (a WS-05 hull class, e.g. `hull.ground.mbt`, or a kit class), not a design. The concrete design is chosen per unit at raise or refit. This matches HOI4's equipment-variant split [R], keeps templates valid across decades, and makes "refit" mean "change the design in a role".

**D1c — Rollup operators.** One general expression language was rejected: illegible and untestable. Five operators cover every field in salvo §8 (table in §4.3). Each produces a `Breakdown` with one term per role, plus doctrine and template-rule terms, so every aggregate answers "why" in ≤ 12 terms.

**D3 — Offense and defense as scalars or structures.** The salvo kernel needs per-munition-class mounts and ordered defensive layers (salvo §8.2–8.3, T-2). Scalars would force WS-07 to re-derive them. → The structured view is the contract; the scalars are labeled summaries, computed from it by fixed formulas (§4.3.3).

**D4 — Manpower granularity.**
(a) One scalar per polity. Can't express pilots versus riflemen, or conscript terms.
(b) **Three personnel classes (`general`, `technical`, `aircrew`) × a few pools, with content age/sex profiles.** Training lags differ by class (4, 9 and 24 months [D]), which is the lesson.
(c) Cohort-tracked personnel per unit. Heavy, and WS-03 already tracks cohorts.
→ (b). WS-03's interface needs draws and casualties by sex and 5-year band (WS-03 §2.3); content profiles map pool flows onto bands exactly.

**D5 — `military_power`.**
(a) Combat-model score (e.g., Lanchester or salvo forecast). Depends on the opponent; not one number.
(b) **Replacement value at constant reference prices × strength × readiness × supply × experience.** Domain-neutral, conserves cleanly across tiers, and newer designs count more because they cost more in real terms.
(c) Personnel count. Ignores equipment.
→ (b) [D] (OQ-F1). Per-domain values (land, sea, air, space, strategic) are published beside the total so WS-06 doesn't compare carriers with brigades.

**D6 — Equipment storage.** National depot (one stock per design per polity) versus per-base depots. Per-base storage adds transport orders with little play value. → **National equipment depot; munitions stored per base** (the salvo note needs base stock for aircraft sorties, §7.2) [D] (OQ-F2).

**D7 — Warheads.** (a) A WS-05 design and production line; (b) **a WS-11 production abstraction** at a `NuclearComplex` base, gated by WS-05 weapon techs and costed with WS-08b's fissile index (P-05b). Warheads are not slot designs, and the stockpile is a count. → (b) [D] (OQ-F4). Delivery systems (missiles, SSBNs, bombers) remain WS-05 designs.

## 3. Interfaces

### 3.1 Stage placement (WS-02 §3.3)

| Stage | WS-11 function | Writes | Reads (stage-start) |
|---|---|---|---|
| Scenario start | `force::init` | `force` (units, bases, pools, stockpiles from WS-08a bands and `forces_start.ron`, §6.3) | setting, content, `rnd` designs |
| R1 ApplyOrders | `force::apply_order` | `force` order fields (queued lifecycle actions, deployments, templates, priorities) | orders; WS-04 `gate_orders` |
| R5b Forces | `force::forces` | `force`: deliveries → units/depot/base stocks; personnel intake, training, releases; status transitions and mobilization; readiness, experience; replacements; refits; movement; supply; base construction; warhead production; reloads (incl. WS-08b `resupply_flows`); service and polity summaries; ISR `Observation`s; events | `rnd.outputs.deliveries` (R3, same tick); `econ` funded fractions and recruitment pool (R4, same tick); `gov` laws and `coercion_loyalty` (R5, same tick); `space` positions (stage start); WS-06 access matrix and clauses (previous tick) |
| R6 External | `force::combat_view`, `force::base_view`, `force::isr_coverage` (pure, called by WS-07) | nothing | `force` (R5b-committed) |
| R6b Losses | `force::losses` | `force`: Δs, destroyed and captured units, munitions and CM spent, base damage and capture, casualties; WS-08b `space.hazards` | `engagements`, `space.hazards` |
| R7 Tier | `Tiered for ForceBlock` | `agg.military_power`, `force.macro`, detail `Option` | R6b results |
| D1/D2 | `force::deterrence`, `force::believed_deterrence`, `force::rollup`, `force::validate_template` (pure) | nothing | `PolityView` for the believed forms |

Lags that follow (stated so nobody is surprised): manpower drawn in R5b reaches WS-03's labor force next tick (WS-02 §3.3). WS-03 sees this tick's upkeep demand in next tick's R4. Deliveries from R3 arrive in R5b the same tick (RND-093). WS-04 reads personnel and readiness one tick old (WS-04 §3.3). A unit created in R5b gets a WS-08b `positions` entry from the next tick (WS-08b §3.3). Combat losses reach `military_power` in the same tick (ARC-024).

### 3.2 Data model (sketch; Rust types are the schema, ADR-0001)

```rust
pub struct BaseId(pub u32);  pub struct GroupId(pub u32);  pub struct TemplateId(pub u16);  // dense, never reused

pub struct ForceBlock {
    pub units: Vec<Unit>,                 // dense by UnitId (global, owner in Unit); dead units keep their slot
    pub groups: Vec<Group>,
    pub bases: Vec<Base>,                 // owned by this polity (host may differ)
    pub templates: Vec<TemplateDef>,      // custom templates; content presets are referenced, not copied
    pub depot: BTreeMap<DesignId, f64>,   // national equipment stock (ground vehicles, aircraft, kit)
    pub manpower: Manpower,
    pub doctrine: Vec<(ServiceIdx, DoctrineIdx, Tick /* adopted */)>,
    pub priorities: ServicePriorities,    // rationing weights when funded fraction < 1
    pub strategic: Strategic,
    pub summary: ForceSummary,            // Tracked polity-level outputs (§3.3–3.8)
    pub macro_: Option<MacroForce>,       // macro tier only (§4.12)
    agg_cache: Vec<UnitAgg>,              // #[serde(skip)]; rebuilt on load and on any fill/template/doctrine change
}

pub struct Unit {
    pub id: UnitId, pub owner: PolityId, pub alive: bool, pub name: String,
    pub template: (TemplateRef, u8 /* version */), pub service: ServiceIdx, pub domain: ForceDomain,
    pub status: Status,                   // Active | Reserve | Cadre | Mothballed | Refit { until: Tick, to: u8 }
    pub fills: Vec<Fill>,                 // one per role, sorted by role ID
    pub personnel: [Fill; 3],             // general, technical, aircrew (auth, on_hand)
    pub conscript_share: f32,
    pub s: Lazy, pub readiness: Lazy, pub experience: Lazy, pub supply: Lazy,
    pub home: BaseRef,                    // Base(b) | Unit(carrier or tender) | None
    pub location: RegionId, pub path: Vec<RegionId>, pub posture: Posture,
    pub group: Option<GroupId>,
    pub magazines: Vec<MagStock>,         // ships, batteries, spacecraft: (class, ≤ 2 lots, cap)
    pub cm_stock: u32,
    pub commissioned: Tick,
}
pub struct Fill { pub role: RoleIdx, pub item: Item /* Design(DesignId) | Kit(KitIdx) | Personnel(class) */,
                  pub auth: f32, pub on_hand: f32 }   // on_hand ≤ auth; ships and spacecraft: condition in [0, 1]
pub enum Posture { Garrison, Defend, Attack, Patrol, Blockade, Strike, Deter, Recon, Transit }
pub enum ForceDomain { Land, Sea, Air, Space, Strategic }    // military_power split (§4.10)

pub struct Group { pub id: GroupId, pub owner: PolityId, pub kind: GroupKind /* Squadron | Fleet | Wing | AirForce | OrbitalGroup | JointTaskForce */,
                   pub name: String, pub members: Vec<UnitId>, pub subgroups: Vec<GroupId>,
                   pub area: Vec<RegionId> /* JTF: its theater */, pub posture: Option<Posture> }

pub struct Base { pub id: BaseId, pub owner: PolityId, pub host: PolityId, pub region: RegionId, pub kind: BaseKindIdx,
                  pub level: u8, pub damage: Lazy /* capacity lost, [0, 1] */, pub building: Option<(u8, Tick)>,
                  pub munitions: Vec<MagStock>, pub active: bool /* false when host access lapses */ }

pub struct Manpower {
    pub pipeline: [[f64; 24]; 3],         // recruits by months of training remaining, per class
    pub unassigned: [f64; 3],             // trained, not yet in a unit
    pub conscripts: [[f64; 36]; 3],       // in units, by months of service remaining
    pub reserve_pool: [f64; 3],           // trained ex-service personnel, not in a unit
    pub wounded: [[f64; 4]; 3],           // returning to unassigned, by months remaining
}

pub struct Strategic { pub warheads: Tracked, pub production: Option<WarheadLine>, pub tested: bool,
                       pub alert: AlertLevel /* from WS-04 nuclear-posture law, §4.11 */ }
```

`UnitAgg` (§4.3) and `UnitCombatView` (§3.5) are derived values, never serialized. `Lazy` per unit: `s`, `readiness`, `experience`, `supply`, plus each base's `damage` (ARC-044 pattern). Everything polity-level in `summary` is `Tracked` (WS-02 §3.5).

### 3.3 Orders (WS-11 variants of `OrderKind`)

| Order | Effect | Gate / rejection |
|---|---|---|
| `DefineTemplate { def }` / `RetireTemplate { id }` | Adds a custom template (validated, §4.2.3) or hides one from new raises | Rejected if `validate_template` fails (each error named) |
| `RaiseUnit { template, version, home, status, designs: [(role, design)] }` | Creates a unit (§4.4.1) | Template locked; designs not of the role's hull class; home base lacks capacity |
| `RefitUnit { unit, version, designs }` | Version bump and/or design change per role (§4.4.2) | Version locked; ship or spacecraft refit without a yard (ship) or depot facility (spacecraft) |
| `SetUnitStatus { unit, status }` | Active ↔ Reserve ↔ Cadre ↔ Mothballed (§4.5.4) | Illegal transition (table §4.5.4) |
| `Decommission { unit }` / `SellUnit { unit }` / `SellEquipment { design, count }` | §4.4.3 | Unit in combat this tick; SellEquipment above depot count |
| `Mobilize { scope: Unit(u) \| Service(s) \| General }` / `Demobilize { scope }` | §4.5.5 | `General` is gated by WS-04 (PC cost, P-04f) |
| `FormGroup { kind, members, area }` / `EditGroup` / `DisbandGroup` | §4.7 | Members of another polity; a member in two groups of the same kind |
| `Deploy { target: Unit(u) \| Group(g), region, posture }` | Sets destination and posture (§4.7) | No path through passable regions (§4.7.1); posture illegal for domain; air destination beyond ferry range of any usable base |
| `Rebase { unit, base }` | Changes home base | Capacity or access missing |
| `BuildBase { region, kind }` / `CloseBase { base }` | Adds one level (§4.6.3) or closes | Region attribute caps; host access; sea region |
| `TransferMunitions { from, to, class, count }` | Moves lots between bases over the supply graph (§4.8.3) | No path; count > stock |
| `SetDoctrine { service, doctrine }` | §4.2.4 | Doctrine locked |
| `SetServicePriority { weights }` | Rationing weights (§4.5.3) | Any weight < 0 |
| `SetWarheadProduction { rate }` | §4.11.1 | Fission-weapon tech not `Proven`; no `NuclearComplex` base |
| `NuclearTest` / `NuclearRelease { target, count, kind }` | §4.11.3 | Gated by WS-04 (posture law ≠ no program; P-04f); no warheads |

AI polities emit the same orders (WS-06); macro polities emit none (§4.12). Invalid orders become rejection events (ARC-022).

### 3.4 WS-03 contract

**From WS-03 (R4, same tick):** recruitment pool by sex and 5-year band, ages 15–49 (ECO-084); average wage (for pay); funded fraction φ per line: `defense_personnel`, `defense_os`, `defense_procurement` (WS-11 share, P-03l), `internal_security` (P-03m); unemployment rate.

**To WS-03 (R5b, read next tick):**

| Item | Definition | § |
|---|---|---|
| Upkeep demand per line | personnel pay (§4.5.6), O&S (§4.5.2), WS-11 procurement demand: base construction, kit, warhead production, refit labor (P-03l), all real | §4.5, §4.6.3, §4.11.1 |
| Military personnel withdrawn, by sex and band | active + mobilized personnel in units + pipeline + unassigned + 0.1 × reserve-status personnel [D], mapped to bands by the §6.2 profiles | §4.5.1 |
| War casualties by sex and band | deaths from R6b (§4.8.1) through the same profiles | §4.8 |
| Capital destruction events | base damage and capture (value = base build cost × damage); nuclear use (§4.11.3) | §4.6.4 |

### 3.5 WS-07 contract (agree at the WS-07 kickoff)

WS-07 reads nothing from `force` except through these pure functions (P-10, SYS-009, ARC-024):

```rust
pub fn combat_view(world: &World, u: UnitId) -> UnitCombatView;   // salvo §8.1–8.3 + GroundStats
pub fn base_view(world: &World, b: BaseId) -> BaseCombatView;     // target_class facility: staying power, layers (from co-located batteries), munitions
pub fn isr_coverage(world: &World, p: PolityId, r: RegionId) -> f64;  // §4.10.2, detection input

pub struct UnitCombatView {
    pub unit: UnitId, pub owner: PolityId, pub location: RegionId, pub posture: Posture, pub group: Option<GroupId>,
    pub target_class: TargetClass, pub orbit: Option<NodeId>,           // orbit from WS-08b positions
    pub s: f64, pub readiness: f64, pub supply: f64, pub experience: f64, // separate multipliers (salvo §8.1)
    pub staying_power: f64, pub signature: f64, pub signature_fired: f64, pub sensor: f64, pub ew: f64,
    pub cm_stock: u32, pub cm_cap: u32, pub cm_per_threat: f64,
    pub mounts: Vec<MountView>,     // munition_class, rate, cap, stock lots | aircraft: count, sortie_rate, loadouts (+ base stock ref)
    pub layers: Vec<LayerView>,     // ordered: kind, munition_class or consumes=false, channels, range_band, stock/cap
    pub reload: ReloadRule, pub doctrine: SalvoDoctrine,             // salvo §7.3 preset from the service doctrine
    pub ground: Option<GroundStats>,                                  // firepower, protection, mobility (Lanchester)
    pub speed: f64, pub range: f64,
}
```

**From WS-07 (`engagements`, R6):** per unit, Δs, `destroyed`, `captured_by`, munitions spent by (class, lot), CM spent, hits received; per base, Δdamage and `captured_by`; region control changes. These fields extend the WS-02 `EngagementRecord` (P-07b). WS-11 applies them in R6b (§4.8).

### 3.6 WS-05 contract (accepts WS-05 P-11a and P-11b)

- **Stats:** unit aggregates use `DesignStats.achieved` (true) for simulation (RND-054); other polities' reasoning uses `believed_stats` (P-11a).
- **Deliveries (R3 → R5b):** ship and submarine designs create one tracked unit per delivered hull at the line's site base (§4.4.1); spacecraft create units at the launch site named in `Procure`, or join a named constellation unit (`destination: Unit`); ground vehicles and aircraft go to `depot`; munition lots go to the destination base's stock (salvo option C, §4.8.3); refit lots (`destination: Unit`) complete a refit (§4.4.2).
- **O&S:** non-personnel O&S per unit-year from WS-05 §4.7.4 with its foreign share and `embargoed` flag (§4.5.2).
- **Sales:** `SellUnit` and `SellEquipment` create WS-05 spot listings (§4.8.4 there); on sale, the unit is removed and the buyer receives a delivery of that design (P-05c).
- **Refit:** a design change in a role is a WS-05 `ReviseDesign` result or another existing design; WS-11 only applies it (P-11a).
- **Captures:** a captured unit or base equipment adds 1 unit of that design to the captor's `depot`, which satisfies WS-05's `ReverseEngineer` precondition (§4.8.7 there).
- **Macro quality:** macro equipment efficiency uses `MacroRnd.level` (§4.12; P-11a).
- **Rotary (P-11b accepted):** rotary hulls fill roles in army aviation templates (tracked as squadrons, ConOps §6.1 Air row) and in ship flights (sub-slots of ship templates). Basing reads the design flags `no_runway` (any army base, any air base, any ship with `flight_deck`), `sea_only` (sea zones and coastal bases only) and `ship_capable`.
- **Requested additions (P-05c–e):** `DesignStats.air_capacity` (aircraft a ship or base unit can carry), `troop_capacity` (amphibious lift, personnel), and `flight_deck`/`yard` flags; `ProductionLine.site: Option<BaseId>`; WS-11 reports each shipyard base's capacity so a ship line's rate obeys it.

### 3.7 WS-04 contract

- **From WS-04 (R5, same tick):** conscription law (none / selective / universal), women's status (eligibility, §4.5.1), nuclear-posture law (§4.11.2), `coercion_loyalty[faction]` (read and reported; no unit effect in the MVP, OQ-F6), gate results for `Mobilize { General }`, `NuclearTest`, `NuclearRelease` (P-04f).
- **To WS-04 (previous tick):** per service (army, navy, air, space, internal security): personnel, personnel-weighted readiness, spend (§4.5.3), and the strategic share of defense spending (closes WS-04 OQ-G9, §4.11.4). Services are content (`services.ron`); every template names one.
- **Budget competition:** services compete through WS-04's service factions (WS-04 §4.2, line interest `lin_f`) over the per-service spend WS-11 publishes (P-04g), and inside WS-11 through `SetServicePriority` when a line is underfunded (§4.5.3).

### 3.8 WS-06 contract

- **From WS-06 (previous tick):** access matrix `a_ij`; treaty clauses `Basing { host, kinds, max_level }`, `Transit { host }` (land/air passage), `Access { host }` (allied supply and resupply, already used by WS-08b §4.9.6); hostility set (who is at war with whom); `controller(region)` if occupation exists (else WS-08a owner).
- **To WS-06:** `deterrence(world, i, j)` and `believed_deterrence(view_j, i)` (pure, with breakdowns, §4.11.4); `Aggregates.military_power` and the per-domain split; public events `MobilizationDeclared`, `NuclearTest`, `NuclearUse`, `UnitDeployed { region, domain }` (visible deployments), `BaseOpened`; ISR `Observation`s (§4.10). WS-06 owns espionage and the public-noise baseline for force subjects (P-06b).

### 3.9 WS-08 contract

- **WS-08a (accepted field ownership, WLD-022):** `forces.size`, `navy`, `air` → preset counts (§6.3); `forces.posture` → default postures; `tech.nuclear` → warheads and test flag (§6.3); region `terrain`, `port_class`, `airfield_class` and edge kinds feed base caps (§4.6.2) and supply (§4.9). WS-11's optional `settings/<s>/forces_start.ron` follows WLD-035's header rule.
- **WS-08b (accepts P-11a–d with these forms):** launch sites are `Base` kind `LaunchSite` with `pads = level`, `cadence` and `max_class` from content, latitude = region centroid (P-11a); `GroundStation` bases support spacecraft (§4.6.2) (P-11a); a spacecraft's position and Δv stock are WS-08b state, everything else is WS-11's, and a delivered spacecraft sits at its launch site (P-11b); spacecraft `reload: at_depot` refills through `resupply_flows`, applied by WS-11 in R5b (P-11c); macro promotion creates stub constellations conserving `MacroSpace.assets`, and R6b applies `space.hazards` like engagement Δs (P-11d).

## 4. Model

### 4.1 Echelons (ConOps §6.1)

| Domain | Design blocks (content) | Tracked unit | Groupings | Size bounds [R/D] |
|---|---|---|---|---|
| Army | element (company, battery, troop) → battalion → regiment (optional) → brigade/BCT | brigade | JTF | brigade 2,000–8,000 personnel, 2–8 maneuver battalions |
| Special operations | element | company | JTF | 60–250 |
| Navy | ship design (WS-05) + ship template (crew, embarked flight) | ship | squadron → fleet | 1 hull |
| Air (incl. army and naval aviation) | aircraft or rotary design → squadron template | squadron | wing → air force | 6–36 aircraft |
| Space | spacecraft design → constellation template | spacecraft or constellation | orbital group | 1–64 spacecraft |
| Internal security | element → battalion → brigade | brigade | JTF | as army |

Missile and air-defense units are brigade templates with `mounts`/`layers` (salvo T-4). Marines, airborne, carrier air wings and gendarmerie are templates (ConOps §6.1). Each template's `echelon` decides whether it can be a root (tracked) or only a child.

### 4.2 Templates

**4.2.1 Schema (content; Rust types are the schema).**

```ron
// content/core/force/elements.ron
Element(id: "elem.tank_company", service_tags: ["army", "security"],
  slots: [ Slot(role: "hull.ground.mbt", count: 14) ],
  personnel: (general: 10, technical: 56, aircrew: 0),
  kit: Some("kit.crew"), supply_need: 0.12, moves: true, requires: [])

// content/core/force/templates/army.ron
Template(id: "tmpl.army.armored_bct", service: "army", echelon: Brigade, root: true, domain: Land,
  versions: [
    Version(v: 1, from_year: 1955, requires: [],
      children: [ Child(t: "tmpl.army.tank_bn", n: 2), Child(t: "tmpl.army.mech_bn", n: 1),
                  Child(e: "elem.sp_artillery_battery", n: 3), Child(e: "elem.bde_hq", n: 1),
                  Child(e: "elem.support_company", n: 2) ],
      rules: []),
    Version(v: 2, from_year: 1965, requires: ["tech.rotorcraft.g2"],
      children: [ /* v1 + */ Child(t: "tmpl.army.aviation_flight", n: 1) ],
      rules: [ Rule(id: "rule.combined_arms", requires: ["doctrine.army.combined_arms"],
                    field: Firepower, op: Mul, value: 1.10) ]),
  ])
```

- A `Child` names an element (`e`) or a non-root template (`t`) with a count. Depth ≤ 4, acyclic (FRC-002).
- `requires` lists tech IDs (`Proven` for the polity), capability flags (WS-05 §3.7) or doctrine IDs. A version is unlocked when all of its requires hold and the year is ≥ `from_year` only for **AI default choice**; the year never locks anything for the player [D].
- `rules` are template-level multipliers on one aggregate field (`Mul` > 0) or additions (`Add`), each a breakdown term with its rule ID as label. At most 4 per version [D].
- Roles are WS-05 hull-class IDs or kit classes (`kit.infantry`, `kit.crew`, `kit.sof`; §4.3.4). Ship and spacecraft templates have one hull role whose `count` is 1 (ship) or `n` (constellation, chosen at raise within bounds).

**4.2.2 Flattening.** `flatten(template, version) -> Vec<(RoleIdx, auth)> + personnel[3] + supply_need + moves_roles` multiplies counts down the tree and sums by role. Ships take crew from `DesignStats.crew` (technical) plus an embarked flight's personnel; squadrons take aircrew `crew × aircraft` plus 12 technical per aircraft [R: maintainer ratios of roughly 10–20 per combat aircraft].

**4.2.3 Custom templates (player and AI).** A custom template is a `TemplateDef` with the same schema, built only from unlocked elements and unlocked sub-templates, within the §4.1 size bounds. `validate_template` returns every error (locked child, bound, cycle, depth, unknown role). Custom templates live in `ForceBlock.templates` (state), presets in content. Up to 32 custom templates per polity [D]. This is the "more customizable than HOI4 at the template level, thinner below" point: composition is free; elements are fixed.

**4.2.4 Doctrines (content options, not authored).** Each service has 2–4 doctrines (§6.4) with `requires`, a salvo doctrine preset (Saturation / Measured / Conserve, salvo §7.3), and ≤ 4 field multipliers (e.g., maneuver: mobility ×1.1, supply need ×1.1). `SetDoctrine` applies from the next tick; every unit of that service takes experience × 0.9 [D], which recovers through the normal experience rule (§4.5.7). One doctrine per service at a time.

### 4.3 Rollup (P-10, SYS-009)

**4.3.1 Inputs.** For unit u: its flattened template version, its `fills` (design per role), the polity's doctrine for its service, the template rules, `DesignStats.achieved` per design, and kit stats (§4.3.4). The rollup is evaluated at **authorized counts and full condition** (s = 1). Strength, readiness, supply and experience are separate multipliers that WS-07 applies (salvo §8.1); they never enter the aggregates.

**4.3.2 Operators.**

| Field (salvo §8.1 / GroundStats) | Operator | Per-role term |
|---|---|---|
| `staying_power`, `signature`, `cm_cap`, ground `firepower`, ground `protection`, `upkeep` | `sum` | auth × per-item value |
| `sensor`, `ew` | `max` over roles with auth > 0 | the max role's value (single term, labeled with that role) |
| `speed`, `range`, ground `mobility` | `min` over roles flagged `moves` | the min role's value |
| `cm_per_threat`, `signature_fired` (as multiplier), `reliability` | `mean` weighted by auth × unit_cost_ref | weight share × value |
| `mounts` (by munition class), `layers` (by kind, class, range band) | `merge`: sum `rate`, `cap`, `channels`; keep content order | one term per contributing role |

Then doctrine and template rule multipliers apply to listed fields, each as one term (Breakdown `product` form, WS-02 §3.5). `target_class` comes from the template's domain and root hull (Land brigade → `ground_mobile`; a brigade whose roles are all `TEL` or `air_defense` with `static` doctrine → `ground_fixed`) [D].

**Infantry and staying power of ground units.** Ground staying power = Σ vehicles × `staying_power` + personnel_general × w_inf, w_inf = 0.002 standard hits per soldier [D] (G-6: salvo hits on brigades convert to Lanchester strength through staying power). Infantry firepower = personnel_general × kit firepower (§4.3.4).

**Ship staying power.** Hughes's cube-root rule: about two thousand-pound-bomb equivalents to put an 8,000-ton ship out of action [V: [CIMSEC, citing Hughes's salvo equations](https://cimsec.org/distributed-maritime-operations-a-salvo-equation-analysis/)], i.e. w = 0.1 · Δ^(1/3) standard hits with Δ in tonnes (k = 0.100 [V: computed]; 1,500 t → 1.15, 9,000 t → 2.08, 100,000 t → 4.64). WS-05 §4.6.3 item 6 already uses hull base × (mass/mass_ref)^(1/3), so P-05e asks WS-05 to set the surface and subsurface hull bases to match k = 0.1 before armor and compartment add-ons. Caveat: fitting historical damage data, one analysis finds a single cube-root coefficient for all warship types implausible and expects closer to linear scaling for some mechanisms [V: [IMA, historical data in modelling warship battle damage](https://cdn.ima.org.uk/wp/wp-content/uploads/2011/10/Historical-Data-in-Modelling-Warships-Battle-Damage-Survival-Probability.pdf)]. The MVP keeps the cube root; M2(e) checks Falklands-scale outcomes (salvo G-11).

**4.3.3 Headline scalars (UI and AI summaries; P-10's six).** Fixed formulas over the structured view, at s = 1:

```
offense   = Σ_mounts rate × p_rel × lethality_ref(class) + firepower        (lethality_ref = lethality vs. the class's first legal target)
defense   = Σ_layers channels × pk_ref(kind) + protection
staying   = staying_power
detection = sensor
mobility  = speed (Sea, Air, Space) | mobility (Land)
upkeep    = personnel pay + O&S per year (real, §4.5)
```

Each is a `Breakdown` whose terms link to the structured fields. WS-07 never reads them.

**4.3.4 Kit.** Infantry and crew equipment isn't a WS-05 design. A kit class has a quality index = the polity's best `Proven` component of the classes it lists (WS-05 components in a `kit` slot kind: small arms, man-portable ATGM, MANPADS, night vision, radios) [D], firepower and cost per person from content × quality. Kit is bought automatically for new personnel and when quality rises (one refresh per person per 15 years [D]) and billed to WS-11 procurement demand (P-03l). This keeps WS-05 free of a per-soldier production line.

**4.3.5 Strength fraction s.** With v_j = auth_j × value(item_j) (unit_cost_ref at reference prices; personnel value = class training cost from content):

```
s = Σ_j v_j · min(1, on_hand_j / auth_j) / Σ_j v_j        (over equipment and personnel fills)
```

`s` is `Lazy` with one term per fill. A new unit, refit, loss or replacement changes `on_hand` and so s; nothing else does. WS-02 §3.8's continuous-strength requirement is met by construction.

### 4.4 Unit lifecycle

**4.4.1 Raise and commission.**
- *Army, SOF, air, internal security:* `RaiseUnit` creates the unit at its home base with on_hand = 0 everywhere, readiness 0.2, experience 0.7, status as ordered. Replacement (§4.5.3) then fills it from the depot and manpower pools. A unit is reported `Formed` when s ≥ 0.7 [D].
- *Ships:* each delivered hull creates a ship unit at the site base with the service's default template for that hull class (content `ship_templates.ron`), condition 1, personnel on_hand 0. The player may re-template it later with a refit.
- *Spacecraft:* delivered as a unit at the launch site (WS-08b P-11b); a delivery with `destination: Unit(c)` adds `count` spacecraft to constellation c (auth += count up to the template bound).

**4.4.2 Refit and modernization.** `RefitUnit` sets `status = Refit { until, to }`. Changed roles return their old equipment to the depot (ships: the old design is consumed) and draw the new designs from the depot. Duration [D]: ground and air 2 months; ships `0.25 × WS-05 lead time` at a base with `yard`; spacecraft only if co-located with a `Resupply` facility (WS-08b §4.9.6), 1 month. During refit, readiness cap 0.3. On completion, experience partially resets:

```
χ  = Σ_changed v_j / Σ_j v_j               (changed value share)
ε' = ε − 0.5 · χ · max(0, ε − ε_0),   ε_0 = 0.7
```

A version bump that adds roles raises `auth`; replacement fills it.

**4.4.3 Mothball, decommission, sell.** Mothballing is a status (§4.5.4). `Decommission` returns personnel to `reserve_pool` (volunteers) or releases them (conscripts), and equipment to the depot (ships and spacecraft are scrapped). `SellUnit` hands the unit to the WS-05 market at WS-05's price; while listed it is Mothballed; when sold it is removed (`UnitRemoved` event) and the buyer gets a delivery. `SellEquipment` lists depot stock the same way.

### 4.5 Manpower, upkeep and readiness

**4.5.1 Pool, eligibility and intake.**

```
pool_c   = Σ_bands(18–49) pop(sex, band) × elig(sex, law) × fit,    fit = 0.7 [R: roughly a third of young men fail entry standards]
elig(M)  = 1;  elig(F) = 0 / 0.05 / 0.15 for women's status restricted / partial / equal [D]  (× 1 more with universal conscription + equal, content flag)
```

- **Volunteers** (monthly, every class by content shares):
  `intake_vol = Σ_{18–24} pop × elig × fit × b_vol × (pay/wage)^η_p × (u/u_ref)^η_u × war × e^{z}`,
  b_vol = 0.0008 per month [R: an all-volunteer force of ≈ 180k recruits a year from ≈ 20M aged 18–24 is ≈ 0.9%/yr], η_p = 1.0, η_u = 0.3 [R: enlistment elasticities recalled as ≈ 0.5–1.0 for pay and ≈ 0.2–0.4 for unemployment], u_ref = 0.05, war = 1.5 in a defensive war and 0.8 in an expeditionary one [D], z ~ N(0, 0.1²) (stream `"frc"`, entity polity, purpose RECRUIT). Intake never exceeds the shortfall plus 5% [D]. Fixture: 10M eligible, pay ratio 1.2, u = 0.06, war = 1, z = 0 → 10,139.7 recruits/month [V: computed].
- **Conscripts:** under `selective`, intake = min(shortfall, 0.25 × men turning 18 × fit) per year, monthly 1/12; under `universal`, 0.9 × fit of the eligible turning-18 cohort per year [D]. Service term 24 months [R: terms of 12–36 months are typical], after which they leave units pro rata to units' conscript shares and join `reserve_pool`.
- **Training pipeline:** recruits enter `pipeline` for 4 / 9 / 24 months (general / technical / aircrew) [D], then `unassigned`. Personnel in pipeline count as withdrawn labor and are paid.
- **Attrition:** volunteers leave units at 1%/month [R: annual turnover of the order of 10–15%] into `reserve_pool`; `reserve_pool` decays 1/120 per month (ageing out) [D].
- **Ceiling:** total military personnel ≤ pool (FRC-030); intake stops at the ceiling.
- **Bands:** every flow maps to sex × band through the content profile for its source (volunteer, conscript, reservist; §6.2), so WS-03 receives exact counts (ECO-084).

**4.5.2 O&S demand and funded fraction.**

```
os_u     = Σ_fills on_hand_j × os_per_year(design_j, age_j) / 12 × k_status × tempo(posture)       (real)
k_status = 1.0 Active, 0.4 Reserve, 0.15 Cadre, 0.03 Mothballed, 1.2 Refit (yard work) [D]
tempo    = 1.0 Garrison/Defend/Deter; 1.3 Patrol/Recon; 1.6 Attack/Blockade/Strike; 1.1 Transit [D]
φ_os,u   = φ_os × (1 − foreign_share_u × embargoed_u)      (WS-05 §3.6)
```

Gendarmerie and border-guard units (service `internal_security`) bill the `internal_security` line instead of defense lines (P-03m).

**4.5.3 Rationing and service priority.** When φ < 1 on a line, WS-11 allocates the paid amount across services ∝ demand × weight (default weights 1) [D], then pro rata within a service. The effective φ per unit is what readiness uses. Per-service spend (personnel + O&S + procurement attributed by hull class → service, content map) is published to WS-04.

**4.5.4 Status transitions.**

| From → To | Lag [D] | Readiness cap | Personnel kept | Notes |
|---|---|---|---|---|
| Active → Reserve | immediate | 0.5 | 30% full-time; rest to `reserve_pool` with a unit tag | reservists stay civilian (withdrawn 0.1) |
| Active/Reserve → Cadre | immediate | 0.15 | 15% | equipment stays with the unit |
| any → Mothballed | 1 month | 0 | 0 | equipment preserved; O&S 0.03 |
| Reserve → Active (mobilize) | 1 month [R: reserve call-ups take days to weeks] | 1.0 | refill from tagged reservists, then `reserve_pool`, then `unassigned` | readiness starts at the reserve value |
| Cadre → Active | 3 months fill | 1.0 | as above | readiness rises from 0.15 at the §4.5.7 rate |
| Mothballed → Active | 3 months (ships 6) | 1.0 | as above | |

**4.5.5 Mobilization.** `Mobilize { General }` moves every Reserve and Cadre unit to Active, allows drafting the `reserve_pool` regardless of the conscription law, raises volunteer `war` to its war value, and emits `MobilizationDeclared` (public, WS-06). It is not industrial conversion (WS-03 ECO-016), which the player orders separately. `Demobilize` reverses status changes over 3 months.

**4.5.6 Pay.** Personnel demand = Σ personnel × wage × pay_ratio_class × k_pay, with pay_ratio from content (volunteer 1.0, conscript 0.25, reservist not mobilized 0.1, aircrew × 1.5) [D]. With φ_pers < 1, volunteer intake scales by φ_pers^η_p and attrition by 1/φ_pers (capped at 3×) [D].

**4.5.7 Readiness and experience.**

```
target_u = cap(status) × min(φ_os,u, φ_pers) × crowd_u,     crowd_u = min(1, capacity(home) / use(home))
r'       = r + (target − r)/τ,   τ = 6 months if target > r, 4 months if target < r     [D: readiness is slow to build, quick to lose]
ε'       = ε + (ε_train − ε)/24 + 0.05·1[engaged this tick],  ε ∈ [0.5, 1.5]
ε_train  = 0.7 + 0.3·r·(1 − 0.15·conscript_share)            [D]
```

New personnel joining a unit dilute experience by count-weighted average with ε = 0.7. Fixtures [V: computed]: r = 0.8, target 0.4 → 0.7 after one month; r = 0.2, target 1.0 → 0.3333 after one month. Each update is a `Lazy` set with terms (previous, funding, crowding, status cap, combat).

### 4.6 Basing

**4.6.1 Kinds (content `base_kinds.ron`).**

| Kind | Capacity measure per level 1–5 [D] | Used by |
|---|---|---|
| `Naval` | home-ported hull tonnes: 20k, 60k, 150k, 400k, 1M | ships; munitions reload; `yard` at level ≥ 3 |
| `Shipyard` | concurrent hull tonnes building: 10k, 40k, 120k, 300k, 600k | WS-05 ship line sites (P-05d) |
| `Air` | aircraft: 24, 72, 144, 288, 500 | squadrons (runway designs) |
| `Army` | personnel: 5k, 15k, 40k, 100k, 250k | brigades, SOF, internal security; `no_runway` aviation |
| `LaunchSite` | pads = level; cadence and max vehicle class from content | WS-08b launches |
| `GroundStation` | spacecraft supported: 4, 12, 30, 80, 200 | spacecraft readiness (crowding) and ISR downlink |
| `MissileField` | silos: 10, 50, 150, 400, 1,000 | silo ICBM brigades (σ_silo, §4.11.2) |
| `NuclearComplex` | warheads per month: 1, 5, 20, 60, 150 | warhead production |
| `Plant` | none (location and target only) | WS-05 non-ship line sites; WS-07 strikes |

Mobile bases: a ship with `air_capacity` > 0 is a base for its embarked squadrons; one with `troop_capacity` carries marine units; a replenishment ship carries munitions as cargo and reloads ships of its group at sea (salvo `unrep`).

**4.6.2 Caps from the map.** Naval and Shipyard need a land region with a `Coast` edge; level ≤ `port_class` when set, else ≤ 3 [D]. Air level ≤ `airfield_class` when set, else ≤ 4 (≤ 2 on Mountain, Ice or Jungle terrain) [D]. No base in sea zones (WS-08a §3.2).

**4.6.3 Construction and access.** `BuildBase` adds one level after content months (6–36 by kind and level) at a content cost billed to WS-11 procurement demand. Overseas bases need a WS-06 `Basing` clause with the host listing the kind and a max level; if the clause lapses, the base turns inactive, its units must `Rebase` within 3 months, and after that they are moved home automatically (event) [D].

**4.6.4 Capacity effects.** Use > capacity sets crowding (§4.5.7). Forward deployment: an air unit can operate in regions within `range/2` of its base (haversine, WS-08a §4 rule 5); a ship away from any friendly or access base for longer than `endurance_months` gets supply 0.5 [D]; an army unit's supply follows §4.9. Base damage (WS-07) lowers effective capacity by `damage`, which repairs at 0.05/month [D]. A captured base passes to the captor at level − 1 with its munitions; its build cost × damage is reported as a WS-03 capital-destruction event.

### 4.7 Groupings and deployment

**4.7.1 Groups.** A group has a kind (§3.2), members of one owner and, for a JTF, an area (its theater: a set of regions). Orders to a group apply to all members; members may still take unit orders. Groups have no stats of their own: a group's display aggregates are sums of member aggregates. Squadron/fleet, wing/air force and orbital groups organize a service; a JTF mixes services. Default groups at start come from `forces_start.ron` or the §6.3 generator.

**4.7.2 Movement (R5b).** Paths come from WS-02's graph search over passable regions: own, allied, neutral with a `Transit` clause, or hostile only as the final region with posture `Attack` (WS-07 resolves the fight). Monthly reach [D]:

| Domain | Reach per tick |
|---|---|
| Land | 600 km in friendly regions (rail and road); stops on entering a hostile region |
| Sea | speed_kn × 1.852 × 24 × 30 × 0.6 (15 kn ≈ 12,000 km [V: computed]) along `SeaLane`/`Coast` edges |
| Air | rebase to any usable base within ferry range in one tick |
| Space | none (WS-08b missions move spacecraft) |

**4.7.3 Postures.** WS-11 owns posture legality, tempo and basing; WS-07 owns what postures do in combat.

| Posture | Legal domains | Meaning for WS-07 (to agree) |
|---|---|---|
| Garrison | all | at base; internal security; default |
| Defend | Land, Sea, Air, Space | holds a region or area |
| Attack | Land, Sea, Air | seeks engagement in the target region |
| Patrol | Sea, Air, Space | sea/air control in the area; ASW search |
| Blockade | Sea, Air | interdicts `SeaLane` edges (§4.9) |
| Strike | Air, Sea (LACM), Land (missile brigades) | attacks bases, facilities and units in range |
| Deter | Strategic, any | counted for deterrence alert (§4.11) |
| Recon | SOF, Air, Space | generates ISR coverage (§4.10) |
| Transit | all | moving; no engagement initiative |

### 4.8 Losses, munitions and supply links (R6b)

**4.8.1 Applying engagements.** For each unit record: on_hand_j ← on_hand_j × (1 − Δs/s) for every fill (so the new s equals s − Δs exactly); `destroyed` → `alive = false`, `UnitRemoved { engagement }` (ARC-024); `captured_by` → the captor's depot gets 1 of each design in the unit with on_hand ≥ 1 [D]. Personnel lost = Σ class on_hand × Δs/s; of these 30% are deaths (to WS-03 by band) and 70% wounded, of whom half return through `wounded` after 4 months [R: killed-to-wounded ratios of roughly 1:3]. WS-08b debris hazards apply as Δs. Base records apply Δdamage and capture.

**4.8.2 Munitions.** Classes come from content (≈ 12, salvo F-5). Each holder (base, ship, battery, spacecraft) holds ≤ 2 lots per class (salvo §7.4, option C). Engagement `munitions spent by (class, lot)` decrements the holder's lot; aircraft rounds come from the home base's stock and unexpended rounds return there (salvo §7.2). Stock never goes below 0 (FRC-061).

**4.8.3 Refill (R5b).** WS-05 munition deliveries join the destination base's matching lot by count-weighted average or open the second lot; a third merges into its nearest match by categorical fields (salvo §7.4). Units reload min(`reload_per_tick`, base stock) when at a base of their home kind in the same region and in Garrison or Defend posture; ships with `unrep` reload from a replenishment ship in their group; spacecraft through `resupply_flows` (WS-08b). `TransferMunitions` moves lots along the supply graph, arriving after ceil(path km / 600) months and consuming supply capacity [D]. Lots never upgrade: a delivery of a new design never changes an existing lot's categorical fields (FRC-063, RND-058).

### 4.9 Supply (abstract capacity along region edges)

Supply points (SP): 1 SP = one month of a 5,000-person brigade at tempo 1 [D]; a unit's demand d_u = supply_need (template) × tempo.

```
cap_e  = c_kind × terrain_factor × (1 − interdiction_e)          c: Land 100, Strait 40, Coast 60 (+20 per naval-base level at either end), SeaLane 200 [D]
interdiction_e = min(0.9, Σ hostile blockade strength on e / 10)   (SeaLane only; strength = Σ s × readiness of Blockade units in an adjacent sea zone)
sources = friendly Army, Naval and Air bases of level ≥ 2, plus every owned region at 0.5 SP per region [D]
```

Each tick: one multi-source Dijkstra per polity over passable regions by km gives each unit's path to its nearest source. Load_e = Σ d_u over units routed through e; ratio_e = min(1, cap_e / load_e). Unit supply target = min over its path's edges ratio_e × exp(−km / 1,500) (1 if the unit sits on a source region), and `supply` moves to the target with τ = 2 months (`Lazy`). Fixture: one edge with cap 100, load 150, path 600 km → target 0.4469 [V: computed]. Low supply also caps readiness: target_u × min(1, supply/0.5) [D].

### 4.10 ISR assets (P-2)

**4.10.1 Coverage of a target polity j by observer i.**

```
c_ij = Σ_spacecraft(ISR, EW) sensor × s × readiness × gs_i     (global; gs_i = min(1, GroundStation capacity / ISR spacecraft))
     + Σ_squadrons(recon, AEW, MPA, UAV) sensor × s × readiness × 1[j has a region within range/2 of the squadron's base]
     + Σ_SOF(Recon posture) s × readiness × 1[unit located in a region j owns]
```

**4.10.2 Observations.** Each tick with c_ij > 0, WS-11 pushes one `Observation` per force subject of j (6 subjects: power land, sea, air, space; warheads; second strike) with value = (truth + 1) × exp(N(0, σ²)), σ = 1.0 / √(1 + c_ij) on the Log scale (stream `"frc"`, entity = i·65,536 + j, purpose ISR) [D]. The "+1" keeps zero stocks on a Log scale. These subjects are a new WS-02 range `force.intel` (P-02h). `isr_coverage(p, r)` for WS-07 detection is the same sum restricted to assets covering region r.

### 4.11 Strategic forces

**4.11.1 Warheads.** `SetWarheadProduction { rate }` at `NuclearComplex` bases (rate ≤ Σ capacity) needs the WS-05 fission-weapon generation-1 tech `Proven`; thermonuclear and MIRV techs raise yield and warheads per missile through content. Cost per warhead from content × (1 + k_fis·(1 − fissile_self_sufficiency)) (WS-08b P-05b, k_fis = 1.0) billed to WS-11 procurement demand. `warheads` is `Tracked`.

**4.11.2 Legs, survivability and posture.** Delivery units carry the `strategic` tag (SSBN hulls, ICBM/SLBM-armed brigades, bomber squadrons in Deter posture). Warheads are assigned to legs in proportion to launchers × warheads per launcher, up to stock.

| Leg | Survivability σ_leg [D] |
|---|---|
| Silo ICBM | 0.3; 0.6 with launch-on-warning (posture law "assured destruction" and early warning: ≥ 1 EW constellation with s·r ≥ 0.5) |
| Mobile ICBM (TEL) | 0.6 |
| SSBN | deployed fraction (units in Patrol/Deter away from base) × 0.95 |
| Bomber | alert fraction × 0.9; alert = 0.1 (minimal deterrence), 0.3 (assured destruction) |

```
S2_i   = Σ_legs W_leg × σ_leg × r_leg × p_rel_leg          (second-strike warheads; Tracked)
D_ij   = max(0, S2_i − I_j × pk_j)                         (I_j: interceptors held by j's units in strategic Defend posture)
C_ij   = (1 − exp(−D_ij / W_j)) × a_i,   W_j = 100 × (pop_j / 10⁸)^0.5,   a = 0, 0.8, 1.0 by posture law   [D]
```

W_j is the "unacceptable damage" scale; it stands in for McNamara-era assured-destruction criteria [R: roughly a quarter of population and half of industry]. Fixture [V: computed]: SSBN leg 32 warheads, deployed 0.5, r 0.9, p_rel 0.85 → 11.628; silo leg 40, σ 0.3, r 0.9, p_rel 0.8 → 8.640; bomber leg 48, alert 0.3, 0.9, r 0.9, p_rel 0.8 → 9.331; S2 = 29.599; with I_j = 10, pk 0.5, pop_j = 200M: D = 24.599, W_j = 141.42, C = 0.1597 at a = 1.

**4.11.3 Tests and use.** `NuclearTest` consumes one warhead and emits a public `NuclearTest` event (sets `tested`; WS-04 activates the Disarmament Movement). `NuclearRelease { target, count, kind: Demonstration | Counterforce | Countervalue }` removes `count` warheads and emits `NuclearUse { user, target, count, kind, yield }`. WS-11 models nothing else. WS-06, WS-04 and WS-03 own the consequences (P-06c, P-04f, P-03n).

**4.11.4 Outputs.** `deterrence(world, i, j)` returns C_ij with a breakdown by leg; `believed_deterrence(view_j, i)` uses j's belief mean of i's second-strike subject instead of truth (P-2), so ISR and secrecy shape bargaining. Strategic share of defense spending = spend of `strategic`-tagged units and warhead production ÷ total defense-line spend (WS-04 OQ-G9).

### 4.12 Macro tier (P-7, SYS-006)

```rust
pub struct MacroForce { pub power: [f64; 5] /* Land, Sea, Air, Space, Strategic */, pub readiness: [f64; 5],
                        pub personnel: f64, pub warheads: f64, pub second_strike: f64 }
```

```
spend      = def_share × GDP (WS-03 §3.13);  split by domain with archetype shares (content), proc_share = 0.3 [R: procurement ≈ a quarter to a third of defense budgets]
P_d'       = P_d·(1 − δ_d/12) + spend·share_d·proc_share/12 × e_d,     δ: Land 0.05, Sea 0.04, Air 0.07, Space 0.10, Strategic 0.04 /yr [D]
e_d        = φ_dom·step_d^(L_d − L_front,d) + (1 − φ_dom)·q_import,d     (WS-05 §4.8.8, §4.10)
r_d'       = r_d + (min(1, os_paid / (0.05·P_d)) − r_d)/6
military_power = Σ_d P_d × r_d
```

**Tier switch.** Demotion: P_d = Σ_{u in d} V_u·s_u, where V_u = Σ_j auth_j × value(item_j) at reference prices, and r_d = the V·s-weighted mean of r_u·supply_u·ε_u; so military_power is conserved exactly by construction. Promotion: build units from the era preset force mix for the polity's bands (§6.3) with r_u = r_d, supply = 1, ε = 1, then scale every new unit's on_hand by the factor that makes Σ V_u·s_u = P_d per domain (ARC-081 within 1e-6). Spacecraft follow WS-08b P-11d (stub constellations). Personnel and warheads are copied.

## 5. Requirements

| ID | Requirement | Acceptance criterion (testable) | Trace |
|---|---|---|---|
| FRC-001 | Templates, elements, doctrines, services and base kinds load from content with `deny_unknown_fields`; every reference resolves. | The core content loads; fixtures with an unknown role, an unknown child, an unknown service and an unknown tech in `requires` each report that error naming file and ID, all in one run (ARC-051). | SYS-007, P-6 |
| FRC-002 | Template trees are acyclic, ≤ 4 deep, and only `root: true` templates can be raised. | A fixture cycle (A → B → A), a depth-5 fixture and `RaiseUnit` on a battalion template are each rejected with a named error. | §4.2.1 |
| FRC-003 | Flattening multiplies counts down the tree and sums by role. | Fixture brigade: 2 battalions × 3 companies × 14 MBT + 1 HQ element with 4 MBT → auth(MBT) = 88; personnel sums equal the hand total exactly. | §4.2.2 |
| FRC-004 | A template version is usable only when its `requires` hold; the year is advisory. | With `tech.rotorcraft.g2` not `Proven`, `RaiseUnit` on v2 is rejected; after it is `Proven` the same order succeeds; a fixture at year < `from_year` with requires met succeeds for the player and is skipped by `ai_default_version`. | §4.2.1 |
| FRC-005 | Custom templates are validated before use and stored in state. | `DefineTemplate` with a locked element, a brigade of 9,000 personnel and a 33rd template reports exactly those 3 errors; a valid one is raisable and survives save/load (ARC-062 fixture). | §4.2.3 |
| FRC-006 | A new template defined only in content can be built and deployed (SYS-009). | Test scenario adds `tmpl.test.brigade` and its element in content files only; `RaiseUnit` + replacement fills it to s ≥ 0.7 within 6 ticks with stocked depot and pool; `Deploy` moves it to an adjacent owned region next tick. No code change. | SYS-009, SYS-007 |
| FRC-010 | The rollup applies the §4.3.2 operators. | Fixture unit with 3 roles: `staying_power` = Σ auth × value (1e-12 rel); `sensor` = max role value; `speed` = min over `moves` roles; `reliability` = cost-weighted mean; mounts of the same class merge (rates and caps summed); layer order follows content. | P-10, SYS-009 |
| FRC-011 | Every aggregate has a breakdown whose terms sum to the value, one term per contributing role plus doctrine and rule terms. | Property test over every unit in a 120-tick stub-economy run: for every `UnitAgg` field, `|Σ terms − total| ≤ 1e-9·max(1, |total|)` (ARC-041); term labels name roles, rules or doctrines only; ≤ 12 terms for a preset brigade. | SYS-004, SYS-009, P-3 |
| FRC-012 | Unit aggregates equal the rollup of sub-units and equipment. | For each preset template version (fixture designs per role): `rollup` of the unit equals the hand computation from the flattened table within 1e-12 rel; changing one design changes only terms of that role. | SYS-009 |
| FRC-013 | Aggregates exclude s, readiness, supply and experience; WS-07 receives them separately. | Halving a unit's readiness leaves every `UnitAgg` field bit-identical and halves `combat_view().readiness`. | salvo §8.1 |
| FRC-014 | Aggregates are a cache with no effect on state or hash. | Clear and rebuild `agg_cache` mid-run: state hash and all subsequent hashes equal a run without the rebuild (ARC-060). Save/load rebuilds it identically. | P-4, SYS-002 |
| FRC-015 | Ship staying power follows the cube-root anchor. | With WS-05 hull bases set per P-05e and no armor add-ons, a 9,000 t surface fixture has staying power 2.08 ± 0.01 and a 100,000 t one 4.64 ± 0.01. | §4.3.2, salvo §8.6 |
| FRC-016 | Headline scalars follow §4.3.3 and link to the structured fields. | Fixture: offense, defense, staying, detection, mobility and upkeep equal the formulas within 1e-12; every term carries a `link` to a `UnitAgg` field. | P-10 |
| FRC-017 | `UnitCombatView` fills every salvo §8.1–8.3 field (and `ground` for land units) with no NaN. | For one preset per domain, every field is present and finite; `combat_view` is the only `force` reader in WS-07 code (grader grep). | salvo §8, ARC-024 |
| FRC-018 | s is the value-weighted fill fraction. | Fixture: fills (auth 10, on_hand 5, value 2) and (auth 4, on_hand 4, value 5) → s = (10 + 20)/40 = 0.75 exactly; `explain` of s has one term per fill. | §4.3.5, WS-02 §3.8 |
| FRC-020 | `RaiseUnit` creates an empty unit that replacement fills. | Raise with 0 depot and 0 unassigned: s = 0; add depot and personnel: s rises by at most the absorption cap per tick (25% of auth) and reaches 1. | §4.4.1 |
| FRC-021 | Each delivered ship hull or spacecraft becomes one tracked unit the same tick; vehicles and aircraft go to the depot. | A stub R3 delivery of 2 frigates, 1 spacecraft and 30 MBT: after R5b there are 3 new units (2 ships at the site base, 1 spacecraft at the launch site) and depot(MBT) += 30. | RND-093, WS-08b P-11b |
| FRC-022 | Refit swaps designs, returns old equipment and partially resets experience. | Fixture with ε = 1.2, changed value share χ = 0.4: after refit, ε = 1.2 − 0.5·0.4·0.5 = 1.1 exactly; depot(old design) += old on_hand; readiness ≤ 0.3 during refit; a ship refit at a base without `yard` is rejected. | §4.4.2 |
| FRC-023 | Decommission and sale remove the unit and route personnel and equipment. | Decommission: volunteers to `reserve_pool`, conscripts released, ground equipment to depot, ship scrapped; `SellUnit` creates a WS-05 listing and, on sale, removes the unit with `UnitRemoved`; headcount conserved within 1e-9. | §4.4.3, WS-05 §4.8.4 |
| FRC-030 | The pool, eligibility and ceiling follow §4.5.1. | Fixture cohorts: pool equals the formula within 1e-12; with women's status `restricted`, female pool = 0; total personnel never exceeds pool across a 240-tick forced-intake run. | §4.5.1, ECO-084 |
| FRC-031 | Volunteer intake follows the elasticity formula. | Fixture (10M eligible, pay ratio 1.2, u = 0.06, war 1, z = 0) → 10,139.71 per month within 1e-6 rel; doubling pay ratio doubles intake (η_p = 1). | §4.5.1 |
| FRC-032 | Conscription follows the law and term. | Universal law, 1M fit eligible turning 18 per year: monthly intake = 0.9 × 1M/12 within 1e-9; conscripts leave after exactly 24 ticks into `reserve_pool`; with law `none`, intake = 0 unless `Mobilize { General }`. | §4.5.1, WS-04 §6.3 |
| FRC-033 | The training pipeline delays assignment by class. | Recruits entering at tick t appear in `unassigned` at t + 4, t + 9 and t + 24 for general, technical and aircrew; they are counted as withdrawn and paid while in the pipeline. | §4.5.1 |
| FRC-034 | Manpower draws and casualties reach WS-03 by sex and band, conserving counts. | Sum over bands of the exported withdrawn personnel equals total withdrawn personnel within 1e-9 rel; ECO-084's labor test passes against the WS-11 export; deaths exported equal 30% of personnel losses. | WS-03 §2.3, ECO-084 |
| FRC-035 | Status transitions follow §4.5.4. | Each row of the table on a fixture: lag, readiness cap, personnel kept and O&S factor match; an illegal transition (Mothballed → Reserve) is rejected. | §4.5.4 |
| FRC-036 | General mobilization activates reserves within the lag and is public. | 10 Reserve and 5 Cadre units with a full `reserve_pool`: after 1 tick all Reserve units are Active; Cadre units are Active after 3; one `MobilizationDeclared` event; the order is rejected if WS-04's gate refuses it. | §4.5.5 |
| FRC-040 | O&S and personnel demand follow §4.5.2 and §4.5.6, billed to the right lines. | Fixture: os_u equals the formula for each status and posture; internal-security units appear only in the `internal_security` demand; Σ unit demands = exported line demand within 1e-9. | §4.5, WS-03 §3.6 |
| FRC-041 | Readiness moves toward its funded target with asymmetric time constants. | r = 0.8, target 0.4 → 0.7 after 1 tick; r = 0.2, target 1.0 → 0.33333 after 1 tick; with φ_os = 0.7 (ECO-093 fixture), target = 0.7 × cap; breakdown terms sum to the change. | §4.5.7, ECO-093 |
| FRC-042 | An embargo unfunds the foreign O&S share. | Unit with foreign share 0.5, φ_os = 1: target falls to 0.5 × cap when `embargoed`, and is 1 × cap otherwise. | WS-05 §3.6 |
| FRC-043 | Rationing follows service priority weights. | Two services with equal demand, paid amount = 50% of demand, weights (3, 1): funded fractions 0.75 and 0.25 within 1e-12. | §4.5.3 |
| FRC-044 | Experience follows §4.5.7 and dilutes with new personnel. | Fixture: one tick of training at r = 1, conscript share 0 from ε = 0.7 → 0.7125; adding 50% new personnel at ε = 0.7 to ε = 1.1 gives 0.9667 within 1e-4; ε stays in [0.5, 1.5]. | §4.5.7 |
| FRC-045 | Doctrine change applies an experience penalty and changes doctrine terms. | `SetDoctrine` on a service: next tick every unit of that service has ε × 0.9 and its aggregates carry the new doctrine term; locked doctrines are rejected. | §4.2.4 |
| FRC-050 | Bases enforce kind capacity, map caps and access. | A Naval base in a region with no `Coast` edge is rejected; Air level 5 on Mountain terrain is rejected; an overseas base without a `Basing` clause is rejected; clause lapse → inactive base and automatic rehoming after 3 ticks. | §4.6 |
| FRC-051 | Over-capacity crowding lowers readiness targets. | Air base level 1 (24 aircraft) with 36 home aircraft: crowd = 0.6667 and each squadron's target scales by it. | §4.6.4 |
| FRC-052 | Launch sites and ground stations expose the WS-08b fields. | A LaunchSite of level 3 exposes pads = 3, content cadence and max class, and its region's latitude; ISR spacecraft beyond GroundStation capacity reduce `gs_i` proportionally. | WS-08b P-11a |
| FRC-053 | Base damage and capture apply in R6b. | Engagement Δdamage 0.4 → capacity × 0.6 and repair to 0.65 after one tick; capture → base owner = captor, level − 1, munitions transferred, one WS-03 capital-destruction event. | §4.6.4, salvo G-8 |
| FRC-060 | Combat losses apply proportionally and reach aggregates the same tick (ARC-024). | Engagement Δs = 0.2 on s = 0.8: every fill's on_hand × 0.75 and s = 0.6 exactly after R6b; `explain_change(military_power)` has that unit's term; a destroyed unit is gone after R7 with `UnitRemoved` referencing the record. | ARC-024, SYS-009 |
| FRC-061 | Munition stocks are conserved and never negative. | Over a 120-tick stub-combat run: per holder and class, Δstock = deliveries + transfers in − spent − transfers out exactly; no stock < 0; aircraft returned rounds = loadout − fired. | salvo §10 |
| FRC-062 | Lots follow option C. | Delivery of a lot with the same categorical fields averages `p_rel` by count; a different `threat_class` opens a second lot; a third different lot merges into the nearest categorical match. | salvo §7.4 |
| FRC-063 | New munition designs never change existing stock. | After a new AShM design is fielded, every pre-existing lot's stats are bit-identical until a delivery of the new design arrives at that holder. | RND-058, salvo G-5 |
| FRC-064 | Reload rules follow §4.8.3. | A ship at its Naval base in Garrison refills min(reload_per_tick, stock); the same ship in Patrol away from base does not, unless an `unrep` replenishment ship in its group has stock; a spacecraft's magazine delta equals WS-08b's depot delta (WLD-097). | salvo §7.2, WS-08b P-11c |
| FRC-070 | Movement follows passability and reach. | Land unit, 1,500 km path through own regions: arrives on tick 3; a path through a neutral region without `Transit` is rejected; Air rebase beyond ferry range is rejected; Sea unit at 15 kn covers ≤ 12,001 km per tick. | §4.7.2 |
| FRC-071 | Postures are checked for legality per domain. | `Blockade` for a land brigade and `Recon` for a frigate are rejected; every legal pair in §4.7.3 is accepted. | §4.7.3 |
| FRC-072 | Group orders reach every member. | `Deploy` on a fleet of 5 ships sets all 5 destinations and postures; a unit in two fleets is rejected. | §4.7.1 |
| FRC-080 | Supply follows §4.9. | Fixture edge cap 100, load 150, 600 km path → target 0.44688 within 1e-5; after one tick at τ = 2 from 1.0, supply = 0.72344; a unit on a source region has target 1. | §4.9 |
| FRC-081 | Blockades interdict sea lanes. | 5 Blockade units with s·r = 1 adjacent to a SeaLane → interdiction 0.5 and the edge cap halves; 20 such units cap interdiction at 0.9. | §4.9 |
| FRC-090 | ISR coverage generates observations whose noise falls with coverage. | Observer with c_ij = 3 on a target with true land power P: 10,000 observations' log-errors have SD 0.5 ± 0.01; with c_ij = 0, no observation is pushed; I1 is the only belief writer (ARC-014 pattern). | P-2, §4.10 |
| FRC-091 | ISR uses the WS-02 `force.intel` subject range, omitted by macro observers. | A high-fidelity observer holds 6 × n_polities force subjects; a macro observer's `BeliefBlock::get` returns `None` for them. | P-02h, ARC-016 |
| FRC-100 | Warhead production respects tech, capacity and fissile cost. | Rejected before fission gen-1 is `Proven`; rate capped at Σ NuclearComplex capacity; unit cost doubles at fissile self-sufficiency 0 (k_fis = 1). | §4.11.1, WS-08b P-05b |
| FRC-101 | Second strike and deterrence follow §4.11.2. | The §4.11.2 fixture gives S2 = 29.599 and C = 0.15965 within 1e-4; breakdown terms are the three legs and sum to S2; a = 0 under "no program". | §4.11, SYS-004 |
| FRC-102 | Believed deterrence uses the viewer's belief, not truth. | Fixture where j's belief mean of i's second strike is 2 × truth: `believed_deterrence(view_j, i)` equals `deterrence` computed with 2 × S2; it reads only `PolityView` (ARC-012). | P-2 |
| FRC-103 | Tests and use are events, not modeled exchanges. | `NuclearTest`: warheads − 1, `tested = true`, one public event; `NuclearRelease { count: 5 }`: warheads − 5, one `NuclearUse` event with the payload fields; no other WS-11 state changes; both rejected when WS-04's gate refuses. | ConOps §3, §4.11.3 |
| FRC-104 | The strategic share of defense spending is published. | Fixture with strategic units' spend 30 and warhead production 10 out of 200 total defense-line spend → 0.2 exactly. | WS-04 OQ-G9 |
| FRC-110 | Per-service outputs reach WS-04. | Per service: personnel (sum over units), personnel-weighted readiness, and spend; Σ services' spend = total WS-11 spend within 1e-9; GOV-012 and GOV-014 pass against them. | WS-04 §3.3 |
| FRC-120 | `military_power` is the rollup of unit power and is Tracked. | military_power = Σ_u V_u·s_u·r_u·supply_u·ε_u within 1e-9 rel, one term per unit; the per-domain split sums to the total. | WS-02 §3.8, SYS-009 |
| FRC-121 | Macro forces follow §4.12. | Fixture: one year of spend at e_d = 1 and δ = 0 adds spend·share·proc_share to P_d within 1e-9; readiness follows the O&S ratio. | §4.12, P-7 |
| FRC-122 | A tier switch conserves military power and its domain split (ARC-081). | Promote then demote a polity with forces in all five domains (incl. spacecraft in two regimes): military_power and each P_d within 1e-6 rel at both steps; personnel and warheads exact. | SYS-006, ARC-081 |
| FRC-130 | Start state follows the WS-08a bands (§6.3). | For each `forces.size`/`navy`/`air` band and each `tech.nuclear` status, the generated force matches the §6.3 counts and stockpile; two polities with identical bands and no overrides get identical forces (WLD-021). | WS-08a §3.5 |
| FRC-140 | Determinism and RNG streams. | ARC-060 double run with forces active is hash-identical; WS-11 uses only the `"frc"` tag with RECRUIT and ISR purposes, and no stream key repeats in a tick (WS-02 §3.4). | SYS-002, P-4 |
| FRC-141 | WS-11 meets its share of the tick and memory budgets. | ARC-090 bench with ~300 units per high-fidelity polity (preset mix, 20 bases each, supply and ISR on): R5b + R6b WS-11 functions ≤ 5 ms median on the reference machine; `force` blocks ≤ 2 MB of the postcard snapshot. | SYS-008, WS-02 §5 |
| FRC-142 | Unit-level variables are `Lazy` and explainable on query. | ARC-044 passes with WS-11's real unit variables (s, readiness, experience, supply, base damage) in place of the stub. | ARC-044, SYS-004 |

## 6. Content/data

### 6.1 Files

| File | Owner | Contents |
|---|---|---|
| `content/core/force/services.ron` | WS-11 | services (army, navy, air, space, internal_security, plus `sof` mapped to army for WS-04), hull class → service map, pay ratios |
| `content/core/force/elements.ron` | WS-11 | ≈ 40 element types (§6.4) |
| `content/core/force/templates/<service>.ron` | WS-11 | preset templates and versions |
| `content/core/force/ship_templates.ron` | WS-11 | default template per surface/subsurface hull class |
| `content/core/force/doctrines.ron` | WS-11 | per-service doctrines with salvo presets and multipliers |
| `content/core/force/base_kinds.ron` | WS-11 | §4.6.1 table, build months and costs |
| `content/core/force/kit.ron` | WS-11 | kit classes → WS-05 component classes, per-person firepower and cost |
| `content/core/force/profiles.ron` | WS-11 | age/sex profiles per source (§6.2) |
| `content/core/force/start_mix.ron` | WS-11 | band → preset force mix per era (§6.3) |
| `content/settings/<s>/forces_start.ron` | WS-11 | optional per-polity overrides: named units, bases, groups (WLD-035 header) |
| munition classes | WS-07 | salvo §8.4, §8.6 matrices (WS-11 reads class IDs) |

### 6.2 Profiles (shares by 5-year band) [D]

| Source | 15–19 | 20–24 | 25–29 | 30–34 | 35–39 | 40–44 | 45–49 |
|---|---|---|---|---|---|---|---|
| Volunteer | 0.10 | 0.40 | 0.25 | 0.13 | 0.07 | 0.03 | 0.02 |
| Conscript | 0.45 | 0.55 | 0 | 0 | 0 | 0 | 0 |
| Reservist | 0 | 0.20 | 0.30 | 0.25 | 0.15 | 0.07 | 0.03 |

Sex split per source = the eligible pool's sex shares (§4.5.1).

### 6.3 Start-state mapping (WS-11 is primary owner of `forces.*` and `tech.nuclear` weapon status)

| Band | `forces.size` → army brigades (+ SOF companies) | `navy` → major surface + subsurface hulls | `air` → squadrons |
|---|---|---|---|
| VeryHigh | 60 (+12) | 120 | 90 |
| High | 25 (+6) | 40 | 35 |
| Medium | 10 (+3) | 12 | 12 |
| Low | 4 (+1) | 3 | 4 |
| VeryLow | 1 (+0) | 0 | 1 |

Counts [D] are 1955-era preset units in the `start_mix.ron` proportions (e.g., infantry 50%, mechanized 25%, armored 15%, artillery 10% of brigades), created at 70% Active, 30% Reserve [D], with bases generated at the capital (Army, Air), the largest coastal region (Naval, Shipyard if `navy` ≥ Medium) and one `NuclearComplex` for `Tested`/`Arsenal`. `forces.posture` → default posture (Defensive → Defend; Deterrent → Deter for strategic units, Defend otherwise; Expeditionary → Garrison with one JTF; Internal → Garrison with internal-security share doubled; Minimal → Garrison, all Reserve). `tech.nuclear`: `NoProgram`/`Program` → 0 warheads; `Tested` → 5; `Arsenal` → 200 × (forces.size ≥ High) or 50 otherwise, and 2,000 for VeryHigh [D; 1955 stockpiles were of the order of a few thousand for the largest holder and a few hundred for the second [R]]. Macro polities get the same numbers as `MacroForce` power (V of the generated mix) without units. Per-polity overrides belong in `forces_start.ron`.

### 6.4 Preset template list per service and era (deliverable)

Eras: **E1** 1955–69 · **E2** 1970–89 · **E3** 1990–2014 · **E4** 2015–39 · **E5** 2040–60. A cell is a template version; "—" means not available; versions unlock by the listed techs or doctrines, not by date (§4.2.1). Real-world analogues are inspiration only [R].

| Service | Template line | E1 | E2 | E3 | E4 | E5 | Unlock drivers (examples) |
|---|---|---|---|---|---|---|---|
| Army | Infantry brigade (leg/motorized) | v1 | v2 (APCs, organic ATGM) | v3 (night vision kit) | v4 (organic UAV company) | v5 (loitering munitions) | kit lines, autonomy |
| Army | Mechanized brigade | v1 (APC) | v2 (IFV) | v3 | v4 (APS) | — | IFV hull, active protection |
| Army | Armored brigade/BCT | v1 | v2 (+aviation flight) | v3 (combined-arms battalions) | v4 | v5 (manned-unmanned teams) | rotorcraft g2, `doctrine.army.combined_arms`, autonomy |
| Army | Airborne / air-assault brigade | v1 (para) | v2 (air assault, helicopters) | v3 | v4 | — | rotorcraft g2 |
| Army | Medium (wheeled) brigade | — | — | v1 | v2 | — | wheeled vehicle line |
| Army | Mountain / light brigade | v1 | — | v2 | — | — | |
| Army | Artillery brigade | v1 (tube) | v2 (SP, MLRS) | v3 (guided rockets) | v4 | — | SPA hull, guidance |
| Army | Air-defense brigade (battery brigade, T-4) | v1 (guns, SAM) | v2 | v3 (ABM-capable) | v4 (DEW point layer) | — | SAM, phased array, DEW |
| Army | Missile brigade (SRBM/GLCM, TEL) | v1 | v2 | v3 (HGV) | v4 | — | rocketry, HGV |
| Army | Strategic missile brigade (silo / mobile) | v1 (silo) | v2 (mobile) | v3 (MIRV) | — | — | ICBM hull, MIRV |
| Army | Army aviation squadron (rotary: attack, utility, scout) | v1 (utility, scout) | v2 (+attack) | v3 | v4 (+UAV) | — | rotary hulls (WS-05 P-11b) |
| SOF | Recon company | v1 | v2 | v3 | v4 (UAV) | — | |
| SOF | Direct-action company | v1 | v2 | v3 | v4 | — | |
| Navy | Surface combatant (patrol … cruiser) | v1 | v2 (+helicopter flight) | v3 (VLS) | v4 (unmanned escorts) | — | rotary, VLS line |
| Navy | Carrier (with air capacity) | v1 | v2 | v3 | v4 | — | carrier hull |
| Navy | Amphibious ship (troop capacity) | v1 | v2 (+helicopters) | v3 | — | — | |
| Navy | Submarine (SSK/SSN/SSGN) | v1 | v2 | v3 | v4 (UUV bay) | — | naval reactor, quieting |
| Navy | SSBN (strategic) | v1 | v2 | v3 | — | — | SLBM, MIRV |
| Navy | Auxiliary (replenishment, mine warfare) | v1 | v2 | — | — | — | |
| Navy | Marine brigade | v1 | v2 | v3 | v4 | — | amphibious vehicles |
| Air | Fighter squadron | v1 | v2 | v3 | v4 (LO) | v5 | airframe gens, LO |
| Air | Strike squadron | v1 | v2 | v3 (PGM) | v4 | — | guidance |
| Air | Bomber squadron (strategic) | v1 | v2 | v3 | — | — | |
| Air | AEW&C / recon / MPA squadron | v1 (recon, MPA) | v2 (+AEW) | v3 | v4 | — | radar, sonar |
| Air | Transport / tanker squadron | v1 | v2 | v3 | — | — | |
| Air | UAV squadron | — | — | v1 | v2 | v3 (autonomous) | autonomy |
| Space | Constellation (comsat, ISR, nav, early warning) | v1 (ISR, comsat; late E1) | v2 (+early warning, nav) | v3 | v4 (proliferated LEO) | v5 | satellite bus, launch |
| Space | Space combat group (ASAT, co-orbital) | — | v1 | v2 | v3 | v4 | ASAT, docking |
| Internal security | Gendarmerie / border guard / militia brigade | v1 ×3 | v2 ×3 | — | — | — | |

Count: 30 template lines, 78 versions; plus ≈ 40 elements (tank, mech infantry, motorized infantry, light infantry, para, air-assault, mountain, recon, engineer, SP artillery, towed artillery, MLRS, gun AD, SAM, ABM, TEL, silo group, HQ, support, signals, EW, aviation flights ×3, UAV, SOF ×2, gendarmerie, border guard, militia, marine, naval-flight ×2, aircrew and maintenance blocks, constellation block, ASAT block) and 12 doctrines (army: attrition, combined arms, networked maneuver, distributed; navy: sea control, sea denial, distributed lethality; air: air superiority first, strike-centric, integrated; space: support, contested) [D]. Authoring ≈ 160 entries, sized for one content session.

## 7. Verification

- Unit and integration tests named `frc_NNN_*`, one or more per requirement; fixtures in `crates/sim-core/tests/content/force/`.
- **M2(e) forces prototype** (P-9, before M6; Python, `prototypes/forces/`, ≈ 16 h [D]): (1) rollup of every §6.4 preset against hand tables; (2) a 1955–2060 batch (N = 200) for one high-fidelity polity with WS-03/04 stubs: personnel within the pool, readiness bounded, no negative stocks, O&S share of defense spending in 30–50% [R band], and a volunteer force sustaining 1–2% of population under 3% GDP spend [R]; (3) supply and ISR fixtures; (4) the deterrence fixture and sensitivity of C to W_j and σ_leg; (5) Falklands-scale salvo check of k = 0.1 with WS-07's prototype (salvo G-11).
- Batch stability (WS-10): no unit count above 400 per high-fidelity polity (R-6) in 1,000 AI runs; zero non-finite values (WS-02 §3.7).

## 8. Edits to other documents (proposed, not made)

| ID | Document | Edit | Why |
|---|---|---|---|
| P-CON-F1 | ConOps §5 P-10, §6 Combat row | P-10: "Each tracked unit exposes a few aggregates (offense, defense, staying power, detection, mobility, upkeep) as summaries; combat consumes the structured view behind them: per-munition-class mounts and defensive layers" (salvo T-2) | §2 D3 |
| P-CON-F2 | ConOps §3 Munitions row | "Stockpiles in a few aggregate classes **held per unit or base, ≤ 2 lots per class**" (salvo T-1) | §4.8.2 |
| P-02h | WS-02 §3.2, §5 | New subject range `force.intel` after WS-08b deposits: 6 subjects per polity (power ×4, warheads, second strike), Log scale on value + 1; macro observers omit it. Memory ≈ 6 × 282 × ~80 B ≈ 0.14 MB | §4.10 |
| P-02i | WS-02 §3.2, §3.5 | Add `BaseId(u32)`, `GroupId(u32)` to `ids.rs`; `Owner::Base(BaseId)` for base-level `Lazy` vars. `EventKind` gains WS-11 events (`MobilizationDeclared`, `NuclearTest`, `NuclearUse`, `UnitDeployed`, `BaseOpened`, `UnitFormed`) by PR | §3.2 |
| P-03l | WS-03 §3.6 | The `defense_procurement` line prorates between WS-05 demand and WS-11 demand (base construction, kit, warhead production), like the space-program line between WS-05 and WS-08 (P-03b) | §3.4 |
| P-03m | WS-03 §3.6 | Internal-security units bill the `internal_security` line, which pays min(appropriation, WS-11 demand + non-unit spending) and exports a funded fraction | §4.5.2 |
| P-03n | WS-03 §3.3 | `NuclearUse { kind: Countervalue }` destroys capital and population per warhead by content (WS-03 owns the magnitudes) | §4.11.3 |
| P-04f | WS-04 §3.4 | `gate_orders` covers `Mobilize { General }` (PC cost, Military +), `NuclearTest` and `NuclearRelease` (posture law ≠ no program; ethics box; PC); WS-04 reacts to `NuclearTest` (Disarmament activation, already in §6.2) and `NuclearUse` (legitimacy) | §3.7, §4.11.3 |
| P-04g | WS-04 §4.4 | Service factions' line interest reads WS-11's per-service spend (`service_spend[s]`), not raw procurement lines | §3.7, §4.5.3 |
| P-05c | WS-05 §4.8.4 | On a `SellUnit` sale, the buyer receives a delivery of that design with the unit's age; WS-05 removes the listing and WS-11 removes the unit | §4.4.3 |
| P-05d | WS-05 §3.2, §4.7 | `ProductionLine.site: Option<BaseId>` (WS-11 bases, not WS-08b `FacilityId`); a ship line's rate is limited by its Shipyard's tonnage capacity (WS-11 pure fn `site_capacity`) | §4.6.1 |
| P-05e | WS-05 §3.6, §4.6.3 | `DesignStats` gains `air_capacity`, `troop_capacity`, flags `flight_deck` and `yard`-requiring refit; surface/subsurface hull staying-power bases match k = 0.1 TPBE per t^(1/3) before add-ons; a `kit` slot kind for personal-equipment components; refit lots delivered to `Unit(u)` priced at the changed-component share | §4.3, §4.4.2 |
| P-06b | WS-06 | Supply `Basing`, `Transit` and `Access` clauses, the hostility set and (if any) `controller(region)`; own espionage and the public baseline on `force.intel` subjects; use `believed_deterrence` in bargaining | §3.8 |
| P-06c | WS-06 | React to `NuclearUse` (war, relations, alliance cascades) and `MobilizationDeclared` (threat) | §4.11.3 |
| P-07b | WS-07, WS-02 §3.2 | `EngagementRecord` per unit: Δs, destroyed, captured_by, munitions spent by (class, lot), CM spent, hits; per base: Δdamage, captured_by; region control changes. WS-07 consumes only `combat_view`, `base_view`, `isr_coverage`; posture semantics per §4.7.3 | §3.5 |
| P-08d | WS-08b §8 | Mark P-11a–d accepted with the §3.9 forms | §3.9 |

Inbound proposals closed here: WS-05 P-11a, P-11b; WS-08b P-11a–d; WS-04 OQ-G9 (§4.11.4).

## 9. Open questions

| ID | Question | Recommendation (default if unanswered) |
|---|---|---|
| OQ-F1 | `military_power` as replacement value × multipliers (D5) | Accept; a combat-model score is opponent-dependent and can't be one aggregate |
| OQ-F2 | National equipment depot and per-base munitions (D6) | Accept; per-base equipment depots add transport orders without new decisions |
| OQ-F3 | Custom templates built from fixed content elements, but elements not player-editable | Accept: "customizable templates" without player-authored companies or doctrine |
| OQ-F4 | Warheads as a WS-11 production abstraction (D7) | Accept; revisit if WS-05 wants warhead designs with yields |
| OQ-F5 | ConOps edits P-CON-F1/F2 (salvo T-1/T-2) | Accept both |
| OQ-F6 | Units following a coup (`coercion_loyalty`) | Defer: no unit-level effect in the MVP; WS-04 resolves coups from service personnel × readiness |
| OQ-F7 | Region control and occupation (who owns `controller(region)`) | Ask WS-06/WS-07 at their kickoffs; WS-11 uses WS-08a ownership until then |

## 10. Sources

- War Footing: [store page](https://store.steampowered.com/app/4727860/); [Dev Blog #2](https://sdg-website.pages.dev/dev-blog/02-how-war-footing-plays.html) (battalions as the primary unit, brigades for larger commitments, companies recently added) [V]; [Dev Blog #4](https://sdg-website.pages.dev/dev-blog/04-the-economy.html) (via m1-findings §3.4).
- Hughes cube-root rule: [CIMSEC, Distributed Maritime Operations – A Salvo Equation Analysis](https://cimsec.org/distributed-maritime-operations-a-salvo-equation-analysis/); Hughes 1995, [Naval Research Logistics 42(2)](https://ideas.repec.org/a/wly/navres/v42y1995i2p267-289.html); critique: [IMA paper on historical warship damage data](https://cdn.ima.org.uk/wp/wp-content/uploads/2011/10/Historical-Data-in-Modelling-Warships-Battle-Damage-Survival-Probability.pdf).
- Salvo kernel and munition lots: `docs/research/salvo-model.md`.
