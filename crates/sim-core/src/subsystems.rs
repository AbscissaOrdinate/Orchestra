//! Subsystem entry points. In M4 every real subsystem is a no-op stub that returns its block
//! unchanged; WS-03/04/05/06/07/08b/11 replace the bodies, not the signatures (§3.3).
//! Each subsystem that owns belief subjects declares its range length (§3.2, ARC-016).
use crate::content::Content;
use crate::explain::{Breakdown, Owner, Term, VarKey};
use crate::ids::UnitId;
use crate::pipeline::{PolityView, TickCtx};
use crate::state::{EngagementRecord, Event, EventKind, Order, Polity, Tier, World};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EconBlock {}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GovBlock {}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RndBlock {}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceState {}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpaceState {}

/// WS-11 stub: tracked units with a plain strength (per-unit `Lazy` state arrives in M4c).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ForceBlock {
    pub units: Vec<StubUnit>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StubUnit {
    pub id: UnitId,
    /// 0 < s ≤ 1 (§3.8).
    pub strength: f64,
}

/// WS-03 (stub). R3 production, R4 finance.
pub mod econ {
    use super::*;
    /// Structural-parameter subjects: 6 per polity [D] placeholder; WS-03 confirms in M5.
    pub fn subject_count(c: &Content) -> u32 {
        6 * c.polities.len() as u32
    }
    pub fn production(world: &World, _ctx: &mut TickCtx) -> Vec<EconBlock> {
        world.econ.clone()
    }
    pub fn finance(world: &World, _ctx: &mut TickCtx) -> Vec<EconBlock> {
        world.econ.clone()
    }
}

/// WS-04 (stub). R5 politics.
pub mod gov {
    use super::*;
    pub fn politics(world: &World, _ctx: &mut TickCtx) -> Vec<GovBlock> {
        world.gov.clone()
    }
}

/// WS-05 (stub). R3 project progress.
pub mod rnd {
    use super::*;
    /// Three subjects per tech: viability, perf, cost (`tech_base + 3·t + k`, §3.2).
    pub fn subject_count(c: &Content) -> u32 {
        3 * c.tech_ids.len() as u32
    }
    /// Macro polities hold no tech beliefs (v1.3, WS-05 P-02a).
    pub fn holds_range(tier: Tier) -> bool {
        tier == Tier::High
    }
    pub fn progress(world: &World, _ctx: &mut TickCtx) -> Vec<RndBlock> {
        world.rnd.clone()
    }
}

/// WS-06 (stub). D2: AI sees only its `PolityView` (ARC-012).
pub mod ai {
    use super::*;
    pub fn decide(view: PolityView) -> Vec<Order> {
        #[cfg(any(test, feature = "stubs"))]
        return crate::stubs::ai_orders(view);
        #[cfg(not(any(test, feature = "stubs")))]
        {
            let _ = view;
            Vec::new()
        }
    }
}

/// WS-07 (stub). R6: writes `engagements` only, never `force` or `space` (ARC-024).
pub mod combat {
    use super::*;
    pub fn engage(world: &World, _ctx: &mut TickCtx) -> Vec<EngagementRecord> {
        #[cfg(any(test, feature = "stubs"))]
        return crate::stubs::combat(world);
        #[cfg(not(any(test, feature = "stubs")))]
        {
            let _ = world;
            Vec::new()
        }
    }
}

/// WS-11 (stub). Sole writer of `force`: R5b dynamics, R6b losses (§3.3).
pub mod force {
    use super::*;
    pub fn forces(world: &World, _ctx: &mut TickCtx) -> Vec<ForceBlock> {
        world.force.clone()
    }
    /// Removes every unit an engagement destroyed, with an event citing the record.
    pub fn losses(world: &World, ctx: &mut TickCtx) -> Vec<ForceBlock> {
        let mut force = world.force.clone();
        for (i, e) in world
            .engagements
            .iter()
            .enumerate()
            .filter(|(_, e)| e.destroyed)
        {
            let Some(block) = force.get_mut(e.polity.index()) else {
                continue;
            };
            let before = block.units.len();
            block.units.retain(|u| u.id != e.unit);
            if block.units.len() < before {
                ctx.events.push(Event {
                    tick: ctx.tick,
                    kind: EventKind::UnitRemoved {
                        polity: e.polity,
                        unit: e.unit,
                        engagement: i as u32,
                    },
                });
            }
        }
        force
    }
}

/// WS-08b (stub). R2 `resources`; R5b and R6b `space`.
pub mod resources {
    use super::*;
    /// One size subject per deposit plus one cost-ratio subject per off-Earth deposit (WS-08b D7).
    pub fn subject_count(c: &Content) -> u32 {
        (c.earth_deposits + 2 * c.off_earth_deposits) as u32
    }
    pub fn exogenous(world: &World, _ctx: &mut TickCtx) -> ResourceState {
        world.resources.clone()
    }
}

pub mod space {
    use super::*;
    pub fn movement(world: &World, _ctx: &mut TickCtx) -> SpaceState {
        world.space.clone()
    }
    pub fn losses(world: &World, _ctx: &mut TickCtx) -> SpaceState {
        world.space.clone()
    }
}

/// R7: refresh `agg` of high-fidelity polities from detail (§3.8). Tier switches are M4f.
pub mod tier {
    use super::*;
    /// Stub `military_power` = Σ stub unit strengths, one term per unit.
    pub fn refresh(world: &World, _ctx: &mut TickCtx) -> Vec<Polity> {
        let mut polities = world.polities.clone();
        for (p, block) in polities.iter_mut().zip(&world.force) {
            if p.tier != Tier::High {
                continue;
            }
            let terms = block.units.iter().map(unit_term).collect();
            p.agg.military_power.set(Breakdown::sum(terms));
        }
        polities
    }

    fn unit_term(u: &StubUnit) -> Term {
        Term {
            label: Cow::Owned(format!("unit {}", u.id.0)),
            value: u.strength,
            link: Some(VarKey {
                owner: Owner::Unit(u.id),
                block: Cow::Borrowed("force"),
                field: Cow::Borrowed("strength"),
            }),
        }
    }
}
