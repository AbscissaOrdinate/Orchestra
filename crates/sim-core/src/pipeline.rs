//! Tick pipeline (WS-02 §3.3, ADR-0005): fixed ordered stages; within a stage every subsystem
//! reads the stage-start `&World` and returns new blocks, which are committed at stage end.
use crate::belief::{BeliefBlock, Observation};
use crate::ids::{PolityId, Tick};
use crate::state::{Event, EventKind, Order, OrderKind, Polity, World};
use crate::subsystems::{
    EconBlock, ForceBlock, GovBlock, RndBlock, ai, combat, econ, force, gov, resources, rnd, space,
    tier,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stage {
    D1Orders,
    D2AiDecide,
    R1ApplyOrders,
    R2Exogenous,
    R3Production,
    R4Finance,
    R5Politics,
    R5bForces,
    R6External,
    R6bLosses,
    R7Tier,
    I1Observe,
    I2Report,
}

/// Every tick runs these in this order (ARC-020).
pub const STAGES: [Stage; 13] = [
    Stage::D1Orders,
    Stage::D2AiDecide,
    Stage::R1ApplyOrders,
    Stage::R2Exogenous,
    Stage::R3Production,
    Stage::R4Finance,
    Stage::R5Politics,
    Stage::R5bForces,
    Stage::R6External,
    Stage::R6bLosses,
    Stage::R7Tier,
    Stage::I1Observe,
    Stage::I2Report,
];

/// Per-tick context passed to subsystems. Sinks are appended in call order. M4c adds the RNG
/// factory and explain capture (§3.3–3.5).
pub struct TickCtx<'a> {
    pub tick: Tick,
    pub events: &'a mut Vec<Event>,
    pub observations: &'a mut Vec<Observation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TickReport {
    /// The tick that just ran.
    pub tick: Tick,
}

/// What D2 decide code sees (ARC-012): its own polity, blocks and beliefs, plus public
/// information (every polity's identity, tier and aggregates; the map and Sol graph).
/// There is no accessor for `Truth` or another polity's detail blocks.
///
/// ```
/// fn own(v: &sim_core::PolityView) -> f64 { v.polity().agg.gdp.get() + v.polities().len() as f64 }
/// ```
/// ```compile_fail
/// fn peek(v: &sim_core::PolityView) { let _ = v.truth(); }
/// ```
/// ```compile_fail
/// fn peek(v: &sim_core::PolityView) { let _ = v.world.truth(); }
/// ```
/// ```compile_fail
/// fn peek(v: &sim_core::PolityView) { let _ = &v.world.econ[1]; }
/// ```
/// ```compile_fail
/// fn peek(v: &sim_core::PolityView) { let _ = v.econ_of(sim_core::PolityId(1)); }
/// ```
#[derive(Clone, Copy)]
pub struct PolityView<'a> {
    world: &'a World,
    id: PolityId,
}

impl<'a> PolityView<'a> {
    pub fn new(world: &'a World, id: PolityId) -> Self {
        Self { world, id }
    }
    pub fn tick(&self) -> Tick {
        self.world.header.tick
    }
    pub fn polity(&self) -> &'a Polity {
        &self.world.polities[self.id.index()]
    }
    /// Public: identity, tier and aggregates of every polity.
    pub fn polities(&self) -> &'a [Polity] {
        &self.world.polities
    }
    pub fn map(&self) -> &'a crate::graph::Graph<crate::state::Region, crate::state::RegionEdge> {
        &self.world.map
    }
    pub fn sol(&self) -> &'a crate::state::Sol {
        &self.world.sol
    }
    pub fn beliefs(&self) -> &'a BeliefBlock {
        &self.world.beliefs()[self.id.index()]
    }
    pub fn econ(&self) -> &'a EconBlock {
        &self.world.econ[self.id.index()]
    }
    pub fn gov(&self) -> &'a GovBlock {
        &self.world.gov[self.id.index()]
    }
    pub fn rnd(&self) -> &'a RndBlock {
        &self.world.rnd[self.id.index()]
    }
    pub fn force(&self) -> &'a ForceBlock {
        &self.world.force[self.id.index()]
    }
}

