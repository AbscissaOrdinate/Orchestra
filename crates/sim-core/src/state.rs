//! World state model (WS-02 §3.2, ADR-0004): plain structs of `Vec`s indexed by typed IDs.
use crate::belief::{BeliefBlock, Observation, SubjectRange};
use crate::explain::Tracked;
use crate::graph::Graph;
use crate::ids::{BodyId, PolityId, Tick, UnitId};
use crate::subsystems::{EconBlock, ForceBlock, GovBlock, ResourceState, RndBlock, SpaceState};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldHeader {
    pub seed: u64,
    pub scenario: String,
    /// blake3 over the loaded content files; filled in by M4d (ARC-053). Zero until then.
    pub content_hash: [u8; 32],
    pub tick: Tick,
    /// Subject ranges in fixed order: WS-03, WS-05, WS-08b (§3.2, ARC-016).
    pub subject_ranges: Vec<SubjectRange>,
}

/// Every latent value sampled at world generation (P-1). Built only by `worldgen`; reachable only
/// as `&Truth` (ARC-011).
///
/// ```
/// fn read(w: &sim_core::World) -> &sim_core::Truth { w.truth() }
/// ```
/// ```compile_fail
/// fn write(w: &mut sim_core::World) { let _t: &mut sim_core::Truth = &mut w.truth; }
/// ```
/// ```compile_fail
/// fn write(w: &mut sim_core::World) { let _t = w.truth_mut(); }
/// ```
/// ```compile_fail
/// fn write(w: &mut sim_core::World) { w.truth().tech.clear(); }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Truth {
    /// Per tech: viability, perf, cost. Placeholder 1.0 [D] until M4c/M5 sample them with
    /// `worldgen_stream` (ARC-034, WS-05).
    pub tech: Vec<[f64; 3]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct World {
    pub header: WorldHeader,
    truth: Truth,
    /// Static after load.
    pub map: Graph<Region, RegionEdge>,
    /// Static after worldgen.
    pub sol: Sol,
    pub polities: Vec<Polity>,
    pub econ: Vec<EconBlock>,
    pub gov: Vec<GovBlock>,
    pub rnd: Vec<RndBlock>,
    pub force: Vec<ForceBlock>,
    pub resources: ResourceState,
    pub space: SpaceState,
    /// This tick only; written by WS-07 in R6, read by WS-11 in R6b.
    pub engagements: Vec<EngagementRecord>,
    /// Per polity; written only in stage I1 (ARC-014).
    beliefs: Vec<BeliefBlock>,
    /// This tick only.
    pub events: Vec<Event>,
    /// Test-only stub subsystem state (§7).
    #[cfg(any(test, feature = "stubs"))]
    pub stub: crate::stubs::StubBlock,
}

impl World {
    pub(crate) fn new(
        header: WorldHeader,
        truth: Truth,
        map: Graph<Region, RegionEdge>,
        sol: Sol,
        polities: Vec<Polity>,
        beliefs: Vec<BeliefBlock>,
    ) -> Self {
        let n = polities.len();
        Self {
            header,
            truth,
            map,
            sol,
            econ: vec![EconBlock::default(); n],
            gov: vec![GovBlock::default(); n],
            rnd: vec![RndBlock::default(); n],
            force: vec![ForceBlock::default(); n],
            resources: ResourceState::default(),
            space: SpaceState::default(),
            engagements: Vec::new(),
            beliefs,
            events: Vec::new(),
            #[cfg(any(test, feature = "stubs"))]
            stub: crate::stubs::StubBlock::new(n),
            polities,
        }
    }

    pub fn truth(&self) -> &Truth {
        &self.truth
    }

    pub fn beliefs(&self) -> &[BeliefBlock] {
        &self.beliefs
    }

    /// The I1 writer: applies observations in push order (ARC-014).
    pub(crate) fn apply_observations(&mut self, observations: &[Observation]) {
        for o in observations {
            if let Some(b) = self.beliefs.get_mut(o.polity.index()) {
                b.apply(o);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolityKind {
    NationState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    High,
    Macro,
}

/// Present for every polity in every tier (§3.8).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Aggregates {
    pub gdp: Tracked,
    pub population: Tracked,
    pub military_power: Tracked,
    pub orbital_assets: Tracked,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Polity {
    pub id: PolityId,
    pub content_id: String,
    pub alive: bool,
    pub kind: PolityKind,
    pub tier: Tier,
    pub agg: Aggregates,
}

/// Map region. M4b keeps only the ID; WS-08a §3.2 fields arrive with M4d.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub content_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    Land,
    Strait,
    Coast,
    SeaLane,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionEdge {
    pub kind: EdgeKind,
    /// Great-circle km.
    pub cost: u32,
}

/// WS-08b §3.2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrbitRegime {
    Surface,
    Low,
    Medium,
    Synchronous,
    High,
    Lagrange,
    Transit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub content_id: String,
}

/// Sol graph node. M4b subset of WS-08b §3.2 (radii and `hub` arrive with M4d).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolNode {
    pub content_id: String,
    pub body: BodyId,
    pub regime: OrbitRegime,
}

/// Sol graph edge. M4b subset of WS-08b §3.2; derived edges (WS-08b §4.2) arrive later, so M4b
/// loads only `transfers.ron` edges.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    /// km/s.
    pub dv: f64,
    pub months: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sol {
    pub bodies: Vec<Body>,
    pub graph: Graph<SolNode, Transfer>,
}

/// One combat result (WS-07 writes these in R6; WS-11 applies them in R6b). Index = record ID.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EngagementRecord {
    pub polity: PolityId,
    pub unit: UnitId,
    pub destroyed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub polity: PolityId,
    pub tick: Tick,
    pub kind: OrderKind,
}

/// One enum in sim-core; each WS adds its variants by PR (§3.3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderKind {
    /// Test stub order: sets the stub value; valid range [0, 1].
    #[cfg(any(test, feature = "stubs"))]
    StubSet { value: f64 },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub tick: Tick,
    pub kind: EventKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventKind {
    /// A player order failed D1 validation. `index` is its position in this tick's order list
    /// (the input log has the order itself, so no raw values reach state).
    OrderRejected {
        polity: PolityId,
        index: u32,
        reason: Cow<'static, str>,
    },
    /// WS-11 removed a unit in R6b; `engagement` indexes this tick's `World.engagements`.
    UnitRemoved {
        polity: PolityId,
        unit: UnitId,
        engagement: u32,
    },
    /// Test stub trace: a stage ran (ARC-020).
    #[cfg(any(test, feature = "stubs"))]
    StageRan(crate::pipeline::Stage),
}
