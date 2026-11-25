# WS-02 — Architecture & Data Model
Status: Draft · Parent: ConOps §5, §7, SYS-002, SYS-003, SYS-004, SYS-006, SYS-007, SYS-008 · Last updated: 2026-11-22

Tags: **[V]** verified (source or measurement given) · **[R]** recalled, not checked · **[D]** design choice.
Decisions are recorded in ADR-0004 to ADR-0009. This spec is the contract that WS-03 to WS-09 code against.

---

## 1. Purpose & scope

WS-02 defines the skeleton of the simulation: the Cargo workspace, the world-state data model, the monthly tick pipeline, the RNG, explainability, content loading, save/load and replay, the CLI, fidelity tiers, and the performance budget.

**In scope (MVP):** everything listed above, plus no-op stub subsystems so the pipeline, breakdowns, hashing and batch runner can be tested before WS-03 to WS-05 exist.

**Out of scope:** every subsystem's model equations (WS-03 to WS-08); the meaning of specific orders, events and variables; any UI beyond a Tauri hello-world (WS-09); save migration between format versions (post-MVP); cross-platform bit-identical determinism (§7, OQ-A1 in §8).

## 2. Decisions summary

| # | Decision | Recommendation | ADR |
|---|---|---|---|
| 1 | ECS vs. plain structs | **Plain structs of `Vec`s indexed by typed IDs.** No ECS. | 0004 |
| 2 | Breakdown storage | **Current-tick breakdown stored inline with each tracked value; last 12 world snapshots kept in memory; older ticks recomputed by deterministic replay.** | 0007 |
| 3 | Belief representation | **Gaussian mixture, 1–3 components, on a declared scale (linear / log / logit), with a (mean, σ) facade.** | 0008 |
| 4 | Phase ordering | **Fixed ordered stages. Within a stage, subsystems read the stage-start state and return new blocks; blocks are committed at stage end. One writer per block.** | 0005 |
| 5 | Save format | **postcard for saves and hashing, blake3 for the state hash, JSON for export, RON for content. Versioned header; mismatched versions are rejected.** | 0009 |
| — | RNG | **ChaCha8Rng, one fresh stream per (subsystem, entity, tick, purpose) keyed by blake3 `derive_key` from the world seed. Exact version pins plus golden values.** | 0006 |

### 2.1 Options and trade-offs (condensed; full text in the ADRs)

**D1 — ECS vs. structs.** The MVP world is tiny: ~45 polities, ~100 regions, ~50 bodies, a few hundred techs [D]. An ECS (`bevy_ecs` 0.20.1 is current [V: crates.io, 2026-11-22]) buys cache-friendly iteration over millions of entities and a parallel scheduler. Here that's unneeded, and the parallel scheduler is a determinism hazard. It also adds a large dependency whose API changes every minor release [R], and its components are harder to serialize as one value for hashing and saves. Plain structs are serde-derivable, hashable as one value, and borrow-checked for the read-old/write-new rule (D4). → Structs.

**D2 — Breakdown memory.** Estimate [D]: ~1,600 tracked scalars (6 high-fidelity polities × ~200 + 40 macro × ~10) × ~6 terms × ~40 B ≈ 0.4 MB/tick. Keeping every tick for 1,260 ticks is ≈ 0.5 GB, so that's out. Keeping the current tick only can't answer "why did this change last month?". Recomputing always means every query waits on a replay. → Inline current tick + a ring of the last 12 snapshots (≈ 12 × a few MB) + replay from the nearest autosave for older ticks. Determinism (SYS-002) makes the replay exact.

**D3 — Beliefs.** A single Gaussian (mean, σ) satisfies P-2 for intel, fog of war, treaties and prospecting. It can't represent "this tech either works well or is a dead end", which WS-05 needs (ConOps §6: mixtures for multimodal outcomes). Particle sets are general but heavy (~100× memory) and noisy. → A mixture capped at 3 components, stored inline (no heap). A Gaussian observation updates it in closed form, and its moment-matched (mean, σ) is what every consumer outside R&D sees. Positive and bounded quantities use log and logit scales so σ stays meaningful.

**D4 — Ordering.** In-place sequential updates are simplest, but every subsystem's view depends silently on call order. A full double buffer (everyone reads tick t−1) gives a one-month lag on every coupling, which breaks the tax-revenue-from-this-month's-output chain. → Staged commit: a few ordered stages, read-stage-start/commit-at-end inside each stage. Each coupling lag becomes explicit (it follows from stage placement), and the borrow checker enforces it.

