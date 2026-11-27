# WS-05 — R&D Uncertainty, Craft Design, Defense Industry & Market
Status: Draft v0.1 · Parent: ConOps §3 (R&D, craft design, defense market, corporations rows), §5 (P-1, P-2, P-3, P-4, P-6, P-7, P-8, P-10), §6 (R&D, craft design, defense industry rows), SYS-002, SYS-004, SYS-005, SYS-006, SYS-008, SYS-009 · Depends on: WS-02 v1.2, ADR-0006, ADR-0008, WS-03 Draft v0.4 (§2.2–2.3, §3.1, §3.6, §3.10), WS-04 Draft v0.4 (§3.3), WS-08a Draft v0.3 (§3.5), WS-08b Draft v0.4 (§8 P-05a/b), salvo note v0.3 (§7.4, §8) · Last updated: 2026-11-27

Tags: **[V]** verified (source linked, or computed and reproducible) · **[R]** recalled, not checked · **[D]** design choice (to be tuned in M2(b)).
Prototype plan: `prototypes/rnd/PLAN.md`. Calibration seed: `prototypes/rnd/calib_growth.py`.

---

## 0. Design intent: what the system should teach

Innovation should feel uncertain, and the uncertainty should have causes the player can learn to manage. Each lesson below comes out of a mechanism; none is scripted.

| Lesson (SE / OR) | Mechanism | § |
|---|---|---|
| Mature technology before you commit to a design (knowledge-based acquisition) | Cost and schedule growth scale with the immaturity of the least mature critical technology. Calibrated to GAO: 4.8% vs. 34.9% RDT&E growth for mature vs. immature starts [V] | 4.5 |
| Requirements churn is expensive | `ChangeSpec` destroys completed work in proportion to the changed share and costs more after design freeze; production changes reset part of the learning curve | 4.5.6, 4.7 |
| Point estimates are optimistic; budget at a confidence level | The engineering estimate is the belief mean. The risk-adjusted P50/P80 comes from the belief plus the public growth distribution | 4.5.7 |
| Value of information | Science and tests are paid for in money and time, and they buy σ reduction before commitment | 4.2, 4.3 |
| Bayesian updating under ambiguous evidence; when to kill a program (sunk cost) | Test failures happen to working technologies too. A dead end is never confirmed with certainty, only made improbable | 4.2.3, 4.5.5 |
| Earned value management | Spending, earned value and CPI are explicit; EAC = AC + (BAC − EV)/CPI with a cause breakdown | 4.5.4 |
| Brooks's law and stretch-outs | Funding above the efficient rate has log returns; funding below it pays standing-army overhead | 4.5.2 |
| Learning curves, rate effects and the unit-cost death spiral | Wright cumulative-average curve with flattening; a rate term; quantity cuts raise unit cost | 4.7 |
| Lifecycle cost: O&S dominates | O&S per unit-year grows with age; it is about two thirds of 30-year lifecycle cost at default values | 4.7.4 |
| Make, license or buy; supply-chain dependency | Four acquisition paths trade cost, lead time and dependency; component provenance carries embargo risk | 4.8 |
| Fast-follower advantage | Fielding a design is public; everyone else learns that its technologies work | 4.2.4 |
| Industrial base atrophy | Contractor competence decays without work | 4.8.5 |

## 1. Purpose & scope

WS-05 owns technology truth and every polity's knowledge of it; science programs; development and design projects; craft designs and their derived stats; production (learning, rate, capacity, lifecycle cost); and the defense market (spot buy, arms deal, license, in-house), with contractors as AI suppliers.

**In (MVP):** P-1 truth per tech sampled at worldgen (viability, performance, cost, hidden enabler edges); P-2 beliefs (estimate ± σ) narrowed by science, tests, espionage and observing others; the science/engineering split; lognormal cost and schedule growth scaled by immaturity; beta-distributed line viability and binary test evidence; mixture performance priors; funding-scaled project slots with diminishing returns per project; hull presets, component slots and derived stats for 7 craft domains; component provenance; Wright learning with flattening, rate effects, capacity, O&S; four acquisition paths; contractors; civil technology effects on WS-03; a macro-tier reduced form; the WS-08a `tech.level` mapping.

**Out:** movable external components having any stat effect (OQ-3: cosmetic only); 3D; supply chains beyond component provenance; corporate finance and private-venture designs (OQ-R4); per-munition-type stock (salvo F-5); doctrine (WS-11); combat resolution (WS-07); station and facility design (WS-08b templates); player-authored tech content at runtime.

## 2. Decisions

| # | Decision | Recommendation |
|---|---|---|
| 1 | Tech graph granularity | **≈ 300 tech nodes over 12 engineering domains**, authored as **≈ 75 tech lines of 2–6 generations plus ≈ 20 singletons**, with per-generation parameters generated procedurally from each line's template. **10 science fields** sit beside the graph. §4.1, §6 |
| 2 | Belief representation and update rule | **ADR-0008 mixtures, exact Bayesian observations only, no σ decay.** Three subjects per tech: viability (a two-point mixture on a ±1 latent), performance (log, up to 3 components) and cost (log). Effort accrues precision, which is emitted as at most one pooled observation per subject per quarter. Binary test outcomes are converted to Gaussian observations that reproduce the exact Bernoulli likelihood ratio. §4.2 |
| 3 | Dead ends | **A tech is a dead end when its sampled `works` is false.** `works` is drawn per generation from a line-level Beta (so lines are good or bad together), by viability class. Two other ways to disappoint: the low performance mode, and a hidden `Requires` enabler that blocks completion with a distinct event (never disguised as a dead end). The player learns from science (weak), tests (strong), others fielding it (decisive positive), espionage and adjacent generations. §4.1.2, §4.2.3 |
| 4 | Design performance from components | **Slot-and-budget sums plus four named laws.** Components fill typed slots and consume mass and power; stats combine by `sum`, `max`, `min` or `product`; four physics-lite laws (log mass ratio, Admiralty speed, power ratio, staged payload) give range, Δv, speed, mobility and payload. Every stat has an exact breakdown. §4.6 |
| 5 | Market pricing and AI suppliers | **Cost-plus quotes**: seller's current marginal cost × a path markup × a utilization surcharge, in seller currency. **Rule-based sellers**: accept when the seller's export policy tier for the buyer covers the design's sensitivity tier and capacity allows. Contractors are domestic AI firms with domain competence that bid for home projects and sell their home polity's exportable designs. WS-06 sets AI export policy; WS-05 only applies it. §4.8 |

### 2.1 Options and trade-offs (condensed)

**D1 — Granularity.**

| Option | For | Against |
|---|---|---|
| ~100 nodes, coarse (Civilization-like) | Cheap to author; fast | One node per decade per domain: too few decisions; dead ends become catastrophic, not interesting |
| **~300 nodes in lines and generations** | Matches the WS-02 ARC-090 bench (300 techs) and ADR-0008's ~1,000 subjects per polity; about 3 nodes per domain per decade, so roughly one decision per domain every few years; authoring is ~75 line templates, not 300 entries (R-3) | Generations inside a line look alike unless the procedural steps vary; mitigated by sampled truth |
| ~800+ nodes (one per component variant, HOI4/Stellaris-like) | Fine-grained | Authoring and balancing beyond a 10 h/week budget; belief memory ×3; the UI turns into a list |

**D2 — Beliefs.** A simple σ decay (σ ← σ(1 − k·effort), mean drifts to truth) is cheaper but deterministic: the mean can't be wrong, so there is nothing to learn and no surprise. It is also a second mechanic, against P-2. Exact Bayesian updates with ADR-0008 mixtures keep one mechanic, are calibrated by construction when truth is drawn from the prior, and support bimodal "works or dead end" beliefs. The cost is observation volume, which pooling solves: n Gaussian observations of the same subject combine exactly into one with summed precision [V: standard conjugate result], so emitting at most one observation per subject per quarter loses nothing.

**D3 — Dead ends.**

| Option | For | Against |
|---|---|---|
| Independent Bernoulli per tech | Simplest | Every generation is a fresh coin; no "this whole approach is a lemon" worlds |
| **Line-level Beta, Bernoulli per generation, by viability class** | Lines are correlated, so world variety is meaningful; a dead generation ends its line in that world; class priors are legible ("speculative") | Needs a cross-generation inference rule, because beliefs are per subject (§4.2.5) |
| Dead ends only through hidden enablers | No hard failure | Never "it just doesn't work"; muddles the lesson |

Requires-edges deliberately do **not** fail tests: a blocked project raises a distinct `Blocked` event naming the field of the missing enabler. If a blocked test counted as viability evidence, beliefs would become miscalibrated (RND-012) and the player would be taught a wrong inference.

**D4 — Derived stats.**

| Option | For | Against |
|---|---|---|
| Pure additive (Stellaris-like) | Trivially legible | No trade-offs: bigger is always better; no range/payload physics, which WS-08b needs (Δv, payload to LEO) |
| **Budgeted sums + 4 laws** | Real trade-offs (fuel vs. payload vs. armor); WS-08b's Tsiolkovsky needs come free; one code path for 7 domains; each law has a textbook form a reviewer can check | Laws are code, not content (acceptable: like WS-03's CES); constants per domain still need tuning |
| Per-domain engineering models | Most realistic | 7 bespoke models; illegible; out of scope |

**D5 — Pricing.** Auctions or supply-demand clearing are heavy and opaque with ~6 sellers. Fixed list prices ignore learning and capacity. Cost-plus at the seller's current marginal cost passes learning through to buyers (exports lower the seller's own costs, as in real arms markets [R]), and the utilization surcharge makes scarce capacity expensive. War Footing's ordering holds: an arms deal is cheaper per unit than a spot buy but slower [V: m1-findings §3.4].

## 3. Interfaces

### 3.1 Stage placement (WS-02 §3.3)

