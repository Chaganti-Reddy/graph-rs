//! Minimum s-t cut from a completed max-flow computation.
//!
//! By the **Max-Flow Min-Cut theorem**, the value of the maximum flow from
//! source `s` to sink `t` equals the capacity of the minimum cut separating
//! `s` from `t`. After running a max-flow algorithm, the min-cut can be read
//! directly from the residual graph in O(V + E).

use crate::FlowGraph;
use std::collections::VecDeque;

/// The result of a minimum s-t cut computation.
///
/// The cut partitions the graph's nodes into two sets: `source_side` (nodes
/// reachable from the source in the residual graph) and `sink_side` (all
/// other nodes). Every edge crossing from `source_side` to `sink_side` in the
/// **original** graph is a cut edge; their capacity sum equals the max flow.
#[derive(Debug, Clone)]
pub struct MinCut {
    /// Nodes reachable from the source in the residual graph after max-flow.
    ///
    /// This is the source partition `S` of the min-cut `(S, T)`.
    pub source_side: Vec<usize>,
    /// Nodes **not** reachable from the source in the residual graph.
    ///
    /// This is the sink partition `T` of the min-cut `(S, T)`.
    pub sink_side: Vec<usize>,
    /// The cut edges: `(u, v)` pairs where `u ∈ source_side`, `v ∈ sink_side`,
    /// and there is a forward edge `u → v` in the original graph.
    pub cut_edges: Vec<(usize, usize)>,
    /// Total capacity of all cut edges. Equals the maximum flow value by the
    /// Max-Flow Min-Cut theorem.
    pub capacity: f64,
}

/// Computes the minimum s-t cut from a [`FlowGraph`] on which a max-flow
/// algorithm has already been run.
///
/// The function does **not** run max-flow itself — call [`edmonds_karp`] or
/// [`ford_fulkerson`] first, then pass the same graph here.
///
/// # How it works
///
/// After max-flow saturates the network, BFS from `source` in the **residual**
/// graph (only traversing edges with positive remaining capacity) discovers all
/// nodes still reachable from `source`. These form the source side `S` of the
/// cut. Every original edge crossing from `S` to `T = V \ S` is a cut edge.
///
/// # Arguments
///
/// - `graph` — a [`FlowGraph`] after max-flow has been computed.
/// - `source` — the source node index.
///
/// # Returns
///
/// A [`MinCut`] describing the partition and cut edges.
///
/// # Complexity
///
/// O(V + E).
///
/// # Examples
///
/// ```
/// use graph_flow::{FlowGraph, edmonds_karp, min_cut};
///
/// let mut g = FlowGraph::new(4);
/// g.add_edge(0, 1, 10.0);
/// g.add_edge(0, 2, 5.0);
/// g.add_edge(1, 3, 10.0);
/// g.add_edge(2, 3, 5.0);
///
/// let max_flow = edmonds_karp(&mut g, 0, 3);
/// let cut = min_cut(&g, 0);
///
/// // Max-flow = min-cut capacity (Max-Flow Min-Cut theorem).
/// assert_eq!(max_flow, cut.capacity);
/// assert_eq!(cut.capacity, 15.0);
/// ```
///
/// [`edmonds_karp`]: crate::edmonds_karp::edmonds_karp
/// [`ford_fulkerson`]: crate::ford_fulkerson::ford_fulkerson
pub fn min_cut(graph: &FlowGraph, source: usize) -> MinCut {
    // BFS over the residual graph to find all nodes reachable from source.
    let reachable = residual_reachable(graph, source);

    let mut source_side = Vec::new();
    let mut sink_side = Vec::new();

    for (node, &is_reachable) in reachable.iter().enumerate() {
        if is_reachable {
            source_side.push(node);
        } else {
            sink_side.push(node);
        }
    }

    // Collect cut edges and sum their original capacities.
    let mut cut_edges = Vec::new();
    let mut capacity = 0.0f64;

    for &u in &source_side {
        for edge in &graph.adjacency[u] {
            // A cut edge goes from source_side to sink_side and has positive
            // original capacity (not a reverse/residual-only edge).
            if !reachable[edge.to] && edge.capacity > 0.0 {
                cut_edges.push((u, edge.to));
                capacity += edge.capacity;
            }
        }
    }

    MinCut {
        source_side,
        sink_side,
        cut_edges,
        capacity,
    }
}

/// BFS over the residual graph (only edges with `residual() > 0`).
///
/// Returns a boolean array where `reachable[v]` is `true` iff `v` is
/// reachable from `source` via edges with positive residual capacity.
fn residual_reachable(graph: &FlowGraph, source: usize) -> Vec<bool> {
    let n = graph.node_count();
    let mut reachable = vec![false; n];
    let mut queue = VecDeque::new();

    reachable[source] = true;
    queue.push_back(source);

    while let Some(node) = queue.pop_front() {
        for edge in &graph.adjacency[node] {
            if !reachable[edge.to] && edge.residual() > 0.0 {
                reachable[edge.to] = true;
                queue.push_back(edge.to);
            }
        }
    }

    reachable
}
