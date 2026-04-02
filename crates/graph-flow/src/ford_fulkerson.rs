//! Ford-Fulkerson maximum flow via DFS augmenting paths.

use crate::FlowGraph;

/// Computes the **maximum flow** from `source` to `sink` using the
/// Ford-Fulkerson algorithm with DFS augmenting paths.
///
/// Repeatedly finds an augmenting path from `source` to `sink` in the
/// residual graph using DFS, then pushes the bottleneck capacity along that
/// path. Terminates when no augmenting path exists.
///
/// # When to use this vs [`edmonds_karp`]
///
/// Ford-Fulkerson with DFS is simple but has a worst-case complexity of
/// O(E · max_flow) — it can be very slow if max_flow is large and augmenting
/// paths carry only 1 unit of flow each. Prefer [`edmonds_karp`] for general
/// use since BFS augmentation guarantees polynomial time regardless of
/// capacity values.
///
/// Ford-Fulkerson is primarily useful as an educational reference and for
/// small graphs or integer capacities where max_flow is bounded.
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
/// O(E · max_flow). Not guaranteed to terminate with irrational capacities
/// (use integer or rational capacities).
///
/// # Examples
///
/// ```
/// use graph_flow::{FlowGraph, ford_fulkerson};
///
/// // Simple two-path network:
/// //   0 --10-- 1 --10-- 3
/// //   |                 |
/// //   +---5--- 2 --5----+
/// let mut g = FlowGraph::new(4);
/// g.add_edge(0, 1, 10.0);
/// g.add_edge(0, 2, 5.0);
/// g.add_edge(1, 3, 10.0);
/// g.add_edge(2, 3, 5.0);
///
/// let flow = ford_fulkerson(&mut g, 0, 3);
/// assert_eq!(flow, 15.0);
/// ```
///
/// [`edmonds_karp`]: crate::edmonds_karp::edmonds_karp
pub fn ford_fulkerson(graph: &mut FlowGraph, source: usize, sink: usize) -> f64 {
    // Trivial case: no flow is possible when source and sink are the same node.
    if source == sink {
        return 0.0;
    }

    let mut total_flow = 0.0;

    loop {
        // Find an augmenting path via DFS and push flow along it.
        let mut visited = vec![false; graph.node_count()];
        let pushed = dfs_augment(graph, source, sink, f64::INFINITY, &mut visited);

        if pushed == 0.0 {
            break; // No augmenting path found — done.
        }
        total_flow += pushed;
    }

    total_flow
}

/// DFS that finds one augmenting path and returns the bottleneck flow pushed.
///
/// Returns `0.0` if no path from `node` to `sink` exists in the residual
/// graph.
fn dfs_augment(
    graph: &mut FlowGraph,
    node: usize,
    sink: usize,
    pushed: f64,
    visited: &mut Vec<bool>,
) -> f64 {
    if node == sink {
        return pushed;
    }

    visited[node] = true;

    // Iterate by index so we can call push_flow after the recursive call
    // without holding a borrow on graph.adjacency.
    for edge_idx in 0..graph.adjacency[node].len() {
        let neighbour = graph.adjacency[node][edge_idx].to;
        let residual = graph.adjacency[node][edge_idx].residual();

        if visited[neighbour] || residual <= 0.0 {
            continue;
        }

        let bottleneck = dfs_augment(graph, neighbour, sink, pushed.min(residual), visited);

        if bottleneck > 0.0 {
            graph.push_flow(node, edge_idx, bottleneck);
            return bottleneck;
        }
    }

    0.0
}
