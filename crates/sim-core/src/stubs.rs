//! Test-only stub subsystems (WS-02 §7; compiled under `cfg(test)` or feature `stubs`).
//! Each stage, the trace stub writes one `Tracked` per polity and logs the stage as an event.
//! Deterministic in M4b; M4c turns the step into a random walk via `RngFactory`.
//! The knobs drive the ARC-014/020–024 tests.
use crate::belief::Observation;
use crate::explain::{Breakdown, Term, Tracked};
use crate::ids::{PolityId, Tick, UnitId};
use crate::pipeline::{Stage, TickCtx};
use crate::state::{EngagementRecord, Event, EventKind, World};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StubBlock {
    /// Trace stub: one tracked variable per polity, +1 per stage.
    pub x: Vec<Tracked>,
    /// ARC-021 pair, both written in R3 from each other's stage-start value.
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    /// Set by `OrderKind::StubSet` in R1.
    pub set: Vec<f64>,
    /// ARC-014: the observed subject's belief mean as each stage started.
    pub seen: Vec<(Stage, Option<f64>)>,
    pub knobs: Knobs,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Knobs {
    /// The trace stub writes NaN in this stage (ARC-023).
    pub nan_stage: Option<Stage>,
    /// Run the ARC-021 pair in swapped call order.
    pub swap_pair: bool,
    /// Stub combat destroys this unit at this tick (ARC-024).
    pub kill: Option<(Tick, PolityId, UnitId)>,
    /// Pushed in R3 every tick; its subject's belief is logged in `seen` (ARC-014).
    pub observe: Option<Observation>,
}

impl StubBlock {
    pub fn new(n: usize) -> Self {
        Self {
            x: vec![Tracked::default(); n],
            a: vec![1.0; n],
            b: vec![1.0; n],
            set: vec![0.0; n],
            ..Self::default()
        }
    }
}

/// The trace stub's output for one stage, committed at stage end.
pub(crate) struct Traced {
    x: Vec<Tracked>,
    seen: Option<(Stage, Option<f64>)>,
}

impl Traced {
    pub(crate) fn commit(self, world: &mut World) {
        world.stub.x = self.x;
        world.stub.seen.extend(self.seen);
    }
}

pub(crate) fn trace(world: &World, ctx: &mut TickCtx, stage: Stage) -> Traced {
    ctx.events.push(Event {
        tick: ctx.tick,
        kind: EventKind::StageRan(stage),
    });
    let k = &world.stub.knobs;
    if let (Stage::R3Production, Some(o)) = (stage, &k.observe) {
        ctx.observations.push(o.clone());
    }
    let step = if k.nan_stage == Some(stage) {
        f64::NAN
    } else {
        1.0
    };
    let x = world
        .stub
        .x
        .iter()
        .map(|prev| {
            let mut next = prev.clone();
            next.set(Breakdown::stock(
                prev.get(),
                vec![Term {
                    label: Cow::Owned(format!("{stage:?}")),
                    value: step,
                    link: None,
                }],
            ));
            next
        })
        .collect();
    let seen = k.observe.as_ref().map(|o| {
        let belief = world.beliefs()[o.polity.index()].get(o.subject);
        (stage, belief.map(|b| b.mean()))
    });
    Traced { x, seen }
}

/// ARC-021: `a' = b + 1`, `b' = 2a`, each reading the other's stage-start value.
pub(crate) fn pair(world: &World, _ctx: &mut TickCtx) -> (Vec<f64>, Vec<f64>) {
    let a = |w: &World| w.stub.b.iter().map(|b| b + 1.0).collect::<Vec<_>>();
    let b = |w: &World| w.stub.a.iter().map(|a| 2.0 * a).collect::<Vec<_>>();
    if world.stub.knobs.swap_pair {
        let new_b = b(world);
        (a(world), new_b)
    } else {
        (a(world), b(world))
    }
}

pub(crate) fn combat(world: &World) -> Vec<EngagementRecord> {
    match world.stub.knobs.kill {
        Some((tick, polity, unit)) if tick == world.header.tick => vec![EngagementRecord {
            polity,
            unit,
            destroyed: true,
        }],
        _ => Vec::new(),
    }
}

pub(crate) fn validate(value: f64) -> Option<Cow<'static, str>> {
    (!(0.0..=1.0).contains(&value)).then_some(Cow::Borrowed("value out of range [0, 1]"))
}

pub(crate) fn apply(world: &mut World, polity: PolityId, value: f64) {
    world.stub.set[polity.index()] = value;
}