**D5 — Save format.** RON is readable but large and slow, and saves don't need to be hand-edited. bincode is out: 3.0.0 is a tombstone release whose source is `compile_error!` [V: built 2026-11-22], and 2.0.1 is the end of the line. postcard 1.2.0 is maintained [V: crates.io], compact, serde-based, and encodes `f64` as its exact 8 little-endian bytes [V: measured `2.5_f64` → `00 00 00 00 00 00 04 40`]. That makes it a bit-exact hash input. It isn't self-describing, so any type change breaks old saves; a version header plus a fixture test catches that. → postcard.

## 3. Interfaces

Signatures below are normative in shape. Names may change in implementation if the spec is updated to match.

### 3.1 Workspace

```
Cargo.toml              workspace; members = crates/sim-core, crates/cli, app/src-tauri
                        default-members = crates/sim-core, crates/cli   (app needs webkit system libs)
rust-toolchain.toml     pinned stable release, edition 2024
clippy.toml             disallowed-types / disallowed-methods (ARC-004)
crates/sim-core/        headless library: state, pipeline, rng, explain, content, save, tier
crates/cli/             binary `orchestra`
app/                    Tauri 2 hello-world (src-tauri depends on sim-core)
content/core/           setting-agnostic content (techs, components, hulls, gov types)
content/maps/earth/     region graph
content/maps/sol/       Solar System body graph template (WS-08 generates the world variant)
content/settings/date/  polities, ownership, start state (ADR-0003)
content/scenarios/      scenario = map + setting + start date + overrides
```

Dependencies for `sim-core` (each justified, all pinned in `Cargo.lock`, RNG crates pinned with `=`):

| Crate | Version | Why it can't be avoided |
|---|---|---|
| `serde` (derive) | 1.0.x [V 1.0.229 latest] | One derive feeds content, saves, export, hashing |
| `ron` | 0.12.x [V 0.12.3] | Content format (ADR-0001) |
| `rand_core` / `rand_chacha` / `rand` / `rand_distr` | `=0.10.1` / `=0.10.0` / `=0.10.4` / `=0.6.0` [V: compiled together 2026-11-22] | ChaCha8Rng and standard distributions; `rand_distr` requires `rand` |
| `postcard` (`use-std`) | 1.2.x [V 1.2.0] | Saves and the hash byte stream (ADR-0009) |
| `blake3` | 1.8.x [V 1.8.7] | State hash, content hash, RNG key derivation — one crate for three jobs |

`cli` adds `clap` (derive) 4.6.x [V 4.6.9] and `serde_json` 1.0.x [V 1.0.154]. Graph search (delta-v shortest path) uses `std::collections::BinaryHeap`; no `petgraph` [D]. Errors are a hand-written enum; no `thiserror`/`anyhow` in sim-core [D].

### 3.2 World state model

```rust
// Typed dense indices. Never reused within a world; dead entities keep their slot.
pub struct PolityId(pub u16);  pub struct RegionId(pub u16);  pub struct BodyId(pub u16);
pub struct TechIdx(pub u32);   // content entities get dense indices at load (sorted by string ID)

pub struct Tick(pub u32);      // months since scenario start; Date derived from it. No wall clock.

pub struct World {
    pub header: WorldHeader,              // seed, scenario id, content hash, tick
    truth: Truth,                         // private field; read via &Truth only (ARC-011)
    pub map: Graph<Region, RegionEdge>,   // static after load
    pub sol: Graph<Body, Transfer>,       // static after worldgen
    pub polities: Vec<Polity>,
    // one block vector per subsystem, indexed by PolityId; added by each WS:
    pub econ: Vec<EconBlock>,             // WS-03 (stub in M4)
    pub gov:  Vec<GovBlock>,              // WS-04 (stub in M4)
    pub rnd:  Vec<RndBlock>,              // WS-05 (stub in M4)
    pub force: Vec<ForceBlock>,           // WS-11 (stub in M4): units, groupings, manpower, munitions, bases
    pub engagements: Vec<EngagementRecord>, // WS-07, this tick only: losses and munitions expended per unit
    pub beliefs: Vec<BeliefBlock>,        // per polity; owned by the Observe stage
    pub events: Vec<Event>,               // this tick only
}

pub struct Polity {
    pub id: PolityId, pub content_id: String, pub alive: bool,
    pub kind: PolityKind,                 // NationState only in MVP; enum is open (ConOps §3)
    pub tier: Tier,                       // High | Macro
    pub agg: Aggregates,                  // always present, every tier (§3.8)
}

pub struct Graph<N, E> { pub nodes: Vec<N>, pub edges: Vec<(u16, u16, E)>, adj: Vec<Vec<u32>> }
// edges sorted by (from, to) at load; adjacency derived, not serialized.
```

Rules: every collection in `World` is a `Vec` or `BTreeMap`. Subsystem blocks hold the high-fidelity detail as `Option<Box<Detail>>` (`None` for macro polities). Caches that can be rebuilt are `#[serde(skip)]` and rebuilt after load.

