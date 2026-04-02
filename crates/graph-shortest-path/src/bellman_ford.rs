use graph_core::{Graph, GraphError, NodeId};
use std::collections::HashMap;

/// Result of a Bellman-Ford shortest-path search.
pub struct BellmanFordResult {
    /// Map from `NodeId` to the shortest distance from the source.
    ///
    /// Nodes not reachable from the source are absent.
    pub distances: HashMap<NodeId, f64>,
    /// Parent of each node in the shortest-path tree.
    pub parents: HashMap<NodeId, NodeId>,
}

/// Runs the Bellman-Ford algorithm from `source`.
///
/// Unlike Dijkstra, Bellman-Ford handles **negative edge weights**. It detects
/// negative-weight cycles reachable from `source` and returns an error if one
/// exists.
///
/// # Algorithm
///
/// Relax all edges V-1 times. On the V-th relaxation pass, if any distance
/// can still be improved a negative-weight cycle exists.
///
/// # Errors
///
/// - [`GraphError::NodeNotFound`] if `source` is not in the graph.
/// - [`GraphError::NegativeCycle`] if a negative-weight cycle is reachable.
///
/// # Complexity
///
/// O(V · E).
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_shortest_path::bellman_ford;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 4.0).unwrap();
/// g.add_edge(a, c, 2.0).unwrap();
/// g.add_edge(c, b, -1.0).unwrap(); // negative weight, but no negative cycle
///
/// let result = bellman_ford(&g, a).unwrap();
/// assert_eq!(result.distances[&b], 1.0); // a→c→b: 2 + (-1) = 1
/// ```
pub fn bellman_ford<G>(graph: &G, source: NodeId) -> Result<BellmanFordResult, GraphError>
where
    G: Graph<Weight = f64>,
{
    if !graph.contains_node(source) {
        return Err(GraphError::NodeNotFound(source));
    }

    // Initialise: source = 0, all others = ∞.
    let mut distances: HashMap<NodeId, f64> = graph
        .nodes()
        .map(|n| (n, if n == source { 0.0 } else { f64::INFINITY }))
        .collect();
    let mut parents: HashMap<NodeId, NodeId> = HashMap::new();

    let v = graph.node_count();

    // Collect all edges once to avoid repeated iteration over the graph.
    let edges = graph.all_edges();

    // Relax V-1 times.
    for _ in 0..v.saturating_sub(1) {
        let mut updated = false;
        for edge in &edges {
            let u_dist = distances[&edge.source];
            if u_dist == f64::INFINITY {
                continue;
            }
            let candidate = u_dist + edge.weight;
            if candidate < distances[&edge.target] {
                distances.insert(edge.target, candidate);
                parents.insert(edge.target, edge.source);
                updated = true;
            }
        }
        // Early termination: if no update occurred, we're done.
        if !updated {
            break;
        }
    }

    // V-th pass: if any relaxation still succeeds, there's a negative cycle.
    for edge in &edges {
        let u_dist = distances[&edge.source];
        if u_dist == f64::INFINITY {
            continue;
        }
        if u_dist + edge.weight < distances[&edge.target] {
            return Err(GraphError::NegativeCycle);
        }
    }

    // Remove unreachable nodes (distance still ∞) from the result map.
    distances.retain(|_, d| d.is_finite());

    Ok(BellmanFordResult { distances, parents })
}