| Stage | WS-05 function | Writes | Reads (stage-start) |
|---|---|---|---|
| Worldgen | `rnd::worldgen` | `Truth.tech`, `Truth.edges` | content, seed |
| Scenario start | `rnd::init` | `rnd` (from WS-08a `tech` bands, §3.9), tech `Observation`s | `Truth`, setting |
| R1 ApplyOrders | `rnd::apply_order` | `rnd` order fields (allocations, new projects, procurement, requests) | orders, WS-04 `gate_orders` result |
| R3 Production | `rnd::progress` | `rnd` (all polities: science, discovery, projects, designs, lines, contracts, market, macro, outputs); `Observation`s; events | `rnd`, `econ` (funded lines, cost index, capacity and fill rate: last tick), `gov.rnd_morale`, `space` (fissile index), `force` (sell listings), WS-06 access matrix, `Truth` |
| R6 External | `rnd::espionage_yield` (pure, called by WS-06) | nothing itself; returns `Observation`s and a `TechTransfer` that WS-06 pushes as an event | `rnd`, `beliefs` |
| R7 Tier | `Tiered for RndBlock` | `rnd.macro`, detail `Option` | R6b results |
| D1/D2 | `rnd::estimate`, `rnd::market_quotes`, `rnd::believed_stats` (pure) | nothing | `PolityView` only |

Lags that follow from placement (stated so nobody is surprised): WS-03 production in R3 reads `rnd.outputs` at stage start, so tech multipliers act one month after WS-05 computes them. WS-05 spends the line amounts WS-03 paid in last tick's R4, and sees last tick's sector fill rate. Deliveries computed in R3 are read by WS-11 and WS-08 in R5b of the same tick (WS-08b §3.3 already expects this). `TechTransfer` events from R6 are applied in the next tick's R3.

### 3.2 Data model (sketch; Rust types are the schema, ADR-0001)

```rust
// World truth (WS-02 Truth, private, immutable after worldgen; ARC-011)
pub struct TechTruth { pub works: bool, pub perf: f64 /* multiplier on component stats */, pub cost: f64 /* κ, multiplier on dev and unit cost */ }
pub struct EdgeTruth { pub from: TechIdx, pub to: TechIdx, pub kind: EdgeKind }
pub enum EdgeKind { Requires, Boost(f64) /* effort × (1 − b) */ }

pub struct RndBlock {
    pub fields: Vec<FieldState>,     // per science field: effort EMA (Tracked), classified flag
    pub techs: Vec<TechState>,       // dense by TechIdx
    pub projects: Vec<Project>,      // development, design, license setup, reverse engineering
    pub designs: Vec<Design>,        // own and licensed designs
    pub lines: Vec<ProductionLine>,
    pub contracts: Vec<Contract>,    // arms deals and licenses, as buyer or seller
    pub contractors: Vec<Contractor>,
    pub export_policy: ExportPolicy, // tier per buyer, default tier
    pub known_edges: BTreeSet<EdgeIdx>, pub hints: BTreeSet<(TechIdx, FieldIdx)>,
    pub stolen: BTreeMap<(PolityId, SubjectIdx), f64>,  // precision already copied (§4.2.6)
    pub slots: Tracked,
    pub outputs: RndOutputs,         // §3.4 and §3.8 values for other WS
    pub macro_: Option<MacroRnd>,    // macro tier only (§4.10)
}
pub enum TechStatus { Unknown, Known, Developing(ProjectId), Proven { tick: Tick }, Fielded { tick: Tick } }
pub struct TechState { pub status: TechStatus, pub m_sci: f64 /* science maturity 0..1 */, pub priority: f64,
                       acc: [f64; 3] /* precision accrued since last emission, per subject */ }
// TRL (displayed, derived): Unknown —; Known 1 + 3·m_sci; Developing 4 + 2·progress; Proven 6; Fielded 8.

pub struct Project {
    pub id: ProjectId, pub kind: ProjectKind, pub line: BudgetLine, pub funding: f64 /* requested, real per month */,
    pub contractor: Option<ContractorId>,
    pub baseline: Estimate,          // BAC and months at start (point), plus P50/P80 shown at approval
    pub progress: f64, pub ev: f64, pub ac: Tracked, pub eac: Tracked, pub phase: Phase, pub frozen: bool,
    pub tests: Vec<TestRecord>, pub blocked: Option<FieldIdx>,
    draws: ProjectDraws,             // pub(crate): true phase efforts, schedule factor, test thresholds (§4.5)
}
pub enum ProjectKind { Develop(TechIdx), Design(DesignDraft), Revise { base: DesignId, draft: DesignDraft },
                       LicenseSetup(ContractId), Reverse(DesignId) }

pub struct DesignId { pub owner: PolityId, pub seq: u16 }   // no global counter
pub struct Design {
    pub id: DesignId, pub domain: Domain, pub hull: HullClassIdx, pub slots: Vec<SlotFill>,
    pub placement: Vec<(u8, f32, f32)>,   // cosmetic only (OQ-3); never read by stat code
    pub specified: DesignStats,           // believed at design time
    pub achieved: Option<DesignStats>,    // revealed to the owner at acceptance test
    pub origin: Provenance, pub sensitivity: Tier, pub status: DesignStatus,
}
pub struct SlotFill { pub slot: u8, pub component: ComponentIdx, pub provenance: Provenance }
pub struct Provenance { pub origin: PolityId, pub source: Source /* InHouse | Licensed{licensor} | Imported{supplier} */ }

pub struct ProductionLine { pub design: DesignId, pub rate: f64 /* units/month */, pub n_cum: f64, pub n_reset: f64,
                            pub c1: f64, pub start_tick: Tick, pub first_delivery: Tick, pub site: Option<FacilityId>,
                            pub unit_cost: Tracked, acc: f64 /* fractional units */, pub halted: Option<HaltReason> }
```

`Tracked` variables per polity: `slots`, per field `effort`, per project `ac` and `eac`, per line `unit_cost`, and every §3.4/§3.8 output. Design stats don't change after acceptance, so they are stored as values; their breakdowns are rebuilt on query by the pure function `design_stats` (RND-056). SubjectIdx layout: three per tech, `tech_base + 3·t + k`, k ∈ {viability, perf, cost} (edit P-02a asks WS-02 to fix the per-subsystem subject ranges).

### 3.3 Orders (WS-05 variants of `OrderKind`)

| Order | Effect | Gate |
|---|---|---|
| `SetRndAllocation { line, targets: [(Target, real per month)] }` | Splits an R&D line among science fields and projects (Target = `Field(f)` or `Project(id)`) | none (within an enacted line) |
| `SetResearchPriority { tech, weight }` | Weight within its field's science effort (default 1) | none |
| `SetClassification { field, classified }` | Classified science is not published but runs at 0.8× information rate [D] | none |
| `StartDevelopment { tech, line, funding, contractor }` | Development project, needs a free slot and `Known` with m_sci ≥ 1/3 | none |
| `StartDesign { draft, line, funding, contractor }` / `ReviseDesign { base, draft, ... }` | Design project; draft must pass feasibility (RND-051) | none |
| `ChangeSpec { project, slot, component }` | Rework (§4.5.6) | none |
| `SetProjectFunding { project, funding }` / `CancelProject { project }` | — | none |
| `Procure { design, quantity, rate, site, line }` / `SetRate` / `StopProduction` | Production line (§4.7) | none |
| `RequestAcquisition { seller, item, path, quantity, rate }` | Spot buy, arms deal or license (§4.8) | WS-04 (ethics only, PC 0; P-04a) |
| `SetExportPolicy { buyer or default, tier }` | Seller-side policy; AI polities get it from WS-06 | WS-04 (ethics only; P-04a) |
| `ReverseEngineer { design }` | Needs ≥ 1 held unit of a foreign design (§4.8.6) | none |

Invalid orders become rejection events (ARC-022): no slot, unknown tech, infeasible draft, unaffordable line, policy refusal.

### 3.4 WS-03 contract

**From WS-03 (one tick old):** paid amount and funded fraction per WS-05 line (`civil_rnd[field]`, `defense_rnd`, `space_program`, `defense_procurement`); the R&D cost index (deflates every R&D amount to real effort); defense and space sector capacity, utilization and fill rate; FX rates and reserves (WS-03 performs and rations foreign payments, ECO-071). The skilled-labor pool (WS-03 §2.2) is not read in the MVP: it reaches WS-05 through the wage-weighted cost index.

**To WS-03 (`rnd.outputs`, R3):**

| Item | Definition | §, ID |
|---|---|---|
| Demand per line | Σ allocations + project funding + line spend + license fees (real). WS-03 pays min(appropriation, demand) and reports the funded fraction, as for the WS-11 lines (P-03b) | §4.5, RND-090 |
| Sector production demand | Real value of this month's lots on defense and space lines (domestic orders plus export contracts) → ECO-015 demand | §4.7, RND-067 |
| `M_s` per sector | exp(Σ_t e_ts · (a_tp − ā_t)): adoption relative to the world mean (OQ-R1) | §4.9, RND-091 |
| Capital-quality and depreciation modifiers; health-tech, contraception-tech and skill-bias indices | Same adoption form, content effects | §4.9 |
| Foreign procurement by seller | FX outflows: spot, deal deliveries, license fees, royalties, foreign spares | §4.8 |
| Arms export receipts and license fees by buyer | FX inflows | §4.8 |

### 3.5 WS-06 contract (espionage, intel, AI)

- **Espionage.** WS-06 decides the operation and its success. On success it calls `rnd::espionage_yield(world, thief, target, scope) -> (Vec<Observation>, Option<TechTransfer>)`, pushes the observations (source `"espionage"`) and emits the transfer as an event. Scopes: `Field(f)` (every subject the target knows in f), `Tech(t)` (one tech, plus m_sci transfer), `Design(d)` (blueprints: a 50% effort discount on a later `ReverseEngineer`) [D]. Yield is the target's knowledge minus what the thief already copied (§4.2.6). `RndBlock.fields[f].classified` is the counter-intelligence hook WS-06 may use.
- **AI inputs (pure, `PolityView` only):** `estimate(view, draft)`, `market_quotes(view, item)`, `believed_stats(view, design)`.
- **AI outputs consumed:** export policy per buyer (`SetExportPolicy`), acquisition requests, the access matrix `a_ij` (embargo: `a_ij` < 0.2 [D]).
- **Events WS-06 reads:** `DesignFielded { polity, design, hull, techs }` (public), `ArmsDeal`, `LicenseGranted`, `DeliveryHalted { reason: Embargo }`.

### 3.6 WS-11 and WS-07 contract: design stats