**Truth vs. belief (P-1, P-2).** `Truth` holds every latent value sampled at world generation (tech parameters, deposits, hidden enabler edges). It is constructed only by `worldgen` and exposed only as `&Truth`. Beliefs live per polity in `BeliefBlock { by_subject: Vec<Option<Belief>> }`, indexed by a dense `SubjectIdx` (`None` = the polity doesn't know the subject exists).

```rust
pub enum Scale { Linear, Log, Logit }           // positive quantities use Log, probabilities Logit
pub struct Component { pub w: f64, pub mu: f64, pub sigma: f64 }   // on `scale`
pub struct Belief { pub scale: Scale, pub n: u8, pub comps: [Component; 3] }  // Copy, no heap
impl Belief {
    pub fn normal(scale: Scale, mu: f64, sigma: f64) -> Self;
    pub fn mixture(scale: Scale, comps: &[Component]) -> Result<Self, BeliefError>; // 1..=3, Σw = 1
    pub fn mean(&self) -> f64;      // moment-matched, in natural units
    pub fn sigma(&self) -> f64;     // moment-matched, in natural units
    pub fn observe(&mut self, value: f64, noise_sigma: f64); // exact Bayesian update per component
                                                             // + likelihood reweighting
}
pub struct Observation { pub polity: PolityId, pub subject: SubjectIdx, pub value: f64,
                         pub noise_sigma: f64, pub source: Cow<'static, str> }
```

Subsystems never call `observe` directly: they push `Observation`s, and the Observe stage (I1) applies them all in push order. That keeps one belief mechanic (P-2) and one writer for `beliefs`.

### 3.3 Tick pipeline

Stages run in this fixed order every tick [D]:

| Stage | Purpose | Owners (writers) | Reads |
|---|---|---|---|
| D1 Orders | Take player orders for this tick from the input log/UI; validate | `orders` queue | all |
| D2 AiDecide | AI polities emit orders (WS-06) | `orders` queue | `PolityView` only (ARC-012) |
| R1 ApplyOrders | Route each validated order to the owning subsystem's `apply_order`; rejected orders become events | each block, for its own orders | all |
| R2 Exogenous | Shocks, regime switches, scripted events | WS-03 shocks, WS-08 events | stage-start |
| R3 Production | Output, industry learning, R&D project progress | `econ` (production part), `rnd` | stage-start |
| R4 Finance | Tax revenue, budget, bonds, monetary, demographics | `econ` (finance part) | R3 results |
| R5 Politics | Factions, stability, reform | `gov` | R4 results |
| R5b Forces | Readiness and upkeep, mobilization and manpower draw, unit deliveries and lifecycle, deployments (WS-11) | `force` | R5 results |
| R6 External | Diplomacy, treaties, engagements, space ops (WS-06/07/08). Combat reads `force` and writes `engagements`; it never writes `force`. | their blocks, `engagements` | R5b results |
| R6b Losses | Apply `engagements` to `force`: casualties, destroyed units, munitions drawdown, captured bases (WS-11) | `force` | R6 results |
| R7 Tier | Refresh `Polity.agg` from detail; execute tier switches | `polities[].agg`, `tier`, detail `Option`s | R6b results |
| I1 Observe | Apply all `Observation`s to `beliefs` | `beliefs` | all |
| I2 Report | Finalize events, record time series | `events`, `Recorder` | all |

A subsystem's entry point is a plain function. The pipeline calls these in a hand-written, static `stage_*` function (no runtime registry, no inventory/linker magic) [D]. "Registering" a subsystem means adding its block field to `World` and one line to the stage it belongs in.

```rust
pub struct TickCtx<'a> {
    pub tick: Tick,
    rng: RngFactory,                      // §3.4; subsystems call ctx.rng(key)
    pub events: &'a mut Vec<Event>,
    pub observations: &'a mut Vec<Observation>,
}
impl TickCtx<'_> { pub fn rng(&mut self, key: StreamKey) -> ChaCha8Rng; }

// Typical subsystem function: reads the stage-start world, returns its new block(s).
pub fn production(world: &World, ctx: &mut TickCtx) -> Vec<EconBlock>;

// Stage body: every function sees the same &World; commits happen after all have run.
fn stage_r3(world: &mut World, ctx: &mut TickCtx) {
    let econ = econ::production(world, ctx);
    let rnd  = rnd::progress(world, ctx);
    world.econ = econ;
    world.rnd = rnd;
}

pub fn step(world: &mut World, orders: &[Order]) -> TickReport; // runs D1..I2, increments tick
```

**Force state (ConOps OQ-6).** WS-11 owns `force` and writes it in two stages: R5b (its own dynamics) and R6b (applying combat results). WS-07 only ever writes `engagements`. Losses therefore reach `force` and `agg.military_power` in the same tick, and the one-writer-per-block-per-stage rule holds. Known lag: manpower drawn in R5b reduces the labor force seen by WS-03 the following tick.

Within a stage, subsystems run sequentially in the listed order. Sinks (`events`, `observations`) are appended in that order, so they're deterministic. A subsystem that needs a value computed later in the same stage must move to a later stage instead.

**Orders.** `Order { polity: PolityId, tick: Tick, kind: OrderKind }`. `OrderKind` is one enum in sim-core that each WS extends with its own variants by PR. Only player orders go in the input log. AI orders are recomputed deterministically on replay.

### 3.4 RNG

```rust
pub struct StreamKey<'a> { pub tag: &'a str, pub entity: u64, pub purpose: u32 }
impl RngFactory {
    /// One fresh ChaCha8Rng per (world seed, tag, entity, tick, purpose).
    pub fn stream(&mut self, key: StreamKey) -> ChaCha8Rng;
}
// key material = seed u64 LE ‖ tag.len() u32 LE ‖ tag bytes ‖ entity u64 LE ‖ tick u32 LE ‖ purpose u32 LE
// rng = ChaCha8Rng::from_seed(blake3::derive_key("orchestra rng v1", &material))
pub fn worldgen_stream(seed: u64, content_id: &str, purpose: u32) -> ChaCha8Rng; // tick-free, keyed by string ID
```

- Fresh keyed streams make every draw independent of how many draws any other subsystem, polity or tick made. Adding a polity, reordering subsystems inside a stage, or parallelizing later can't perturb other streams [D].
- World truth is keyed by the content string ID, so adding a tech to content doesn't change any other tech's truth [D].
- Cost: 288 ns per stream creation [V: measured, release build, x86-64 container, 2026-11-22]. About 1,000 streams per tick is ≈ 0.3 ms.
- The factory records issued keys and panics if one is requested twice in the same tick (that would silently correlate draws). This check runs in debug and test builds.
- Batch runs: run *i* uses seed `u64::from_le_bytes(derive_key("orchestra batch v1", base_seed ‖ i)[0..8])`.

### 3.5 Explainability

```rust
pub struct Term { pub label: Cow<'static, str>, pub value: f64, pub link: Option<VarKey> }
pub struct Breakdown { pub total: f64, pub terms: Vec<Term> }   // total = Σ terms (ARC-020)
impl Breakdown {
    pub fn sum(terms: Vec<Term>) -> Self;
    pub fn product(label_base: &str, base: f64, factors: &[(Cow<'static, str>, f64)]) -> Self;
        // multiplicative model → additive terms by log share:
        // term_i = (total − base) · ln f_i / Σ ln f_j, plus a `base` term; exact sum by construction.
        // Requires every f_i > 0 (else Err). If Σ ln f_j = 0 the factor terms are 0 and total = base.
    pub fn stock(prev: f64, flows: Vec<Term>) -> Self;   // first term "previous", then flows
}
pub struct Tracked { value: f64, why: Breakdown }      // value is private
impl Tracked { pub fn set(&mut self, b: Breakdown); pub fn get(&self) -> f64; pub fn why(&self) -> &Breakdown; }

pub struct VarKey { pub owner: Owner, pub block: Cow<'static, str>, pub field: Cow<'static, str> }
pub enum Owner { World, Polity(PolityId), Region(RegionId), Body(BodyId) }

pub trait Explain {                         // implemented by hand for each block type
    fn tracked(&self) -> Vec<(&'static str, &Tracked)>;
}
// Query API (sim-core):
pub fn list_vars(world: &World) -> Vec<VarKey>;
pub fn explain(world: &World, key: &VarKey) -> Option<&Breakdown>;            // current tick
pub fn explain_at(hist: &History, key: &VarKey, tick: Tick) -> Result<Breakdown, ExplainError>;
```

- A displayed variable must be a `Tracked`, and the only way to change a `Tracked` is `set(Breakdown)`. So "every update produces a breakdown" is enforced by the type, not by review.
- Internal arrays (cohort vectors, income-quantile bins) may be plain `f64`s. Any aggregate of them that is displayed must be `Tracked`.
- `link` lets the UI drill down into another variable's breakdown without copying it.
- `History` holds the last 12 committed `World` snapshots (`Arc<World>`) plus autosaves (every 12 ticks by default). `explain_at` for an older tick loads the nearest earlier autosave and replays up to the target tick.

### 3.6 Content loading

- Each content kind is a Rust struct with `#[derive(Deserialize)] #[serde(deny_unknown_fields)]`; the Rust types are the schema (ADR-0001).
- Load order: scenario → map → setting → core content. Every entity has a string ID (`"tech.fusion_torch"`, `"polity.donovia"`); dense indices are assigned after sorting by string ID.
- Validation collects **all** errors before failing: unresolved references, duplicate IDs, graph edges to missing nodes, region ownership referencing unknown polities, parameter ranges (σ > 0, probabilities ∈ [0, 1], mixture weights sum to 1), and setting files that reference map regions by ID only. Each error names its file and ID; RON's line/column is included for parse errors.
- Setting/map separation: setting files may reference map region IDs but never define geography; map files never mention polities.
- Content hash = blake3 over the sorted `(relative path, file bytes)` pairs of every loaded file. It goes in the world header and the save header.

### 3.7 Save, load, replay, hash

```
Save file = postcard( SaveHeader { magic: *b"ORCH", format: u32, sim_core: String,
                                   content_hash: [u8; 32], seed: u64, tick: Tick },
                      World,
                      InputLog { orders: Vec<Order>, checkpoints: Vec<(Tick, [u8; 32])> } )
state_hash(world) = blake3(postcard(world))
```

- `SAVE_FORMAT_VERSION` must be bumped whenever any serialized type changes. A committed fixture (`tests/fixtures/save_v{N}.bin`) of a tiny fixed world catches unbumped changes.
- Load rejects a format or content-hash mismatch with a clear error. No migration in the MVP.
- Replay = initial save (or seed + scenario) + input log. The log optionally carries state-hash checkpoints every *k* ticks, so replay reports the first diverging tick.
- Every `f64` in the state must be finite after each stage (checked in debug/test builds). That catches runaway feedback early (ConOps R-2) and keeps NaN payloads out of the hash.

### 3.8 Fidelity tiers

```rust
pub enum Tier { High, Macro }
pub struct Aggregates { pub gdp: Tracked, pub population: Tracked, pub military_power: Tracked,
                        /* WS-03..07 may add fields by spec revision */ }
pub trait Tiered {                          // implemented by each subsystem block with detail
    fn aggregate(detail: &Self::Detail) -> Self::Macro;                       // High → Macro
    fn disaggregate(m: &Self::Macro, template: &Self::Template) -> Self::Detail; // Macro → High
    fn conserved(m: &Self::Macro) -> Vec<(&'static str, f64)>;
}
```

- Every polity always has `agg`. For high-fidelity polities R7 recomputes it from detail each tick, so diplomacy, AI and combat can compare polities of any tier through one interface.
- Tier switches are requested by order (`OrderKind::SetTier`, normally emitted by WS-06 AI) and executed only in R7.
- Demotion drops detail; `agg` already holds the aggregates. Promotion calls `disaggregate` with a content-defined template (e.g., age pyramid shape, quantile shares), rescales to hit the conserved values, then re-aggregates and checks them.
- `military_power` is the rollup of tracked-unit aggregates from `force` (WS-11, ConOps P-10/SYS-009); WS-07 defines how combat uses it. Until then the stub is the sum of stub unit strengths.

### 3.9 CLI (`orchestra`)

| Command | Behavior |
|---|---|
| `validate --content <dir> --scenario <id>` | Load and validate; print every error; exit non-zero on any |
| `run --scenario <id> --seed <u64> --ticks <n> [--inputs <log>] [--save <file>] [--hash-every <k>]` | Run headless; print the final state hash |
| `replay --save <file>` | Replay from the save's start + input log; report match or the first diverging checkpoint |
| `batch --scenario <id> --runs <N> --seed-base <u64> --ticks <n> --metrics <list> [--at <ticks>] [--threads <t>] --out <file.json>` | N independent runs; per metric and per sampled tick: mean, sd, min, p5, p50, p95, max, count of runs that panicked or produced non-finite values |
| `export --save <file> [--tick <t>] --format json --out <file>` | Full world (or listed vars + breakdowns) as JSON |
| `explain --save <file> --var <key> [--tick <t>]` | Print a breakdown |
| `bench --scenario <id> --ticks <n>` | Median and p95 ms per tick (timing lives in the CLI, never in sim-core) |

Batch uses `std::thread::scope`; each run is single-threaded; results are gathered by run index, so the summary is identical for any `--threads` [D].

### 3.10 Tauri stub

`app/src-tauri` exposes two commands: `sim_version() -> String` and `new_world_hash(seed: u64) -> String`. The front end is one HTML page that calls both and prints the results. Nothing else.

## 4. Requirements

| ID | Requirement | Acceptance criterion (testable) | Trace |
|---|---|---|---|
| **Workspace** |||
| ARC-001 | The repo is a Cargo workspace with `crates/sim-core`, `crates/cli`, and `app/src-tauri` (package `orchestra-app`, not a default member). | `cargo build` and `cargo test` (default members) pass on the pinned stable toolchain. | §7 |
| ARC-002 | `sim-core` has no UI, async-runtime, or I/O-framework dependency. | CI step: `cargo tree -p sim-core -e normal` contains none of `tauri`, `wry`, `tokio`, `serde_json`, `clap`. | P-5 |
| ARC-003 | Third-party dependencies are limited to the table in §3.1; RNG crates pinned with `=`; `Cargo.lock` committed. | CI step compares `cargo tree -p sim-core --depth 1 -e normal` against an allowlist file and fails on any addition. | ADR-0004, ADR-0006 |
| ARC-004 | Banned in `sim-core`: `HashMap`/`HashSet`, `SystemTime`, `Instant`, thread-local/OS RNG, `unsafe`. | `clippy.toml` `disallowed-types`/`disallowed-methods` lists them; `#![forbid(unsafe_code)]`; `cargo clippy -p sim-core -- -D warnings` passes. | P-4, SYS-002 |
| **State model** |||
| ARC-010 | Polities are a single generic type with `kind` and `tier` fields; no polity, tech, region or name is hard-coded. | Test: loading the test setting creates polities only from data; a CI `grep` for every setting/content ID in `crates/*/src/` returns no matches. | P-6, P-7 |
| ARC-011 | World truth is immutable after world generation. | Test: `blake3(postcard(world.truth()))` is identical at tick 0 and after a 120-tick run with stub subsystems. A `compile_fail` doc test shows no `&mut Truth` is reachable from `World`. | P-1, SYS-005 |
| ARC-012 | Decide-stage code sees a polity's own state, its beliefs and public information only. | `PolityView` has no accessor returning `&Truth` or another polity's detail block; `compile_fail` doc test. | P-1, P-2 |
| ARC-013 | `Belief` supports 1–3 Gaussian components on a linear, log or logit scale, with moment-matched mean and σ. | Unit tests: (a) a single-component update matches the Kalman closed form within 1e-12; (b) a two-component belief observed near one mode shifts > 90% weight to that mode; (c) mean/σ of a mixture equal analytic moments within 1e-12; (d) 100 unbiased observations of a truth bring σ below 10% of its prior and the mean within 3σ of truth. | P-2, SYS-005 |
| ARC-014 | All belief updates go through `Observation`s applied in stage I1. | Test: a stub subsystem's observations change `beliefs` only after I1; no other code path writes `world.beliefs` (pub(crate) setter; reviewed by grader). | P-2 |
| ARC-015 | Region and body graphs support neighbor queries and minimum-cost paths. | Test: on a 5-node fixture graph, `shortest_path` returns the known optimum delta-v path and cost; adjacency lists are sorted. | ConOps §3 |
| **Pipeline** |||
| ARC-020 | Each tick runs stages D1, D2, R1–R5, R5b, R6, R6b, R7, I1, I2 in that order. | Test: stub subsystems append their stage name to an event list; the per-tick sequence equals the table in §3.3. | ConOps §2 |
| ARC-021 | Within a stage, every subsystem reads the stage-start state; blocks commit at stage end; each block has exactly one writer. | Test: two stub subsystems in one stage each read the other's block; results are identical when their call order is swapped. Ownership holds because subsystem functions take `&World` (compile-enforced). | P-4 |
| ARC-022 | Orders are validated before application; invalid orders are rejected as events, never panics. | Test: an order for a dead polity, a past tick, and an out-of-range value each produce a rejection event and leave state unchanged. | SYS-001 |
| ARC-023 | All state `f64`s are finite after every stage in debug and test builds. | Test: a stub subsystem that writes NaN causes a panic naming the stage and block. | R-2, SYS-002 |
| ARC-024 | Combat losses reach `force` and aggregates in the same tick, and only WS-11 functions write `force`. | Test: a stub combat function in R6 records the destruction of one stub unit in `engagements`; after R7 of the same tick the unit is gone from `force` and `agg.military_power` has dropped by its strength, with a breakdown term naming the engagement. Grader check: no WS-07 function returns a `ForceBlock`. | SYS-009, P-4 |
| **RNG** |||
| ARC-030 | All randomness comes from `RngFactory::stream` or `worldgen_stream`, as specified in §3.4. | Grader check plus ARC-004 lint; no other `SeedableRng` constructor in `sim-core` outside `rng.rs` (CI `grep`). | P-4, SYS-002 |
| ARC-031 | RNG golden values are pinned. | Test `arc_031_rng_golden`: `ChaCha8Rng::seed_from_u64(42)` first three `u64`s = `12578764544318200737, 17529487244874322312, 7886285670807131020`; `LogNormal::new(0.0, 0.5)` first sample from seed 42 = `1.2699666231939588`; `stream(seed 42, tag "eco", entity 3, tick 0, purpose 0)` first `u64` = `16460933805651989612`, first `f64` after it = `0.9772430632731485`. [V: measured 2026-11-22, x86-64 Linux, the pinned versions] | SYS-002 |
| ARC-032 | Streams are independent of other subsystems' draw counts. | Test: adding a stub subsystem that draws 1,000 values in R2 leaves every other subsystem's outputs bit-identical. | P-4 |
| ARC-033 | A stream key is never issued twice within a tick. | Test: requesting the same key twice in one tick panics in a debug build. | P-4 |
| ARC-034 | World truth for an entity depends only on (seed, content ID). | Test: adding an unrelated tech to the test content leaves every existing tech's sampled truth bit-identical. | P-1, SYS-005 |
| **Explainability** |||
| ARC-040 | Every displayed variable is a `Tracked`, settable only with a `Breakdown`. | `Tracked.value` is private; `compile_fail` doc test for direct assignment. | P-3, SYS-004 |
| ARC-041 | Breakdown terms sum to the value. | Property test over every `VarKey` from `list_vars` after a 120-tick stub run: `|Σ terms − total| ≤ 1e-9 · max(1, |total|)`. Unit tests for `sum`, `product`, `stock` with edge cases (factor = 1, base = 0, a single factor). | SYS-004 |
| ARC-042 | `list_vars` and `explain` return a breakdown for every listed variable of the current tick. | Test: every key from `list_vars` returns `Some`. | SYS-004 |
| ARC-043 | `explain_at` returns breakdowns for past ticks. | Test: after 40 ticks, `explain_at(tick 35)` (ring) and `explain_at(tick 5)` (replay from autosave) each equal the breakdown captured live at that tick, bit-for-bit. | SYS-004, SYS-002 |
| **Content** |||
| ARC-050 | Content deserializes into Rust types with unknown fields rejected. | Test: a fixture with a misspelled field fails, naming the file and field. | ADR-0001, P-6 |
| ARC-051 | Validation reports all errors in one pass. | Test: a fixture with 3 independent errors (dangling ref, duplicate ID, σ ≤ 0) reports exactly those 3. | P-6 |
| ARC-052 | Setting and map are separate; a polity can be added with data only. | Test (SYS-007): copying the test setting and adding one polity file plus an ownership entry yields a world with N+1 polities and no code change. A map file mentioning a polity ID fails validation. | SYS-007, ADR-0003 |
| ARC-053 | Content indices are deterministic. | Test: shuffling file order and entries within files yields the same content hash and the same dense indices. | P-4 |
| **Save / replay** |||
| ARC-060 | Two runs with the same seed, scenario and input log produce identical state hashes at every tick. | Test: 2 × 120-tick runs with stub subsystems and a scripted input log; hashes equal every tick. CI also runs `orchestra run` twice and compares output. | SYS-002 |
| ARC-061 | Save → load → continue equals an uninterrupted run. | Test: run 60, save, load, run 60 more; final hash equals a straight 120-tick run. | SYS-002 |
| ARC-062 | Saves carry a versioned header and are rejected on mismatch. | Tests: wrong magic, wrong format version and wrong content hash each give a specific error. The fixture test fails if serialized types change without a version bump. | SYS-002 |
| ARC-063 | Replay detects divergence. | Test: a log with a deliberately corrupted checkpoint reports that checkpoint's tick. | SYS-002 |
| **CLI** |||
| ARC-070 | `orchestra run` and `validate` work as in §3.9. | Integration test via `std::process::Command` on the test scenario. | SYS-003 |
| ARC-071 | `orchestra batch` emits the summary statistics of §3.9 as JSON, and the result doesn't depend on thread count. | Test: `--runs 8` with `--threads 1` and `--threads 4` produce byte-identical JSON; mean/sd for a stub metric with known distribution match analytic values within 3 standard errors. | SYS-003 |
| ARC-072 | `export` writes valid JSON of the world or of selected variables with breakdowns. | Test: the output parses with `serde_json` and round-trips the selected values exactly. | SYS-003, P-5 |
| **Tiers** |||
| ARC-080 | Every polity exposes `Aggregates` regardless of tier. | Test: after each tick, every alive polity has finite `gdp`, `population` and `military_power` with breakdowns. | P-7, SYS-006 |
| ARC-081 | A tier switch conserves aggregates. | Test: promote then demote a stub polity; each conserved quantity is within 1e-6 relative of its pre-switch value at both steps. | SYS-006 |
| ARC-082 | Tier switches happen only in R7 and only by order. | Test: a `SetTier` order issued in D1 changes `tier` only after R7 of the same tick; no other stage changes `tier`. | SYS-006 |
| **Performance** |||
| ARC-090 | Pipeline overhead (stubs that touch every tracked variable) is ≤ 5 ms/tick at MVP scale. | `orchestra bench` on a synthetic scenario (1 player + 5 high + 40 macro polities, 100 regions, 50 bodies, 300 techs, ~300 stub tracked units per high-fidelity polity, ~1,600 polity-level tracked vars plus unit state per OQ-A6), release build: median ≤ 5 ms on reference hardware; CI threshold 10 ms. | SYS-008 |
| ARC-091 | Per-subsystem budgets sum to the SYS-008 total. | Budget table (§5) is in this spec; each WS spec states its own budget; `bench` prints per-stage medians. | SYS-008 |
| **Tauri stub** |||
| ARC-100 | The Tauri app builds and calls sim-core. | Manual check on the dev machine: the window shows the version and a world hash that matches `orchestra run --ticks 0 --seed <same>`. | ADR-0002 |

## 5. Performance budget (SYS-008) [D]

Target: median ≤ 100 ms per tick, release build, on reference hardware (OQ-A3), with 1 player + 5 high-fidelity AI + 40 macro polities.

| Stage / owner | Budget (ms) |
|---|---|
| Pipeline overhead, RNG, commits, finiteness checks (WS-02) | 5 |
| R2–R4 Economy, finance, demographics (WS-03) | 20 |
| R5 Politics (WS-04) | 10 |
| R3 R&D, design, industry (WS-05) | 15 |
| D2 AI + R6 diplomacy (WS-06) | 20 |
| R6 combat (WS-07) | 10 |
| R6 space ops, R2 events (WS-08) | 5 |
| R5b forces, R6b losses (WS-11) | 5 |
| I1 Observe, I2 Report, R7 Tier | 5 |
| Margin | 5 |

Snapshot ring memory budget: ≤ 64 MB for 12 snapshots (≈ 5 MB per `World`) [D]. If WS-03 to WS-05 push past it, reduce the ring length before changing representation.

## 6. Content/data

The schemas themselves belong to each WS. WS-02 fixes only these:

- `content/scenarios/<id>.ron`: `Scenario { id, map: String, setting: String, start: (year, month), end: (year, month), overrides: [...] }`
- `content/maps/earth/regions.ron`: `[Region { id, name, lat, lon, terrain, ... }]`, `edges.ron`: `[(from_id, to_id, RegionEdge { kind, cost })]`
- `content/settings/<setting>/polities/*.ron` (one per polity), `ownership.ron`: `[(region_id, polity_id)]`, `start.ron`: start values keyed by polity ID
- `content/maps/sol/bodies.ron`, `transfers.ron`: template nodes and delta-v edges; WS-08 defines the seeded variant
- `crates/sim-core/tests/content/`: a minimal test setting (3 polities, 6 regions, 4 bodies, 5 techs) used by every ARC test

## 7. Verification

- Unit and integration tests named `arc_NNN_*`, one or more per requirement.
- Stub subsystems in `sim-core/src/stubs.rs` (test feature only): each writes one `Tracked` per polity per stage with a random-walk breakdown. They exercise the pipeline, RNG, breakdowns, tiers and hashing before any real model exists.
- CI: `fmt --check`, `clippy -D warnings`, `test`, the ARC-002/003 dependency checks, the ARC-060 double-run hash comparison, and `bench` against the ARC-090 CI threshold.
- Determinism scope: bit-identical results are guaranteed for the same target triple and the same build. `f64` transcendental functions (`ln`, `exp`, `powf`) come from the platform math library and may differ across platforms [R], and `rand_distr` uses them. Cross-platform determinism is deferred (OQ-A1).

## 8. Open questions

| ID | Question | Default if unanswered |
|---|---|---|
| OQ-A1 | Is cross-platform determinism needed (e.g., shared saves between Windows and Linux, or MP lockstep)? If yes: route all transcendental functions through the `libm` crate and replace `rand_distr` samplers with in-house ones. | No. Same-platform only in MVP. |
| OQ-A2 | Is the 12-tick ring and 12-tick autosave interval right for "why did X change?" UX (WS-09)? | 12 / 12 |
| OQ-A3 | Reference hardware for SYS-008. | 4-core x86-64 at ~3 GHz, 16 GB RAM [D]; Sean to name the actual machine |
| OQ-A4 | Should `Aggregates` include government debt and treasury from the start, so WS-06 AI can compare fiscal strength across tiers? | Add when WS-03 is drafted |
| OQ-A5 | ConOps §7 says the architecture is to be confirmed in an ADR during WS-02. ADR-0004 does that; ConOps §7 should cite it and drop "provisional". | **Closed:** ConOps §7 edited in this PR, effective when the PR is approved |
| OQ-A6 | Unit-level state (~1,800 units × readiness/strength/experience) would multiply the D2 inline-breakdown estimate several-fold and push the snapshot ring past 64 MB. Should unit-level variables store plain values and build breakdowns on query (stock form: the previous snapshot's value + this tick's flows), keeping inline breakdowns for polity-level aggregates only? | Yes [D]. Update D2/ADR-0007 estimates, and ARC-090 to measure ring memory. |
