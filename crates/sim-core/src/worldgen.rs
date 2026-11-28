//! World generation from loaded content (M4b subset). Every entity gets a dense index after
//! sorting by string ID (§3.6), so file order and entry order never matter.
use crate::belief::{Belief, BeliefBlock, SubjectRange};
use crate::content::{Content, ContentError};
use crate::graph::Graph;
use crate::ids::{BodyId, PolityId, Tick};
use crate::state::{
    Aggregates, Body, OrbitRegime, Polity, Region, RegionEdge, Sol, SolNode, Tier, Transfer, Truth,
    World, WorldHeader,
};
use crate::subsystems::{econ, resources, rnd};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub fn worldgen(content: &Content, seed: u64) -> Result<World, ContentError> {
    let mut polities: Vec<_> = content.polities.iter().collect();
    polities.sort_by(|a, b| a.id.cmp(&b.id));
    let polities: Vec<Polity> = polities
        .iter()
        .enumerate()
        .map(|(i, p)| Polity {
            id: PolityId(i as u16),
            content_id: p.id.clone(),
            alive: true,
            kind: p.kind,
            tier: if content.setting.default_high.contains(&p.id) {
                Tier::High
            } else {
                Tier::Macro
            },
            agg: Aggregates::default(),
        })
        .collect();

    let ranges = subject_ranges(content);
    let beliefs = polities
        .iter()
        .map(|p| initial_beliefs(&ranges, p.tier))
        .collect();

    let header = WorldHeader {
        seed,
        scenario: content.scenario.id.clone(),
        content_hash: [0; 32],
        tick: Tick(0),
        subject_ranges: ranges,
    };
    let truth = Truth {
        tech: vec![[1.0; 3]; content.tech_ids.len()],
    };
    Ok(World::new(
        header,
        truth,
        region_graph(content)?,
        sol(content)?,
        polities,
        beliefs,
    ))
}

/// Fixed order: WS-03 structural, WS-05 tech, WS-08b deposits (§3.2). Contiguous from 0.
fn subject_ranges(content: &Content) -> Vec<SubjectRange> {
    let lens = [
        ("econ", econ::subject_count(content)),
        ("rnd", rnd::subject_count(content)),
        ("resources", resources::subject_count(content)),
    ];
    let mut start = 0;
    lens.into_iter()
        .map(|(owner, len)| {
            let r = SubjectRange {
                owner: Cow::Borrowed(owner),
                start,
                len,
            };
            start += len;
            r
        })
        .collect()
}

/// Every held subject starts unknown (`None`); a macro polity omits the WS-05 range.
/// Under the test stubs, every WS-03 subject starts with a N(0, 1) prior so ARC-014 can observe it.
fn initial_beliefs(ranges: &[SubjectRange], tier: Tier) -> BeliefBlock {
    let slices = ranges
        .iter()
        .map(|r| {
            let held = r.owner != "rnd" || rnd::holds_range(tier);
            let prior = if cfg!(any(test, feature = "stubs")) && r.owner == "econ" {
                Some(Belief::normal(crate::belief::Scale::Linear, 0.0, 1.0))
            } else {
                None
            };
            held.then(|| vec![prior; r.len as usize])
        })
        .collect();
    BeliefBlock::new(ranges, slices)
}

/// Map regions sorted by ID; each listed edge is added in both directions.
fn region_graph(content: &Content) -> Result<Graph<Region, RegionEdge>, ContentError> {
    let file = PathBuf::from(format!("maps/{}/edges.ron", content.scenario.map));
    let ids = sorted_ids(content.regions.iter().map(|r| r.id.as_str()));
    let nodes = ids
        .keys()
        .map(|id| Region {
            content_id: id.to_string(),
        })
        .collect();
    let mut edges = Vec::new();
    for (a, b, e) in &content.edges {
        let (a, b) = (lookup(&ids, a, &file)?, lookup(&ids, b, &file)?);
        let edge = RegionEdge {
            kind: e.kind,
            cost: e.cost,
        };
        edges.push((b, a, edge.clone()));
        edges.push((a, b, edge));
    }
    Ok(Graph::new(nodes, edges))
}