/// Runs D1..I2 on `world` with this tick's player `orders`, then increments the tick.
pub fn step(world: &mut World, orders: &[Order]) -> TickReport {
    let tick = world.header.tick;
    let mut events = Vec::new();
    let mut observations = Vec::new();
    let mut queue = Vec::new();
    let mut ctx = TickCtx {
        tick,
        events: &mut events,
        observations: &mut observations,
    };
    for stage in STAGES {
        #[cfg(any(test, feature = "stubs"))]
        let traced = crate::stubs::trace(world, &mut ctx, stage);
        match stage {
            Stage::D1Orders => queue = validate(world, &mut ctx, orders),
            Stage::D2AiDecide => {
                for p in world.polities.iter().filter(|p| p.alive) {
                    queue.extend(ai::decide(PolityView::new(world, p.id)));
                }
            }
            Stage::R1ApplyOrders => {
                for order in &queue {
                    apply_order(world, order);
                }
            }
            Stage::R2Exogenous => world.resources = resources::exogenous(world, &mut ctx),
            Stage::R3Production => stage_r3(world, &mut ctx),
            Stage::R4Finance => world.econ = econ::finance(world, &mut ctx),
            Stage::R5Politics => world.gov = gov::politics(world, &mut ctx),
            Stage::R5bForces => {
                let force = force::forces(world, &mut ctx);
                let space = space::movement(world, &mut ctx);
                world.force = force;
                world.space = space;
            }
            Stage::R6External => world.engagements = combat::engage(world, &mut ctx),
            Stage::R6bLosses => {
                let force = force::losses(world, &mut ctx);
                let space = space::losses(world, &mut ctx);
                world.force = force;
                world.space = space;
            }
            Stage::R7Tier => world.polities = tier::refresh(world, &mut ctx),
            Stage::I1Observe => world.apply_observations(ctx.observations),
            Stage::I2Report => world.events = std::mem::take(ctx.events),
        }
        #[cfg(any(test, feature = "stubs"))]
        traced.commit(world);
        #[cfg(debug_assertions)]
        crate::finite::check_world(world, stage);
    }
    world.header.tick = Tick(tick.0 + 1);
    TickReport { tick }
}

/// R3: every function sees the same `&World`; commits happen after all have run.
fn stage_r3(world: &mut World, ctx: &mut TickCtx) {
    let econ = econ::production(world, ctx);
    let rnd = rnd::progress(world, ctx);
    #[cfg(any(test, feature = "stubs"))]
    let (a, b) = crate::stubs::pair(world, ctx);
    world.econ = econ;
    world.rnd = rnd;
    #[cfg(any(test, feature = "stubs"))]
    {
        world.stub.a = a;
        world.stub.b = b;
    }
}

/// D1: an order is valid if its polity exists and is alive, it is for this tick, and its owning
/// subsystem accepts its values. Invalid orders become `OrderRejected` events, never panics.
// Without the stubs, `OrderKind` has no variants yet, so `Order` is uninhabited and this body is
// unreachable. Drop the allow when the first real variant lands (WS-03+, or `SetTier` in M4f).
#[cfg_attr(
    not(any(test, feature = "stubs")),
    allow(unreachable_code, unused_variables)
)]
fn validate(world: &World, ctx: &mut TickCtx, orders: &[Order]) -> Vec<Order> {
    let mut valid = Vec::new();
    for (index, order) in orders.iter().enumerate() {
        let reason = match world.polities.get(order.polity.index()) {
            None => Some(Cow::Borrowed("unknown polity")),
            Some(p) if !p.alive => Some(Cow::Borrowed("polity is dead")),
            Some(_) if order.tick != world.header.tick => {
                Some(Cow::Borrowed("order is not for this tick"))
            }
            Some(_) => validate_kind(&order.kind),
        };
        match reason {
            None => valid.push(order.clone()),
            Some(reason) => ctx.events.push(Event {
                tick: ctx.tick,
                kind: EventKind::OrderRejected {
                    polity: order.polity,
                    index: index as u32,
                    reason,
                },
            }),
        }
    }
    valid
}

/// Routes to the owning subsystem's value check. (No real `OrderKind` variants exist in M4b;
/// each WS adds its own with its check.)
#[cfg(any(test, feature = "stubs"))]
fn validate_kind(kind: &OrderKind) -> Option<Cow<'static, str>> {
    match *kind {
        OrderKind::StubSet { value } => crate::stubs::validate(value),
    }
}
#[cfg(not(any(test, feature = "stubs")))]
fn validate_kind(_: &OrderKind) -> Option<Cow<'static, str>> {
    None
}

/// R1: routes a validated order to the owning subsystem's `apply_order`.
#[cfg(any(test, feature = "stubs"))]
fn apply_order(world: &mut World, order: &Order) {
    match order.kind {
        OrderKind::StubSet { value } => crate::stubs::apply(world, order.polity, value),
    }
}
#[cfg(not(any(test, feature = "stubs")))]
fn apply_order(_: &mut World, _: &Order) {}
