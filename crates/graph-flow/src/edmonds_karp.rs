//! Edmonds-Karp maximum flow via BFS augmenting paths.

use crate::FlowGraph;
use std::collections::VecDeque;

/// Computes the **maximum flow** from `source` to `sink` using the
/// Edmonds-Karp algorithm (Ford-Fulkerson with BFS augmenting paths).
///
/// Edmonds-Karp improves on [`ford_fulkerson`] by always choosing the
/// **shortest** augmenting path (fewest edges) via BFS rather than an
/// arbitrary path via DFS. This simple change reduces the number of
/// augmentations from potentially O(max_flow) to O(V · E), giving a
/// polynomial runtime guarantee regardless of capacity values.
///
/// # How it works
///
/// 1. BFS finds the shortest augmenting path from `source` to `sink` in the
///    residual graph.
/// 2. The bottleneck capacity (minimum residual along the path) is computed.
/// 3. Flow is pushed: each forward edge's flow increases, each reverse edge's
///    residual capacity increases by the same amount.
/// 4. Repeat until BFS finds no path.
///
/// # Arguments
///
/// - `graph` — a mutable [`FlowGraph`]; flow values are updated in place.
/// - `source` — index of the source node.
/// - `sink` — index of the sink node.
///
/// # Returns
///
/// The total maximum flow value sent from `source` to `sink`.
///
/// # Complexity
///
/// O(V · E²). Polynomial in all cases — safe for general use.
///
/// # Examples
///
/// ```
/// use graph_flow::{FlowGraph, edmonds_karp};
///
/// // CLRS Figure 26.1 style network:
/// //   0 --16-- 1 --12-- 3
/// //   |        |        |
/// //   13       4        20
/// //   |        |        |
/// //   2 --14-- 4 --7--- 5 (sink=5)
/// //   Note: simplified 4-node version below.
/// let mut g = FlowGraph::new(4);
/// g.add_edge(0, 1, 10.0);
/// g.add_edge(0, 2, 5.0);
/// g.add_edge(1, 3, 10.0);
/// g.add_edge(2, 3, 5.0);
///
/// let flow = edmonds_karp(&mut g, 0, 3);
/// assert_eq!(flow, 15.0);
/// ```
///
/// [`ford_fulkerson`]: crate::ford_fulkerson::ford_fulkerson
pub fn edmonds_karp(graph: &mut FlowGraph, source: usize, sink: usize) -> f64 {
    let mut total_flow = 0.0;

    loop {
        // BFS to find shortest augmenting path.
        // parent[v] = (node u, edge index in adjacency[u]) that discovered v.
        let parent = bfs_path(graph, source, sink);

        match parent {
            None => break, // No augmenting path — max flow reached.
            Some(parent) => {
                // Find bottleneck along the path.
                let bottleneck = path_bottleneck(graph, &parent, source, sink);

                // Augment flow along the path (walk from sink back to source).
                let mut node = sink;
                while node != source {
                    let (prev, edge_idx) = parent[node].unwrap();
                    graph.push_flow(prev, edge_idx, bottleneck);
                    node = prev;
                }

                total_flow += bottleneck;
            }
        }
    }

    total_flow
}

/// BFS over the residual graph from `source` to `sink`.
///
/// Returns `Some(parent)` where `parent[v] = Some((u, edge_idx))` encodes
/// the edge `u → v` used to reach `v`, or `None` if `sink` is unreachable.
fn bfs_path(graph: &FlowGraph, source: usize, sink: usize) -> Option<Vec<Option<(usize, usize)>>> {
    let n = graph.node_count();
    let mut parent: Vec<Option<(usize, usize)>> = vec![None; n];
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    visited[source] = true;
    queue.push_back(source);

    while let Some(node) = queue.pop_front() {
        if node == sink {
            return Some(parent);
        }

        for (edge_idx, edge) in graph.adjacency[node].iter().enumerate() {
            if !visited[edge.to] && edge.residual() > 0.0 {
                visited[edge.to] = true;
                parent[edge.to] = Some((node, edge_idx));
                queue.push_back(edge.to);
            }
        }
    }

    None // sink not reached
}

/// Computes the bottleneck (minimum residual) along the BFS path from
/// `source` to `sink` encoded in `parent`.
fn path_bottleneck(
    graph: &FlowGraph,
    parent: &[Option<(usize, usize)>],
    source: usize,
    sink: usize,
) -> f64 {
    let mut bottleneck = f64::INFINITY;
    let mut node = sink;

    while node != source {
        let (prev, edge_idx) = parent[node].unwrap();
        bottleneck = bottleneck.min(graph.adjacency[prev][edge_idx].residual());
        node = prev;
    }

    bottleneck
}