/// Bodies and nodes sorted by ID. Node ID = `node.<body stem>.<regime>[_<label>]` (WS-08b §3.2).
/// Edges: `transfers.ron` only, both directions unless `one_way`.
fn sol(content: &Content) -> Result<Sol, ContentError> {
    let file = PathBuf::from("maps/sol/transfers.ron");
    let body_ids = sorted_ids(content.bodies.iter().map(|b| b.id.as_str()));
    let mut nodes: Vec<SolNode> = content
        .bodies
        .iter()
        .flat_map(|b| {
            let stem = b.id.split_once('.').map_or(b.id.as_str(), |(_, s)| s);
            let body = BodyId(body_ids[b.id.as_str()]);
            b.nodes.iter().map(move |n| SolNode {
                content_id: match &n.label {
                    Some(l) => format!("node.{stem}.{}_{l}", regime_name(n.regime)),
                    None => format!("node.{stem}.{}", regime_name(n.regime)),
                },
                body,
                regime: n.regime,
            })
        })
        .collect();
    nodes.sort_by(|a, b| a.content_id.cmp(&b.content_id));
    let node_ids = sorted_ids(nodes.iter().map(|n| n.content_id.as_str()));
    let mut edges = Vec::new();
    for t in &content.transfers {
        let (a, b) = (
            lookup(&node_ids, &t.from, &file)?,
            lookup(&node_ids, &t.to, &file)?,
        );
        let transfer = Transfer {
            dv: t.dv,
            months: t.months,
        };
        if !t.one_way {
            edges.push((b, a, transfer.clone()));
        }
        edges.push((a, b, transfer));
    }
    let bodies = body_ids
        .keys()
        .map(|id| Body {
            content_id: id.to_string(),
        })
        .collect();
    Ok(Sol {
        bodies,
        graph: Graph::new(nodes, edges),
    })
}

fn regime_name(r: OrbitRegime) -> &'static str {
    match r {
        OrbitRegime::Surface => "surface",
        OrbitRegime::Low => "low",
        OrbitRegime::Medium => "medium",
        OrbitRegime::Synchronous => "synchronous",
        OrbitRegime::High => "high",
        OrbitRegime::Lagrange => "lagrange",
        OrbitRegime::Transit => "transit",
    }
}

/// String ID → dense index, in sorted order.
fn sorted_ids<'a>(ids: impl Iterator<Item = &'a str>) -> BTreeMap<&'a str, u16> {
    let mut map: BTreeMap<&str, u16> = ids.map(|id| (id, 0)).collect();
    for (i, v) in map.values_mut().enumerate() {
        *v = i as u16;
    }
    map
}

fn lookup(
    ids: &BTreeMap<&str, u16>,
    id: &str,
    file: &std::path::Path,
) -> Result<u16, ContentError> {
    ids.get(id)
        .copied()
        .ok_or_else(|| ContentError::UnknownRef {
            file: file.into(),
            id: id.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// ARC-016(b): a macro polity stores no slice for the WS-05 range; high polities do.
    #[test]
    fn arc_016b_macro_stores_no_tech_slice() {
        // The scenario ID is read from the content, not written here (ARC-010 grep).
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/content");
        let scenario = std::fs::read_dir(root.join("scenarios"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let id = scenario.file_stem().unwrap().to_str().unwrap();
        let world = worldgen(&Content::load(&root, id).unwrap(), 1).unwrap();
        let rnd = world
            .header
            .subject_ranges
            .iter()
            .position(|r| r.owner == "rnd")
            .unwrap();
        for (p, b) in world.polities.iter().zip(world.beliefs()) {
            assert_eq!(b.holds(rnd), p.tier == Tier::High, "{}", p.content_id);
            assert!(b.holds(0) && b.holds(2), "{}", p.content_id);
        }
        assert!(world.polities.iter().any(|p| p.tier == Tier::Macro));
    }
}
