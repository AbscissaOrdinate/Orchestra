//! Directed graph with sorted edges and derived adjacency (WS-02 §3.2, ARC-015).
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Graph<N, E> {
    pub nodes: Vec<N>,
    /// Sorted by (from, to).
    pub edges: Vec<(u16, u16, E)>,
    /// Edge indices per node, sorted by target. Derived, not serialized; `rebuild` after load.
    #[serde(skip)]
    adj: Vec<Vec<u32>>,
}

impl<N, E> Graph<N, E> {
    pub fn new(nodes: Vec<N>, mut edges: Vec<(u16, u16, E)>) -> Self {
        edges.sort_by_key(|&(from, to, _)| (from, to));
        let mut g = Self {
            nodes,
            edges,
            adj: Vec::new(),
        };
        g.rebuild();
        g
    }

    /// Rebuild the adjacency cache from `edges`.
    pub fn rebuild(&mut self) {
        self.adj = vec![Vec::new(); self.nodes.len()];
        for (i, &(from, _, _)) in self.edges.iter().enumerate() {
            self.adj[usize::from(from)].push(i as u32);
        }
    }

    /// Outgoing edges of `n` as (target, edge), in target order.
    pub fn neighbors(&self, n: u16) -> impl Iterator<Item = (u16, &E)> {
        self.adj[usize::from(n)].iter().map(|&i| {
            let (_, to, ref e) = self.edges[i as usize];
            (to, e)
        })
    }

    /// Minimum-cost path (Dijkstra); `cost` must be ≥ 0. Returns the cost and the node sequence
    /// from `from` to `to`. Ties go to the lower node index, so the result is deterministic.
    pub fn shortest_path(
        &self,
        from: u16,
        to: u16,
        cost: impl Fn(&E) -> f64,
    ) -> Option<(f64, Vec<u16>)> {
        let n = self.nodes.len();
        let mut dist = vec![f64::INFINITY; n];
        let mut prev: Vec<Option<u16>> = vec![None; n];
        let mut heap = BinaryHeap::new();
        dist[usize::from(from)] = 0.0;
        heap.push(Reverse((Cost(0.0), from)));
        while let Some(Reverse((Cost(d), u))) = heap.pop() {
            if u == to {
                break;
            }
            if d > dist[usize::from(u)] {
                continue;
            }
            for (v, e) in self.neighbors(u) {
                let nd = d + cost(e);
                if nd < dist[usize::from(v)] {
                    dist[usize::from(v)] = nd;
                    prev[usize::from(v)] = Some(u);
                    heap.push(Reverse((Cost(nd), v)));
                }
            }
        }
        if !dist[usize::from(to)].is_finite() {
            return None;
        }
        let mut path = vec![to];
        while let Some(p) = prev[usize::from(*path.last()?)] {
            path.push(p);
        }
        path.reverse();
        Some((dist[usize::from(to)], path))
    }
}

/// `f64` with a total order, for the heap.
#[derive(Clone, Copy, PartialEq)]
struct Cost(f64);
impl Eq for Cost {}
impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