WS-07 reads only WS-11 unit aggregates (salvo note §8, ARC-024). WS-11 builds them from `DesignStats`, which WS-05 publishes per design (true values for simulation; `believed_stats` for any other polity's reasoning):

```rust
pub struct DesignStats {
    pub domain: Domain, pub hull: HullClassIdx, pub target_class: TargetClass, pub orbit_capable: bool,
    pub staying_power: f64, pub signature: f64, pub signature_fired: f64, pub sensor: f64, pub ew: f64,
    pub cm_cap: u32, pub cm_per_threat: f64,
    pub mounts: Vec<MountStat>,    // munition_class, rate, cap | aircraft: hardpoints, sortie_rate (salvo §8.2)
    pub layers: Vec<LayerStat>,    // kind, munition_class or consumes=false, channels, range_band, cap (§8.3)
    pub reload: ReloadRule,        // at_base | unrep | at_depot | none
    pub speed: f64, pub range: f64, pub endurance_months: f64, pub dv: f64,  // dv: spacecraft only (WS-08b)
    pub mass: f64, pub crew: u32, pub reliability: f64,
    pub unit_cost_ref: f64, pub os_per_year: f64,   // non-personnel O&S at age 0 (§4.7.4)
    pub munition: Option<MunitionStats>,  // class, threat_class, p_rel, lethality[target_class], eccm, range_band (§8.4)
    pub ground: Option<GroundStats>,      // firepower, protection, mobility: Lanchester inputs (salvo G-6)
    pub space: Option<SpaceStats>,        // payload_leo, dry_mass, survey_sensor (WS-08b P-05a)
    pub flags: Vec<CapFlag>,              // e.g., carrier_capable, unrep, aerocapture
}
```

- Munition designs deliver **lots** whose stats are the design's `MunitionStats` (salvo §7.4, option C). A new munition design never changes stock already delivered (RND-058).
- Deliveries: `rnd.outputs.deliveries: Vec<Delivery { design, count, destination: Site | Base | Unit, lot: Option<MunitionStats> }>`, produced in R3 and consumed by WS-11 (and WS-08 for launch vehicles) in R5b.
- WS-11 → WS-05: unit sales (`Sell` → a market listing, §4.8.4); refits are `ReviseDesign` projects whose result WS-11 applies to units; captured units (WS-07/WS-11 event) enable `ReverseEngineer`.
- WS-05 → WS-11 per unit-year: non-personnel O&S cost, its foreign share and supplier, and an `embargoed` flag. Under embargo the foreign share is unfunded, and WS-11's readiness rule decays as for any funded-fraction shortfall.

### 3.7 WS-08b contract (P-05a/b, accepted here)

- Capability flags (`cap.offshore`, `cap.arctic`, `cap.observatory.{1,2,3}`, `cap.orbital_spectrometry`, `cap.lander`, `cap.aerocapture`, `cap.isru`) are content fields `grants: [CapFlag]` on techs, set when the tech is `Proven` by the polity.
- Best exhaust velocity `v_e` = max over the polity's proven rocket-propulsion techs of `base_ve × perf_true` (true value; WS-08 uses it for logistics).
- Launch-vehicle and spacecraft `DesignStats.space` (payload to LEO, dry mass, Δv, reliability, survey sensor class); deliveries to the site named in `Procure`.
- **P-05b:** dev and unit cost of techs and components tagged `fissile` are multiplied by `1 + k_fis · (1 − fissile_self_sufficiency)`, k_fis = 1.0 [D] (no domestic fissile production doubles nuclear program costs).

### 3.8 WS-04 contract

- Out: `arms_dependency[j]` ∈ [0, 1] = share of this polity's 12-month procurement + O&S value whose supply depends on polity j (imported components, deals, spot buys, foreign spares).
- In: `rnd_morale` = 1 + 0.1·A_Technocrats (WS-04 §3.3), multiplying science information and maturity rates and dividing project execution growth mean excess: m_G − 1 → (m_G − 1)/rnd_morale.
- Gate: P-04a (RequestAcquisition, SetExportPolicy).

### 3.9 WS-08a start-state mapping (WS-05 is primary owner of `tech.level`)

| `tech.level` | Frontier year Y_b [D] | Public start observations per known tech [D] |
|---|---|---|
| VeryHigh | 1955 | 3 |
| High | 1952 | 2 |
| Medium | 1948 | 1 |
| Low | 1943 | 0 |
| VeryLow | 1935 | 0 |

- Techs with nominal date D_t ≤ Y_b are `Proven` (they are historical, so they work, §4.1.2), with one observation σ = 0.05 per subject.
- Techs with Y_b < D_t ≤ Y_b + 6 are `Known` with m_sci = 1 − (D_t − Y_b)/6 and the band's public observations (σ 0.3 perf, 0.5 cost, LR 2 viability) [D]. Observations, not a wider prior, carry the band difference, so beliefs stay calibrated.
- `tech.nuclear` (WS-11 owns weapon status): `Program` = a running development project on the fission-weapon generation-1 tech at progress 0.3 [D]; `Tested` and `Arsenal` = that tech `Proven`.
- Two polities in the same band start identical (WLD-021); per-polity overrides, if any, live in `content/settings/<s>/rnd_start.ron` (optional, owned by WS-05).

### 3.10 Pure query functions (UI, AI)

`estimate(view, draft) -> Estimate { point, p50, p80, months_p50, months_p80 }` (§4.5.7) · `market_quotes(view, item) -> Vec<Quote { seller, path, unit_price, lead_months, dependency }>` · `believed_stats(view, design) -> DesignStats` · `design_stats(content, truth, design) -> (DesignStats, Vec<Breakdown>)` (sim-only, for WS-11) · `p_works(belief) -> f64` = weight of the +1 component.

## 4. Model

Notation: monthly ticks; real amounts are deflated by WS-03's R&D cost index for R&D and by the price level for procurement; `u` = immaturity (§4.5.3).

### 4.1 Tech graph and world truth (P-1)

**4.1.1 Structure.** Content defines science fields, engineering domains, tech lines and singletons, components, hull classes and enabler candidates (§6). A line expands into generation techs `tech.<line>.g<k>`. Generation k+1 lists generation k as a prerequisite. Lines may list prerequisites in other lines. Each tech carries: field weights (1–2 fields), a domain, a nominal date D_t = `first + (k − 1)·step_years`, base development cost `base_dev × step_dev^(k−1)`, viability class, a performance prior, a cost prior, optional `grants`, `econ_effects` and component templates.

**4.1.2 Sampled truth** (`worldgen_stream(seed, id, purpose)`, ARC-034):
- **Viability.** Per line: p_line ~ Beta(α_c, β_c) by class (key: line ID). Per generation: works ~ Bernoulli(p_line) (key: tech ID). Techs with D_t ≤ scenario start are `historical` and always work, with performance drawn from the nominal component.

| Class | Beta(α, β) [D] | Mean |
|---|---|---|
| Established | (39, 1) | 0.975 |
| Promising | (8, 2) | 0.80 |
| Speculative | (2, 2) | 0.50 |
| Fringe | (1, 4) | 0.20 |

- **Performance** (Log): a mixture (disappointing, nominal, breakthrough) with weights (0.15, 0.75, 0.10), component medians (0.75, 1.0, 1.3) and log-σ 0.10 each [D]. A line may override it. Truth is drawn from the same mixture as the prior, so beliefs are calibrated.
- **Cost** κ (Log): Normal(0, σ_κ) in logs, σ_κ = 0.35 [D]. κ multiplies the tech's development effort and the unit cost of components that use it.
- **Enabler edges.** (a) Authored candidates (`enablers.ron`: from, to, p, kind), one draw each (key `edge:<from>><to>`). (b) Procedural: for every ordered pair of lines in different fields, an edge exists with p_rand = 0.002 [D] (≈ 11 per world with 75 lines), keyed by the pair, so adding a line only adds pairs. The target generation is drawn uniformly; the source is the source line's generation with the latest D ≤ target D. Kind: Requires with 0.2, else Boost(b ~ U[0.15, 0.40]) [D]. Every edge points from an earlier D to a later one, so the Requires graph plus prerequisites is acyclic by construction.

**Expected dead ends** with the §6 class mix (≈ 45 historical, 128 Established, 76 Promising, 36 Speculative, 15 Fringe): 128·0.025 + 76·0.2 + 36·0.5 + 15·0.8 ≈ 48 per world [D: arithmetic]. Not every dead end is encountered, because a dead generation truncates its line. M2(b) B1 reports both counts and their spread.

### 4.2 Beliefs (P-2, ADR-0008)

**4.2.1 Subjects.** Per known tech, three subjects:

| k | Subject | Scale | Prior (= sampling distribution) |
|---|---|---|---|
| 0 | viability v | Linear | 2 components at +1 (works) and −1 (dead), σ_c = 1e-3, weights (p̄_class, 1 − p̄_class) |
| 1 | performance | Log | the §4.1.2 mixture |
| 2 | cost κ | Log | Normal(0, σ_κ) |

Unknown techs are `None`. A tech becomes known by discovery (§4.4), by a prerequisite generation becoming `Proven`, by public fielding (§4.2.4), by espionage, or at start (§3.9).

**4.2.2 Effort → precision → observations.** Science, tests and other sources add precision Δτ_k (on the subject's scale) to `acc[k]`. At most once per quarter per subject (in months where (month + field index) mod 3 = 0, which spreads the load), and at every test, an accumulated `acc[k]` is emitted as one `Observation` with noise σ = 1/√acc[k] and value = truth (perf, cost: truth × exp(N(0, σ)) on the Log scale; viability: v_true + N(0, σ)), then `acc[k]` is reset. Pooling is exact for Gaussian observations (D2).

**4.2.3 Binary evidence.** A test outcome with likelihood ratio LR = P(outcome | works)/P(outcome | dead) is pushed on the viability subject as value +1 (if LR > 1) or −1 with noise σ_o² = 2/|ln LR| − σ_c². The mixture update then reproduces the exact Bernoulli posterior [V: computed, `calib_growth.py`: prior 0.40, LR 9 → 0.857143 both ways]. UI bands on `p_works` [D]: "likely dead end" < 0.10, "consensus dead end" < 0.02. Nothing ever sets it to 0.

**4.2.4 Observing others.** When a polity's design receives its first production delivery, its techs become `Fielded` for that polity and a public `DesignFielded` event fires. Every other high-fidelity polity, for each tech in the design: becomes `Known` if it was `Unknown`; receives viability evidence LR = 50 [D]; and receives a perf observation with σ = 0.25 / (1 + intel_q), where `intel_q` ≥ 0 comes from WS-06 (default 0) [D]. Fast followers therefore skip most of the dead-end risk, not the cost.

**4.2.5 Cross-generation inference.** Because viability is correlated within a line, when generation k becomes `Proven`, generation k+1 (if known) receives evidence LR = 3; when p_works of k drops below 0.02, k+1 receives LR = 1/3 [D]. This is the one heuristic in the belief system; RND-012 checks that calibration survives it.

**4.2.6 Espionage yield.** For each subject in scope that the target knows: Log/Linear subjects yield an observation with precision max(0, τ_target − τ_stolen)/k_spy², k_spy = 1.2 [D], where τ_target = 1/σ² of the target's belief on the subject's scale; viability yields evidence with ln LR = max(0, logit p_T − logit p̄_class − L_stolen). The copied amount is recorded in `stolen`, so repeated thefts never make the thief more certain than the target. `TechTransfer`: thief m_sci ← max(own, 0.7·m_sci,target) [D].

**4.2.7 Published science.** Science funded from `civil_rnd` in an unclassified field is published: each emitted observation is also pushed to every other high-fidelity polity that knows the tech, with σ ×√2 and an independent noise draw [D]. Defense-line science and classified fields are private. This is Terra Invicta's global/private split (m1-findings §3.1) as a policy choice.

### 4.3 Science

Per field f, real effort per month e_f = (paid civil_rnd[f] + defense allocations to f) × rnd_morale. The `effort` Tracked variable is a 60-month EMA of e_f (used for discovery and display). Within the field, e_f is split across `Known` techs that are not `Proven`, by `priority`:

```
e_t       = e_f · priority_t · w_tf / Σ_j priority_j · w_jf
Δm_sci,t  = e_t / E_sci,t,          E_sci,t = φ_sci · base_dev_t · premature(t, year)    φ_sci = 0.15 [D]
Δτ_k,t    = info_k · e_t / E_sci,t  (× 0.8 if classified)    info = (viability 1.2, perf 30, cost 15) per full science phase [D]
premature = exp(max(0, D_t − year) / τ_p),   τ_p = 8 years [D]
```

m_sci is capped at 1 (TRL 4). One full science phase cuts perf σ from ≈ 0.17 (the mixture's moment σ) to ≈ 0.12 and cost σ from 0.35 to ≈ 0.21, and gives a working tech an expected viability ln LR of 2·1.2 = 2.4 [D: arithmetic]. Premature work is possible but costly, which paces the 1955–2060 timeline without hard date gates.

### 4.4 Discovery and hidden enablers

- **Hidden techs.** A line's first generation (and singletons) start `Unknown` unless §3.9 makes them known. Monthly discovery hazard h_t = 1 − exp(−κ_d · (effort_f/e_ref,f) · r(year)), r = clamp((year − (D_t − 10))/10, 0, 1)², κ_d = 0.05 [D], `e_ref,f` from content (≈ a 1955 major power's effort in the field). Requires every prerequisite to be at least `Known`. One stream per (polity, field, tick) draws all of the field's discovery checks.
- **Requires edges.** A project on B reaches its first test milestone (progress 0.7) and stops there with a `Blocked { field: field(A) }` event unless A is `Proven` by the polity. The hint (B, field(A)) is recorded. While blocked, spending buys no progress (§4.5.2); the player can pause. The edge itself is revealed when A becomes `Proven` and the hint exists, or by espionage on a polity that knows it.
- **Boost edges.** If A is `Proven` when B's project starts, B's true phase efforts are multiplied by (1 − b), so EAC shows an unexplained underrun. The edge is revealed (a) at that project's first test, or (b) monthly with probability 1 − exp(−κ_e · effort_field(B)/e_ref) while A is `Proven` and B is `Known`, κ_e = 0.02 [D] (Terra Invicta's random unlock, m1-findings §3.1).

### 4.5 Projects (engineering)

**4.5.1 Slots.** N_slots = clamp(round(√(B/B_slot)), 1, 20), where B is the 12-month paid total of `defense_rnd` + `space_program` + the project share of `civil_rnd`, and `B_slot` (content, real RC) is the cost of a typical project-year in the era [D]. B = 25·B_slot gives 5 slots. Science programs use no slots. Starting a project beyond the slot count is rejected; a drop in slots never cancels running projects, it only blocks new ones.

**4.5.2 Funding response (diminishing returns per project).** Each project has a true efficient rate R* = E_phase / T_phase (§4.5.3). With requested funding f and x = f/R*:

```
productive work  w = f − o·R*·(1 − x)     for x ≤ 1      (standing-army overhead o = 0.25 [D])
                 w = R*·(1 + β·ln x)       for x > 1      (crash returns β = 0.5 [D]; x capped at 3)
progress         Δp_phase = max(0, w) / E_phase
```

Fixture: x = 0.5 → w = 0.375 R*, so cost per unit progress is ×1.33; x = 2 → w = 1.347 R*, cost ×1.49 for a ×0.74 duration. Progress stops at x ≤ o/(1 + o) = 0.2. Spending is always f: unproductive spending is booked as overhead or crash premium in the EAC breakdown. While `Blocked`, w = 0.

**4.5.3 Execution growth (calibrated, [V: computed] against [V] anchors).** At project start, immaturity u = clamp((6 − TRL_min)/3, 0, 5/3), with TRL_min the lowest TRL among the techs the project must mature (a development project's own tech; a design's non-`Proven` techs; 6 if none). From one stream per project:

```
G (execution growth)  ~ LogNormal(mean 1.03 + 0.54u, CV 0.20 + 0.80u)       contractor competence c: (mean − 1)/c, CV/c
S (schedule growth)   ~ LogNormal(mean 1.04 + 0.30u, CV 0.15 + 0.30u),  corr(ln G, ln S) = 0.6 [D]
BAC (baseline)     = base · E_belief[κ] · premature · fissile             (the engineering point estimate)
True effort E_true = base · κ_true · premature · fissile · (1 − boosts) · G
True months T_true = T̂ · S
```

The estimation error κ_true/E_belief[κ] is not a separate draw: it is whatever the polity's belief gets wrong, and its spread is the belief σ (calibrated, RND-012). The fit below assumed a typical belief log-σ of 0.10 + 0.25u at project start; B3 refits G with the belief σ the simulation actually produces.

Fitted by `prototypes/rnd/calib_growth.py` (400,000 draws) to a start mix of 16% fully mature (GAO-07-406SP [V]) and 84% with TRL_min ~ U[3, 5.5] [D]: mature mean growth 1.030 (GAO 1.026–1.048 [V]), immature 1.345 (GAO 1.323–1.349 [V]), overall mean 1.295 and CV 0.789 (ACAT I: 1.36 and 0.80 [V, m1-findings]); schedule mean 1.19 (RAND 1971: ≈ 15% slip [V]). The GAO split and the ACAT mean can't both be met exactly; the fit sits 0.065 below 1.36 (OQ-R3). By TRL_min: 3 → mean 1.57, 4 → 1.39, 5 → 1.21, 6 → 1.03.

**4.5.4 Phases and earned value.** Phases Design [0, 0.4), Build [0.4, 0.7), Test [0.7, 1.0] of planned effort ω = (0.4, 0.3, 0.3). ln G is split across phases with variance shares (0.6, 0.25, 0.15) [D], so most growth appears early (tuned in M2(b) against the CPI stability rule [R: Christensen]). Reported monthly:

```
BAC = baseline point cost,   EV = progress · BAC,   AC = Σ spending,   CPI = EV/AC
EAC = AC + (BAC − EV)/CPI                                            [R: standard EVM CPI method]
EAC breakdown: BAC + realized growth by phase + rework (tests, spec changes) + overhead + crash premium + projected growth (BAC − EV)(1/CPI − 1)
```

The true final cost is never shown. Terms sum exactly (ARC-041).

**4.5.5 Tests.** Two milestones: integration test at progress 0.7 and acceptance at 1.0. For each tech the project matures:
- If the tech works: the integration test fails iff the test-phase growth draw exceeds a threshold, so P(fail) = 0.15 + 0.25u [D]. Failure and overrun are the same event, not separate rolls. Acceptance passes with q = 0.9; each failure costs 10% of BAC and 2 months of rework [D].
- If the tech is a dead end: tests pass with p_fp = 0.05 [D], and acceptance always fails, so a dead-end project can never complete.
- Each outcome pushes binary evidence (LR from these probabilities; e.g., u = 0.67 gives LR 1/3.0 for a failure and 13.6 for a pass) plus perf precision 100 and cost precision 50 [D].
- Completion: the tech becomes `Proven`; its components unlock; `grants` apply; the next generation becomes `Known`; the cost belief gets an observation with σ 0.05.

**4.5.6 Spec changes.** `ChangeSpec` replaces one slot's component. Changed share χ = the changed component's share of the draft's unit cost. Lost progress Δp = p·χ·k_rw, k_rw = 0.5 before design freeze (progress 0.4) and 1.0 after [D], plus a 1-month review halt; the changed share draws a fresh phase growth factor. GAO found that 63% of programs changed requirements after development began [V: GAO-08-467SP].

**4.5.7 Estimates.** The baseline shown at approval is the **engineering point estimate** (belief mean of cost × base effort; T̂ from content). `estimate()` also gives P50/P80 for cost and months by Monte Carlo over the polity's cost belief and the public G and S distributions at its u (256 draws from a fixed stream, so the UI is reproducible) [D]. A polity that budgets at P80 overruns in about 20% of projects (RND-038). AI budgets at P50 by default; the player chooses.

### 4.6 Craft design

**4.6.1 Domains and hull classes.** 7 domains, 53 hull classes (§6.3). A hull class defines target class, slot list (kind, count, max size), mass capacity, base structure stats, the laws used and its sensitivity tier. Hull instances scale by the domain's structure tech generation (airframe, hull materials, armor), so there are 53 class templates, not one per generation.

**4.6.2 Components.** A component template belongs to a tech line; generation k's instance has stats `base × step^(k−1) × perf_true(tech)` for performance-scaled stats, and unit cost `base_cost × cost_step^(k−1) × κ_true`. Each has slot kind and size, mass, power, and contributions `(stat, op, value)`. Components with tech status below `Proven` can be placed if m_sci ≥ 1/3 (TRL ≥ 2); the design project then matures them concurrently at its u.

**4.6.3 Derived stats.** Pure, ordered; every stat returns a `Breakdown`:
1. **Feasibility:** each slot's kind matches and size ≤ max; Σ component mass + structure ≤ mass capacity × (1 − fuel_min); Σ power draw ≤ Σ power generation. Infeasible drafts are rejected with the binding constraint named.
2. **Combine:** `sum` (staying power add-ons, magazine capacity, channels, cm_cap), `max` (sensor, ew), `min` (reliability floors), `product` (signature multipliers).
3. **Fill:** leftover mass becomes fuel or propellant where the domain has a fill law.
4. **Laws** (code; constants in domain content):

| Law | Form | Used for |
|---|---|---|
| `LogMassRatio` | out = k · eff · ln(m₀/m₁) | Spacecraft Δv (Tsiolkovsky, eff = v_e); aircraft and missile range (Breguet, eff = (V/c)·(L/D)) [R: textbook] |
| `Admiralty` | V = c · (P / Δ^(2/3))^(1/3) | Ship and submarine speed [R: Admiralty coefficient] |
| `Ratio` | out = k · (a/b)^e | Ground mobility (power/mass), sortie rate (maintainability) |
| `StagedPayload` | payload = m₀ · ((e^(−Δv_req/(n·v_e)) − ε)/(1 − ε))^n | Launch-vehicle payload to LEO, Δv_req = 9.4 km/s [R] |

Fixtures [V: computed]: `StagedPayload` with n = 2, v_e = 3.0 km/s, ε = 0.1, m₀ = 100 t gives 1.460 t to LEO; with v_e = 4.4, 7.328 t. `LogMassRatio` with eff 3.1, m₀/m₁ = 10/4 gives 2.841 km/s.

5. **Cost:** first-unit cost c1 = hull cost(mass) + Σ component costs; design development effort = k_dev(domain) × c1 × (1 + 0.5 × number of new components) [D].
6. **Staying power:** hull base × (mass/mass_ref)^(1/3) + Σ armor and compartment contributions [D; the salvo note asks WS-11 to check Hughes's displacement relation].

**4.6.4 Specified vs. achieved.** `specified` = the stats computed from the owner's belief means at design time. `achieved` = the stats computed from truth, revealed to the owner at acceptance and used by WS-11 from then on. Any other polity's view is `believed_stats`: the same function over its own beliefs (prior means for techs it doesn't know). This needs no per-design belief subjects, and the Measured doctrine (salvo §7.3) reads it directly.

**4.6.5 Placement.** External placement is stored for the 2D silhouette and never read by stat, cost, hash-relevant derived or combat code (OQ-3).

### 4.7 Production and defense industry

**4.7.1 Learning curve** (Wright cumulative average [V: DoD standard, m1-findings §3.3] with flattening):

```
A(N) = c1 · N^b · ((1 + N/N_f)/(1 + 1/N_f))^(−b),    b = log2(slope)
T(N) = N · A(N);   lot cost for q units after N = [T(N + q) − T(N)] · (r/r_ref)^(b_r) · producibility
```

The local slope b/(1 + N/N_f) diminishes with N, so A flattens to a plateau ≈ c1·(N_f + 1)^b instead of falling forever [D form; motivation: the flattening effect and Boone's diminishing-rate curve, which fits 169 DoD end items better than Wright [V: Elshaw et al., AFIT-AM-20-161]; Boone's own functional form was not checked]. Marginal cost A·(1 + b/(1 + N/N_f)) > 0 for every slope > 50%, so T is increasing [V: computed for slopes 0.75–0.95, N_f 10–1,000, N ≤ 2×10⁵]. Fixture: c1 = 100, slope 0.8, N_f = 500: units 11–20 cost 292.36; A(500) = 16.895 [V: computed].

| Domain | Slope [R] | N_f [D] | Rate slope b_r [R/D] | Lead time to first delivery [D] |
|---|---|---|---|---|
| Air | 0.80–0.85 (m1-findings) | 500 | log2(0.95) | 24 months |
| Surface, subsurface | 0.88 | 20 | log2(0.95) | 36 + 0.004·tonnes months |
| Ground vehicles | 0.88 | 2,000 | log2(0.95) | 12 |
| Munitions | 0.85 | 5,000 | log2(0.93) | 12 |
| Launch vehicles, spacecraft | 0.92 | 50 | log2(0.97) | 18 / 24 |

**4.7.2 Rate, capacity and setup.** `Procure` sets a rate r (units/month); r_ref = the design's content reference rate. Tooling cost at setup = 6 · c1 · r [D], spread over the lead time. Monthly lot value is WS-03 sector demand; WS-03 returns a fill rate φ ≤ 1 (ECO-015), and next month's deliveries are r·φ (fractional units accumulate; whole units are delivered). Producibility ~ LogNormal(1.10, CV 0.20 + 0.20u_design) [D], drawn at line start and revealed at the first lot.

**4.7.3 Revisions.** A production line switched to a revised design keeps N_eff = N·(1 − 0.5χ) [D] (χ = changed cost share) and uses slope b·1.2 for its next 0.2·N_f units, the steeper post-change learning AFIT found [V: AFIT AD1016823, qualitative].

**4.7.4 O&S.** Non-personnel O&S per unit-year = f_os · A(N_ref) · (1.02)^age, f_os = 0.04 [D], age in years. Over 30 years this totals 0.04 · 40.6 ≈ 1.6× acquisition, so O&S is ≈ 62% of lifecycle cost before personnel [D: arithmetic; the commonly cited ≈ 70% share including personnel is [R]]. Foreign-sourced designs pay 50% of O&S to the supplier in FX [D].

### 4.8 Market and contractors

**4.8.1 Paths.**

| Path | Price (seller currency) | Lead | Dependency score [D] | Other effects |
|---|---|---|---|---|
| In-house | own development + production | full | share of imported components | — |
| Spot buy | listing price, or mc × 1.5 | 1–3 months | 1.0 | quantity ≤ listing or 5% of seller's 12-month output |
| Arms deal | mc × 1.25 per delivery | seller lead + rate share | 0.8 | `ArmsDeal` event (WS-06 relations); the seller's N grows |
| License | fee 0.15 × design dev effort; royalty 5% of buyer lot cost | `LicenseSetup` project: effort 0.25 × dev, months 0.5 × original, u = 0.33 | 0.3 (unlicensed sub-components stay imported) | buyer c1 = seller c1 × 1.3 × (wage ratio)^0.5; tech transfer: perf/cost precision 100, m_sci → 1, viability LR 50 |

`mc` = the seller line's marginal unit cost at its current N; × (1 + 0.5·max(0, utilization − 0.8)) [D]. War Footing's ordering holds: deals are cheaper than spot buys, licensing is slow but cheapest per unit at volume [V: m1-findings §3.4]. With `component_rights` (×1.5 fee), licensed components can go into the buyer's own designs with `Licensed` provenance.

**4.8.2 Seller behavior (minimal AI).** A request is accepted iff `export_policy[buyer] ≥ sensitivity(item)`, the buyer isn't embargoed (`a_ij ≥ 0.2`), and, for deals, the seller line has spare rate (otherwise the earliest start month is offered and auto-accepted if within 12 months) [D]. Sensitivity = hull class tier (content; e.g., SSBN and ICBM `Never`, fighters `Sensitive`, trucks `Surplus`) + 1 if any tech in the design was first fielded in the world < 5 years ago. Policy tiers: `None < Surplus < Standard < Sensitive < License`. Player sellers use the same rule with their own policy (OQ-R5).

**4.8.3 Provenance and embargo.** Every slot fill carries `Provenance`. If WS-06 access from the buyer to an origin polity drops below 0.2: production lines of designs with imported components from it halt (`DeliveryHalted`) until a `ReviseDesign` swaps the component (a spec change with its rework); open deals with it stop delivering; and foreign O&S shares are unfunded (§3.6). `arms_dependency` is reported to WS-04 (§3.8).

**4.8.4 Second-hand.** WS-11 `Sell` lists units at A(N_ref) × (remaining life share) × 0.6 [D] as spot listings.

**4.8.5 Contractors.** Each high-fidelity polity has contractors (setting content `contractors.ron`, optional; else two per domain cluster are generated with placeholder names from the polity name) with competence c_d ∈ [0.5, 1.5] per domain, start 1.0 [D]. A project's contractor scales G (§4.5.3). c_d += 0.05 per completed project in d, and decays 1%/year toward 0.5 without any project in d [D]: design teams atrophy. A contractor's export catalog is its home polity's designs in its domains that the home policy permits. Revenue is the home polity's arms export receipts; contractors have no finances in the MVP.

**4.8.6 Reverse engineering.** `ReverseEngineer` (≥ 1 unit held) is a project with effort 0.5 × the original design effort (0.25 with stolen blueprints), u from the polity's own TRLs; the result is a domestic copy, and the techs inside get strong observations (perf precision 100).

**4.8.7 Macro buyers.** Macro polities import a share 1 − ss_d of their procurement per domain, ss_d = clamp(L_d/L_front,d, 0, 1) × domestic capacity share [D]. Imports are allocated to sellers ∝ policy-allowed × alignment weight (WS-06) × quality^η (η = 2) of the seller's newest exportable design in the domain; units = spend / price. Sellers book receipts and add the units to their line's N. This gives the market about 40 buyers without macro AI.

### 4.9 Civil technology effects (WS-03)

After `Proven`, adoption follows a logistic a(τ) = 1/(1 + exp(−(τ − 8)/2)) in years since proof [D], or imitation of a tech proven elsewhere at half the rate, scaled by WS-03 access-weighted openness. Each tech's `econ_effects` list maps to sectors and indices with effect e × perf_true. `M_s = exp(Σ_t e_ts(a_tp − ā_t))`, with ā_t the world mean adoption over all polities (12-month lag, like WS-03's A_frontier), so world-level growth stays on WS-03's calibrated `g_front` and WS-05 supplies only relative advantage (OQ-R1). Health, contraception and skill-bias indices use the same form.

### 4.10 Macro tier (P-7, SYS-006)

Macro polities carry `MacroRnd { level: [f64; 12] }` (one per domain, in generation-equivalents) and hold **no tech belief subjects** (P-02a):

```
L_d' = min(L_front,d + 0.5,  L_d + (κ_m · √(rnd_share_of_GDP) + θ_m · max(0, L_front,d − L_d)) / 12)     κ_m = 4, θ_m = 0.08/yr [D]
L_front,d = max over high-fidelity polities of their level in d (12-month lag)
level of a high-fidelity polity in d = Σ_{t ∈ d} prog_t / n_lines,d,   prog_t = clamp((TRL_t − 1)/5, 0, 1)
```

`MacroRnd.level` feeds WS-11's macro equipment quality (`step_d^L`). Demotion sets L_d to the formula above. Promotion walks each domain's working techs in D order, setting them `Proven`, until the sum reaches L_d·n_lines,d, gives the last one a fractional TRL through m_sci (or a development project at progress p), and seeds beliefs with the band's observations; L_d is conserved within 1e-6 (ARC-081 pattern).

### 4.11 RNG and explainability

| Draw | Stream |
|---|---|
| Truth | `worldgen_stream(seed, line or tech ID or edge pair key, purpose ∈ {VIAB_LINE, VIAB, PERF, COST, EDGE})` |
| Discovery, Boost reveal | `{tag "rnd", entity: polity << 16 | field, purpose: DISCOVER / REVEAL}` per tick |
| Observation noise | `{tag "rnd", entity: polity << 16 | field, purpose: OBS}` per tick (sorted subject order) |
| Project draws (E, G phases, S, tests) | `{tag "rnd", entity: polity << 32 | project seq, purpose: START}` once, at start |
| Producibility | `{tag "rnd", entity: polity << 32 | line seq, purpose: PROD}` once |
| Estimate Monte Carlo | `{tag "rnd.est", entity: polity << 32 | draft hash, purpose: 0}` (pure functions use a local factory; not world state) |

Breakdowns: EAC (§4.5.4), unit cost (learning × rate × producibility × factors via `Breakdown::product`), `M_s` (one term per tech), slots (budget terms), field effort (line terms), design stats (§4.6.3), dependency (per supplier).

### 4.12 Performance and memory (SYS-008, ARC-091)

Budget: R3 WS-05 ≤ 15 ms (WS-02 §5). Estimate [D]: per high-fidelity polity ~300 techs × a few flops, ≤ 20 projects, ≤ 40 lines, ~20 RNG streams per tick (≈ 6 µs); macro polities 12 updates each; own and published observations total ≈ 4,500 per month after field staggering (≈ 0.5 ms in I1). Expected ≤ 3 ms. Memory: tech beliefs 6 polities × 900 subjects × ~80 B ≈ 0.43 MB (down from ≈ 3 MB if all 46 polities held them); `RndBlock`s ≤ 0.25 MB per high-fidelity polity; total ≤ 2 MB.

## 5. Requirements

| ID | Requirement | Acceptance criterion (testable) | Trace |
|---|---|---|---|
| **Truth and worldgen** ||||
| RND-001 | Tech lines expand into generation techs with deterministic IDs, dense indices, dates and costs. | Fixture line (gens 4, first 1950, step 10, base_dev 100, step_dev 1.6): IDs `tech.x.g1..g4`, D = 1950/60/70/80, base_dev = 100/160/256/409.6 (exact); shuffling content files gives identical indices (ARC-053). | P-6, R-3 |
| RND-002 | Truth per tech depends only on (seed, content ID) and is identical for every polity. | Adding an unrelated line leaves all existing `TechTruth` and edges bit-identical; every polity's view of truth is the same object (ARC-011/034). | SYS-005, P-1 |
| RND-003 | Historical techs (D ≤ scenario start) always work and have nominal-mode performance. | Over 1,000 seeds, every historical tech has works = true and perf within the nominal component's ±4σ. | P-1 |
| RND-004 | Viability and performance truth follow the content priors. | 10,000 worlds: share working per class within ±1.5 pp of the Beta mean; perf mode frequencies within ±1.5 pp of (0.15, 0.75, 0.10); ln κ mean within ±0.01 and sd within ±0.01 of (0, σ_κ). | SYS-005 |
| RND-005 | Enabler edges follow §4.1.2 and are acyclic. | 1,000 worlds with default content: mean edge count within 3 SE of Σ p; zero cycles in Requires ∪ prerequisites; adding a line changes no edge between other lines. | P-1 |
| RND-006 | Dead-end frequency matches the content class mix. | 10,000 worlds: mean dead ends per world within 3 SE of Σ (1 − class mean) over non-historical techs (≈ 48 with default content); M2(b) B1 reports p5–p95. | §0 |
| **Beliefs** ||||
| RND-010 | Each known tech has the three §4.2.1 subjects with priors equal to the sampling distributions; unknown techs are `None`. | Unit: subjects created at discovery equal the content prior component-for-component (1e-12). | P-2, ADR-0008 |
| RND-011 | Binary evidence reproduces the exact Bernoulli posterior. | For priors {0.1, 0.4, 0.8} × LR {1/20, 1/3, 3, 13.6, 50}: `p_works` after one observation equals p·LR/(p·LR + 1 − p) within 1e-6. | P-2 |
| RND-012 | Beliefs are calibrated. | Batch (≥ 200 worlds × 6 high-fidelity polities, 1955–2060, B2 policy): 80% credible intervals of perf and cost contain truth in 80 ± 3% of (polity, tech, quarter) samples; in `p_works` deciles, the observed working frequency is within ± 5 pp of the decile mean. | SYS-005 |
| RND-013 | Beliefs converge toward truth as research increases. | B2: median perf σ ≤ 0.80 × prior after one full science phase and ≤ 0.45 × prior after development completes; median |ln(mean/truth)| decreases monotonically in cumulative effort deciles; dead-end p_works < 0.10 in ≥ 90% of cases after 2 failed tests at u ≤ 1. | SYS-005 |
| RND-014 | Observations are pooled: at most one per subject per quarter (plus one per test), with precision equal to the accrued sum. | Unit: 3 months of accrual Δτ = 5, 7, 11 yields one observation with noise 1/√23 (1e-12); no subject emits twice in a tick (ARC-033 holds). | P-2, SYS-008 |
| RND-015 | Fielding is public. | A stub design's first delivery makes every other high-fidelity polity know its techs and gives each viability LR 50 and one perf observation with σ = 0.25/(1 + intel_q). | P-2 |
| RND-016 | Cross-generation inference follows §4.2.5. | Proving g_k pushes LR 3 to g_{k+1}; p_works(g_k) < 0.02 pushes LR 1/3; nothing else does. | P-2 |
| RND-017 | Espionage never makes the thief more certain than the target. | Repeated `Field` thefts (×10) against a fixed target leave the thief's τ on every subject ≤ τ_target/k_spy² + own τ (1e-9); `stolen` records the copied amount. | P-2, WS-06 |
| RND-018 | Science from unclassified civil lines is published with σ×√2; defense and classified science is not. | Two-polity fixture: B receives exactly A's civil-field emissions with σ ×√2 and none of A's defense-line or classified emissions. | P-2 |
| RND-019 | Decide-stage code can't read truth or hidden project draws. | `compile_fail` doc tests: no `PolityView` accessor returns `TechTruth`, `EdgeTruth` or `ProjectDraws`. | ARC-012, P-1 |
| **Science and discovery** ||||
| RND-020 | Field effort is split by priority and converts to maturity and precision per §4.3. | Unit: e_f = 10, priorities (1, 3), equal weights, E_sci = 20 → Δm = (0.125, 0.375); classified → precision × 0.8; rnd_morale 1.1 scales both by 1.1 (1e-12). | §4.3 |
| RND-021 | Premature work costs exp(Δyears/τ_p). | E_sci for a tech 8 years early = e × base value (1e-12); 0 years early = base. | §4.3 |
| RND-022 | Discovery follows the hazard in §4.4 and needs known prerequisites. | 10⁵ trials at fixed effort: monthly discovery frequency within 3 SE of h_t; zero discoveries when a prerequisite is `Unknown` or year < D_t − 10. | §4.4 |
| RND-023 | Requires edges block at progress 0.7 with a field hint and never push viability evidence. | Fixture: B requires A; B's project stops at 0.7 with `Blocked { field(A) }`, B's viability belief is unchanged; after A is `Proven`, the project resumes and the edge is revealed. | §4.4, RND-012 |
| RND-024 | Boost edges cut effort and are revealed per §4.4. | Fixture: A proven, b = 0.3: B's true effort = 0.7 × unboosted (1e-12); revealed at B's first test; reveal hazard in the passive path within 3 SE over 10⁵ trials. | §4.4 |
| **Projects** ||||
| RND-030 | Slots = clamp(round(√(B/B_slot)), 1, 20); new projects beyond slots are rejected. | B/B_slot ∈ {0, 1, 4, 25, 1000} → {1, 1, 2, 5, 20}; a 6th project at 5 slots → rejection event; a slot drop cancels nothing. | §4.5.1 |
| RND-031 | The funding response follows §4.5.2. | x = 0.5 → w = 0.375 R*; x = 1 → R*; x = 2 → R*(1 + 0.5 ln 2); x ≤ 0.2 → 0; x = 4 is clamped to 3 (all 1e-12). | §0 |
| RND-032 | Execution growth uses the §4.5.3 distributions, drawn once at start from the project stream. | 10⁵ projects at u ∈ {0, 0.67, 1, 1.67}: mean and CV of G and S within 3 SE of target; corr(ln G, ln S) = 0.6 ± 0.02; draws unchanged when other subsystems draw more (ARC-032). | §4.5.3 |
| RND-033 | Cost growth is calibrated to the anchors. | B3 (10⁴ design projects, default start mix, funded at the efficient rate x = 1): realized/baseline mean 1.03 ± 0.03 at u = 0 and 1.35 ± 0.05 for u > 0; overall CV 0.80 ± 0.10; overall mean within [1.25, 1.40]; schedule mean within [1.10, 1.30]. | m1 §3.2 |
| RND-034 | EVM quantities and the EAC breakdown are exact. | Every tick of a fixture project: EV, AC, CPI and EAC follow §4.5.4; EAC breakdown terms sum to EAC within ARC-041 tolerance; at completion EAC = AC. | SYS-004, P-3 |
| RND-035 | Tests follow §4.5.5; dead-end projects never complete. | 10⁵ tests: working-tech failure frequency within 3 SE of 0.15 + 0.25u; dead-end pass frequency 0.05 ± 3 SE; zero completions of dead-end projects over 10⁴ attempts. | §4.2.3 |
| RND-036 | Completion proves the tech and unlocks its consequences. | On completion: status `Proven`, components placeable without concurrency, `grants` set, next generation `Known`, one cost observation σ 0.05. | §4.5.5 |
| RND-037 | Spec changes rework per §4.5.6. | p = 0.3, χ = 0.2 → Δp = 0.03 and 1 month halt; p = 0.6 (post-freeze) → Δp = 0.12; the changed share has a new phase draw. | §0 |
| RND-038 | `estimate()` is calibrated. | B3: projects budgeted at P80 exceed it in 20 ± 4%; at P50 in 50 ± 5%; `estimate` is identical across repeated calls with the same view. | §0 |
| RND-039 | Cancelling a project refunds nothing; evidence already gained stays. | Fixture: cancel at p = 0.5: AC unchanged, beliefs unchanged, slot freed next tick. | §0 |
| **Design** ||||
| RND-050 | Designs are built from 53 hull classes in 7 domains with typed slots. | Content validation: every hull class has a domain, target class, ≥ 1 slot and a sensitivity tier; every component's slot kind exists. | §4.6.1 |
| RND-051 | Feasibility rejects drafts that break slot, mass or power limits and names the binding constraint. | Three fixture drafts (wrong slot kind, 1% over mass, 1 kW short) are rejected with those reasons; a feasible draft passes. | §4.6.3 |
| RND-052 | Derived stats follow §4.6.3 ops and laws with exact breakdowns. | Fixtures reproduce `StagedPayload` 1.460 t and 7.328 t, `LogMassRatio` 2.841 km/s (1e-3 rel); `sum`, `max`, `min`, `product` unit tests; every stat breakdown sums exactly (ARC-041). | P-3, P-10 |
| RND-053 | Component stats scale with the true performance and cost of their tech. | Doubling a tech's truth perf in a fixture doubles every perf-scaled stat of its components; κ scales unit cost (1e-12). | P-1 |
| RND-054 | Specified stats use the owner's beliefs; achieved stats use truth and are revealed at acceptance. | Fixture with perf truth 0.8 and belief mean 1.0: `specified` speed uses 1.0, `achieved` (after acceptance) uses 0.8; WS-11 reads `achieved`. | P-1, P-2 |
| RND-055 | `believed_stats` for another polity's design uses only the viewer's beliefs. | Two viewers with different beliefs compute different stats for the same design; neither changes when the design's truth changes but beliefs don't. | P-2, WS-07 |
| RND-056 | Stored design stats equal a fresh recomputation, and placement never affects them. | For every design after a 120-tick run, `design_stats()` equals the stored values bit-for-bit; permuting `placement` changes no stat and no `DesignStats` hash (OQ-3). | SYS-004, OQ-3 |
| RND-057 | `DesignStats` fill every field WS-11 needs for the salvo and Lanchester inputs. | For one fixture design per domain, every §3.6 field required for its domain is present (per-domain required-field table in content), with no NaN. | SYS-009, WS-07 |
| RND-058 | Munition deliveries carry the delivering design's stats as a lot. | Two munition designs of one class deliver two lots with their own stats; a later third design doesn't change existing lots. | salvo §7.4 |
| RND-059 | Launch vehicles and spacecraft publish the WS-08b fields; capability flags and v_e follow §3.7. | Fixture: proving a tech with `grants: [cap.lander]` sets the flag; v_e = max over proven propulsion techs of base × perf_true; LV stats include payload_leo. | WS-08b P-05a |
| **Production** ||||
| RND-065 | Lot costs follow §4.7.1. | c1 = 100, slope 0.8, N_f = 500: units 11–20 = 292.36 and A(500) = 16.895 (1e-4 rel); T(N) strictly increasing for N ≤ 10⁵ at slopes 0.75–0.95. | m1 §3.3 |
| RND-066 | Rate effects scale lot cost by (r/r_ref)^b_r. | r = 2 r_ref with b_r = log2(0.95) → × 0.95 (1e-12). | §4.7.2 |
| RND-067 | Deliveries respect lead time and WS-03 capacity. | First delivery at start + lead time; with fill rate 0.5, deliveries are 0.5 r per month (fractions accumulate, whole units delivered); total demand value reported to WS-03 equals Σ lot values. | ECO-015 |
| RND-068 | Producibility is drawn once and revealed at the first lot. | Distribution test over 10⁵ lines (mean, CV within 3 SE); `unit_cost` breakdown gains a producibility term at first delivery. | §4.7.2 |
| RND-069 | Revisions reset learning per §4.7.3. | N = 100, χ = 0.4 → N_eff = 80; the next 0.2 N_f units use slope b·1.2. | §0 |
| RND-070 | O&S follows §4.7.4. | Default values: 30-year O&S = 0.04 × Σ 1.02^k × A_ref (1e-12); O&S share of lifecycle cost before personnel in [0.55, 0.70]. | §0 |
| RND-071 | A quantity cut raises unit cost (death spiral reachable). | Fixture: cutting planned quantity from 500 to 200 raises program average unit cost (vs. the original plan) by the analytic A(200)/A(500) ratio (1e-9). | §0 |
| **Market** ||||
| RND-080 | Quotes follow §4.8.1 prices, leads and dependency scores. | Fixture seller (mc = 10, utilization 0.9): spot 15 × 1.05; deal 12.5 × 1.05; license fee 0.15 × dev effort; scores 1.0/0.8/0.3. | §4.8 |
| RND-081 | Sellers accept by policy tier, embargo and capacity per §4.8.2. | Matrix test over tiers × sensitivities × access {0.1, 0.5}: accept iff policy ≥ sensitivity and access ≥ 0.2; a deal beyond spare rate gets the earliest start offer. | D5 |
| RND-082 | Arms deals deliver from the seller's line, pay through WS-03 FX, raise the seller's N and emit `ArmsDeal`. | Two-polity fixture: 10 units at rate 1/month: seller N grows by 10, buyer reserves drop by Σ price × e (ECO-071), event once. | WS-03, WS-06 |
| RND-083 | Licenses run a setup project, then domestic production with royalties and tech transfer. | Fixture: setup effort 0.25 × dev; after it, buyer c1 = 1.3 × seller c1 × wage ratio^0.5; royalty = 5% of each lot; licensed techs m_sci = 1 and viability LR 50. | §4.8.1 |
| RND-084 | Provenance is tracked per slot and drives embargo effects. | Embargo on the origin of one imported component halts the line (`DeliveryHalted`), sets that O&S share unfunded, and a `ReviseDesign` swapping it restores deliveries after its rework. | §4.8.3 |
| RND-085 | `arms_dependency[j]` is the 12-month value share dependent on j. | Fixture with known flows: values match by hand (1e-12); Σ_j ≤ 1. | WS-04 |
| RND-086 | Contractor competence scales growth and evolves per §4.8.5. | c = 1.5 → mean excess and CV ÷ 1.5 (1e-12); +0.05 per completed project; 10 idle years → c decays by 1%/yr to the 0.5 floor. | §0 |
| RND-087 | Reverse engineering works per §4.8.6. | Requires a held unit (else rejected); effort 0.5 × (0.25 with blueprints); result has domestic provenance. | §4.8.6 |
| RND-088 | Macro buyers allocate imports per §4.8.7. | Fixture: 2 sellers, alignment weights (2, 1), equal quality → spend split 2:1; seller N rises by units bought. | §4.8.7 |
| **Interfaces** ||||
| RND-090 | WS-03 line demand and funding follow §3.4. | Stub WS-03 paying 50%: every project's and field's real funding is half its request, deflated by the cost index; demand equals Σ requests. | ECO-092 |
| RND-091 | `M_s` and indices are relative to the world mean per §4.9. | Two-polity fixture with symmetric adoption: M_s = 1 for both (1e-12); one-sided adoption a = 1, e = 0.05: M_s = exp(0.025) and exp(−0.025). | ECO-092, OQ-R1 |
| RND-092 | Espionage yield follows §4.2.6 and transfers maturity on the next R3. | Fixture: `Tech` theft → thief m_sci = max(own, 0.7 × target) after the next R3, observations applied in I1 of the theft tick. | WS-06 |
| RND-093 | Deliveries reach WS-11 and WS-08 in the same tick. | A delivery produced in R3 is visible to stub WS-11 and WS-08 functions in R5b of the same tick. | WS-02 §3.3 |
| RND-094 | The fissile multiplier follows P-05b. | Self-sufficiency 0 → nuclear-tagged costs × 2; 1 → × 1 (1e-12). | WS-08b P-05b |
| RND-095 | WS-08a `tech.level` and `tech.nuclear` map per §3.9. | Each band yields the stated Proven/Known sets and observation counts on a fixture content; `Program` → running project at p = 0.3; same band → identical start. | WLD-022 |
| **Tiers, explainability, determinism, performance** ||||
| RND-100 | Macro polities run §4.10 and hold no tech belief subjects. | Unit equations within 1e-12; promote then demote conserves every L_d within 1e-6; a macro polity has zero tech subjects. | SYS-006, P-7 |
| RND-101 | Every WS-05 `Tracked` variable has a breakdown. | Property test over WS-05 keys after a 120-tick run (ARC-041). | SYS-004 |
| RND-102 | WS-05 is deterministic and order-independent. | Two 240-tick runs with the same seed and inputs give identical hashes; reordering projects in the input log within one tick gives identical outcomes. | SYS-002 |
| RND-103 | WS-05 stays within its time and memory budgets. | `bench`: R3 WS-05 median ≤ 15 ms on the reference machine; WS-05 share of the `World` postcard ≤ 2 MB. | SYS-008 |

## 6. Content/data

### 6.1 Files

```
content/core/rnd/
  fields.ron          Field { id, name, e_ref }                                      10 entries
  domains.ron         Domain { id, name, structure_line, laws: [...], required_stats: [...] }   12 eng. + 7 craft
  lines/<domain>.ron  TechLine { ... } and Tech { ... } (singletons)                 ≈ 75 lines + ≈ 20 singletons
  components.ron      ComponentTemplate { id, line, slot, size, mass, power, cost, contrib: [(stat, op, base, step)] }  ≈ 90
  hulls/<domain>.ron  HullClass { id, domain, target_class, slots, mass_cap, base_stats, sensitivity, lead, slope, n_f }   53
  enablers.ron        [(from, to, p, kind)]                                          ≈ 40
  params.ron          every [D] constant in §4 (one file, tagged comments)
content/settings/<s>/contractors.ron, rnd_start.ron   (optional)
```

### 6.2 Tech line schema (example)

```ron
TechLine(
    id: "line.prop.turbofan",
    domain: "dom.air_propulsion",
    fields: [("field.fluids", 0.6), ("field.materials", 0.4)],
    generations: 5, first: 1958, step_years: 11,
    base_dev: 900.0, step_dev: 1.5,                // real RC (millions)
    viability: Established,
    perf: Default, cost_sigma: Default,
    prereq: [("line.mat.superalloy", 1)],          // line, generation offset
    components: ["comp.engine.turbofan"],
    econ_effects: [(Sector("services"), 0.004)],   // civil aviation spillover
    grants: [],
    tags: [],                                      // e.g. "fissile"
)
```

### 6.3 Granularity and authoring volume (Decision 1)

| Engineering domain | Nodes | Example lines |
|---|---|---|
| Air-breathing propulsion | 20 | piston (hist.), turbojet, turbofan, turboshaft, ramjet/scramjet |
| Rocketry and space propulsion | 28 | solid, storable, cryogenic, staged combustion, NTR, electric, reusability |
| Aerostructures and aerodynamics | 24 | airframe gens, swept/delta, VTOL, rotorcraft, low observability, hypersonic glide |
| Naval architecture | 26 | hull forms, steam, gas turbine, naval reactor, AIP, quieting, VLS |
| Ground vehicles | 20 | armor, composite armor, engines, fire control, active protection |
| Sensors and EW | 30 | radar, phased array, sonar, EO/IR, ESM, ECM |
| Computing and communications | 28 | transistor, IC, microprocessor, networks, satellite comms, autonomy, crypto |
| Weapons and guidance | 32 | inertial, radar seeker, IR seeker, GPS-class nav, warheads, guns, DEW |
| Nuclear | 18 | fission weapon, thermonuclear, MIRV, naval and civil reactors, fusion |
| Space systems | 26 | satellite bus, life support, reentry, ISRU, survey instruments, docking |
| Materials and manufacturing | 24 | alloys, superalloys, composites, ceramics, automation, additive |
| Civil and biomedical | 24 | green revolution, antibiotics/vaccines, contraception, containerization, energy |
| **Total** | **300** | |

Science fields (10): physics, nuclear physics, chemistry, materials science, fluid dynamics, solid-state electronics, computer and information science, life sciences, earth and planetary sciences, astronomy.

Hull classes (53): **air** 9 (fighter, strike, bomber, AEW&C, maritime patrol, transport/tanker, reconnaissance, helicopter, UAV) · **surface** 9 (patrol/corvette, frigate, destroyer, cruiser, carrier, amphibious, mine warfare, replenishment, fast attack craft) · **subsurface** 3 (SSK, SSN, SSBN/SSGN) · **ground** 6 (MBT, IFV, APC, self-propelled artillery, air-defense vehicle, TEL) · **munition** 14 (the salvo §8.4 classes plus ICBM/SLBM) · **launch vehicle** 4 (small, medium, heavy, super-heavy) · **spacecraft** 8 (comsat, ISR, navigation, early warning, survey probe, crewed, lander, tug).

Authoring: ≈ 75 lines + 20 singletons + 90 components + 53 hulls + 40 enablers + 10 fields ≈ 290 entries, against ≈ 600+ for hand-authored nodes. A content-check test (like WS-08a's) validates references, dates, cycles and the domain node counts.

## 7. Verification

- Unit and integration tests named `rnd_NNN_*`, one or more per requirement.
- **M2(b) prototype** (`prototypes/rnd/PLAN.md`): Python + NumPy, vectorized over worlds and polities. Scenarios B1–B8 produce the three required outputs over 1955–2060 (belief convergence, dead-end frequency, cost-overrun distributions) and evaluate RND-004/005/006/012/013/033/038 at full batch size, plus the AI kill-threshold sweep that sets the default AI policy.
- **Batch stability (WS-10):** in 200 × 1955–2060 runs with the B5 portfolio policy: every WS-05 `f64` finite; ≥ 60% of non-dead-end techs with D ≤ 2040 proven by some polity by 2060; median proof date of funded Established techs within ±6 years of D [D]; no polity's dead-end spending exceeds 25% of its cumulative R&D [D].
- **Calibration seed (exists):** `python prototypes/rnd/calib_growth.py` reproduces the §4.5.3 fit and the RND-011 equivalence.

## 8. Edits to other documents (proposed, not made)

| ID | Document | Edit | Why |
|---|---|---|---|
| P-02a | WS-02 §3.2, §5 | Subject index ranges are assigned per subsystem at worldgen in a fixed order (WS-03 structural, WS-05 tech, WS-08b deposits). A macro polity's `BeliefBlock` may omit a subsystem's range (lookup returns `None`). Memory mix: tech beliefs ≈ 0.43 MB | §4.10, §4.12 |
| P-03a | WS-03 §2.3, §3.1 | `M_s` is relative to the world mean (≈ 1 on average); `g_front` stays the world trend (if OQ-R1 is accepted) | §4.9 |
| P-03b | WS-03 §3.6 | R&D, space and procurement lines pay min(appropriation, WS-05 demand) and report a funded fraction, like the WS-11 lines. The space-program line prorates between WS-05 (projects, hardware) and WS-08 (operations) demand | §3.4 |
| P-03c | WS-03 §2.2–2.3 | Add the skill-bias index to the WS-05 inputs row (WS-03 §3.5 already reads it); note that WS-05 doesn't read the skilled-labor pool | §3.4 |
| P-04a | WS-04 §3.4, §6.4 | `gate_orders` covers `RequestAcquisition` and `SetExportPolicy` (ethics-box limits only, PC 0); a policy-map row for export openness (channel `none`; axis to WS-04's choice) | GOV-021, GOV-064 |
| P-06a | WS-06 | Use `rnd::espionage_yield`; set AI export policy tiers (suggested: alignment ≥ 0.8 `License`, ≥ 0.5 `Sensitive`, ≥ 0 `Standard`, ≥ −0.2 `Surplus`, else `None`); supply `intel_q`; read `DesignFielded`, `ArmsDeal`, `LicenseGranted`, `DeliveryHalted` | §3.5 |
| P-11a | WS-11 | Build unit aggregates from `DesignStats.achieved`; consume `deliveries`; pay non-personnel O&S from §4.7.4 with the foreign share and embargo flag; refit = `ReviseDesign`; `Sell` → spot listing; macro equipment quality from `MacroRnd.level` | §3.6 |
| P-SAL | salvo note §8 | `DesignStats` field names (§3.6) are the source of §8.1–8.4 unit inputs; ground units add `GroundStats` (G-6) | §3.6 |
| P-08b | WS-08b §8 | Mark P-05a/b as accepted with the §3.7 forms | §3.7 |
| P-CON | ConOps §6 R&D row | "P-1 latent truth; science fields vs. engineering projects; TRL-scaled lognormal cost and schedule growth; beta line viability; mixture performance; hidden enablers; funding-scaled slots" | §2 |

## 9. Open questions

| ID | Question | Default if unanswered |
|---|---|---|
| OQ-R1 | Civil tech effects on WS-03: **relative** to the world mean (recommended: WS-03's calibration is untouched, and techs give advantage, not world growth), or **absolute** (a world where computing disappoints grows slower, but WS-03 must recalibrate `g_front` and its catch-up term against WS-05)? | Relative |
| OQ-R2 | Macro polities hold domain levels only and no per-tech beliefs (recommended: −2.5 MB of snapshot, and macro polities don't design). They can't be espionage targets for specific techs, only for domain levels. | Levels only |
| OQ-R3 | The m1-findings ACAT triple (mean 1.36, CV 0.80, p85 3.1) is inconsistent: a lognormal with mean 1.36 has p85 ≤ 2.33 for any CV [V: computed]. Please check the source (Defense ARJ 32(1), p. 47); the p85 may be for a different measure. The fit uses mean and CV. | Fit mean and CV; treat p85 as unverified |
| OQ-R4 | Contractors: domestic AI firms with competence and catalogs only (recommended). Private-venture export designs (a contractor designs on its own money, e.g., for export) are post-MVP. | Post-MVP |
| OQ-R5 | When the player is the seller, requests are decided automatically by the player's export policy, with a notification (recommended), not a per-request prompt. | Automatic |

## 10. Sources

- GAO-06-391, *Defense Acquisitions: Assessments of Selected Major Weapon Programs* (2006): RDT&E cost growth 34.9% (immature technology at start) vs. 4.8% (mature). https://www.gao.gov/assets/gao-06-391.pdf [V]
- GAO-07-406SP (2007): 32.3% vs. 2.6%; fully mature technologies in 16% of systems at development start. https://www.gao.gov/assets/a258435.html [V]
- GAO-08-467SP (2008): 63% of programs changed requirements after development start; average delays in initial capability of 16–21 months. https://www.gao.gov/assets/a274174.html [V]
- RAND (1971), DTIC AD0730921: typical programs ≈ 40% cost growth, ≈ 15% schedule slip, performance deviating 30–40% from specification. https://apps.dtic.mil/sti/pdfs/AD0730921.pdf [V: search excerpt]
- RAND RB236: completed programs grew ≈ 46% over the Milestone B estimate. https://www.rand.org/pubs/research_briefs/RB236.html [V]
- Defense ARJ Vol. 32 No. 1 (2025): ACAT I lognormal fit (via m1-findings §3.2; see OQ-R3). https://online.flippingbook.com/view/638780396/47 [V via m1-findings]
- Elshaw et al., *A New Learning Curve for DoD Acquisition Programs: How to Account for the "Flattening Effect"*, AFIT-AM-20-161 / SYM-AM-21-112: Boone's diminishing-rate curve fits 169 DoD end items better. https://www.dair.nps.edu/bitstream/123456789/4419/1/SYM-AM-21-112.pdf [V]
- Moore (2015), AFIT thesis: Wright cumulative-average model as DoD standard (via m1-findings §3.3). https://apps.dtic.mil/sti/pdfs/ADA616177.pdf [V via m1-findings]
- AFIT AD1016823: steeper learning slope after configuration changes (via m1-findings §3.3). https://apps.dtic.mil/sti/tr/pdf/AD1016823.pdf [V via m1-findings]
- Terra Invicta wiki and DD #10; War Footing Dev Blog #4 (via m1-findings §3.1, §3.4) [V via m1-findings]
- EVM CPI method, CPI stability after 20% complete (Christensen) [R]; Breguet, Tsiolkovsky, Admiralty coefficient [R: textbook]
