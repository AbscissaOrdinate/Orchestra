//! Breakdown types (WS-02 §3.5). M4b builds the subset ARC-024 needs: `Term`, `VarKey`/`Owner`,
//! `Breakdown::{sum, stock}`, `Tracked` and the pure `Breakdown::change`. M4c adds `product`,
//! `Lazy`, `History` and the query API (ARC-040–046).
use crate::ids::{BodyId, DepositId, FacilityId, NodeId, PolityId, RegionId, UnitId};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Owner {
    World,
    Polity(PolityId),
    Region(RegionId),
    Body(BodyId),
    Unit(UnitId),
    Deposit(DepositId),
    Facility(FacilityId),
    Node(NodeId),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VarKey {
    pub owner: Owner,
    pub block: Cow<'static, str>,
    pub field: Cow<'static, str>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Term {
    pub label: Cow<'static, str>,
    pub value: f64,
    pub link: Option<VarKey>,
}

/// `total = Σ terms`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Breakdown {
    pub total: f64,
    pub terms: Vec<Term>,
}

impl Breakdown {
    pub fn sum(terms: Vec<Term>) -> Self {
        let total = terms.iter().map(|t| t.value).sum();
        Self { total, terms }
    }

    /// First term "previous", then the flows.
    pub fn stock(prev: f64, flows: Vec<Term>) -> Self {
        let previous = Term {
            label: Cow::Borrowed("previous"),
            value: prev,
            link: None,
        };
        Self::sum(std::iter::once(previous).chain(flows).collect())
    }

    /// `cur − prev`, terms matched by (label, link); unmatched terms count as 0.
    /// Terms follow `cur`'s order, then `prev`-only terms. `total = cur.total − prev.total`.
    /// M4c's `explain_change` (ARC-045) wraps this.
    pub fn change(prev: &Breakdown, cur: &Breakdown) -> Breakdown {
        let same = |a: &Term, b: &Term| a.label == b.label && a.link == b.link;
        let matched = cur.terms.iter().map(|c| {
            let before = prev
                .terms
                .iter()
                .find(|p| same(p, c))
                .map_or(0.0, |p| p.value);
            Term {
                value: c.value - before,
                ..c.clone()
            }
        });
        let removed = prev
            .terms
            .iter()
            .filter(|p| !cur.terms.iter().any(|c| same(p, c)))
            .map(|p| Term {
                value: -p.value,
                ..p.clone()
            });
        Breakdown {
            total: cur.total - prev.total,
            terms: matched.chain(removed).collect(),
        }
    }
}

/// A displayed polity- or world-level variable. The value is private and changes only via `set`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tracked {
    value: f64,
    why: Breakdown,
}

impl Tracked {
    pub fn set(&mut self, b: Breakdown) {
        self.value = b.total;
        self.why = b;
    }
    pub fn get(&self) -> f64 {
        self.value
    }
    pub fn why(&self) -> &Breakdown {
        &self.why
    }
}
