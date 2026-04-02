// gen.rs is compiled as a separate module into every benchmark binary.
// Each binary only calls the helpers it needs, so Rust fires dead_code on
// the rest. Allow it here rather than annotating every individual item.
#![allow(dead_code)]

/// Shared random graph generators for benchmarks.
///
/// Each function produces a reproducible graph from a fixed seed so benchmark
/// numbers are stable across runs.
use graph::prelude::*;

/// A simple LCG pseudo-random number generator — no external dep needed in
/// benchmark helpers.
pub struct Lcg(u64);

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Lcg(seed)
    }

    /// Returns the next value in [0, modulus).
    pub fn next_mod(&mut self, modulus: u64) -> u64 {
        // LCG parameters from Knuth TAOCP vol 2
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 % modulus
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_mod(1_000_000) as f64) / 1_000_000.0 + 0.001 // (0.001, 1.001]
    }
}

/// Builds a random **directed** weighted graph with `n` nodes and
/// approximately `edge_factor * n` edges. Edge weights are in (0.001, 1.001].
///
/// At least one edge per node is guaranteed so the graph has no isolated sink
/// nodes that would make Dijkstra trivially fast.
pub fn random_directed(n: usize, edge_factor: usize) -> AdjacencyList<(), f64> {
    let mut g: AdjacencyList<(), f64> = AdjacencyList::directed();
    let nodes: Vec<NodeId> = (0..n).map(|_| g.add_node(())).collect();

    let mut rng = Lcg::new(0xDEAD_BEEF_1234_5678);

    // Guarantee connectivity: chain 0→1→2→…→n-1
    for i in 0..n - 1 {
        let w = rng.next_f64();
        let _ = g.add_edge(nodes[i], nodes[i + 1], w);
    }

    // Add random edges up to the target count.
    let extra = edge_factor * n;
    for _ in 0..extra {
        let u = rng.next_mod(n as u64) as usize;
        let v = rng.next_mod(n as u64) as usize;
        if u != v {
            let w = rng.next_f64();
            let _ = g.add_edge(nodes[u], nodes[v], w);
        }
    }

    g
}

/// Builds a random **undirected** weighted graph with `n` nodes and
/// approximately `edge_factor * n` edges.  The graph is guaranteed to be
/// connected (spanning chain + random extras).
pub fn random_undirected(n: usize, edge_factor: usize) -> AdjacencyList<(), f64> {
    let mut g: AdjacencyList<(), f64> = AdjacencyList::undirected();
    let nodes: Vec<NodeId> = (0..n).map(|_| g.add_node(())).collect();

    let mut rng = Lcg::new(0xCAFE_BABE_DEAD_C0DE);

    // Spanning chain guarantees connectivity.
    for i in 0..n - 1 {
        let w = rng.next_f64();
        let _ = g.add_edge(nodes[i], nodes[i + 1], w);
    }

    let extra = edge_factor * n;
    for _ in 0..extra {
        let u = rng.next_mod(n as u64) as usize;
        let v = rng.next_mod(n as u64) as usize;
        if u != v {
            let w = rng.next_f64();
            let _ = g.add_edge(nodes[u], nodes[v], w);
        }
    }

    g
}

/// Returns the first `NodeId` in `g`.
pub fn first_node(g: &AdjacencyList<(), f64>) -> NodeId {
    g.nodes().next().expect("graph has at least one node")
}
