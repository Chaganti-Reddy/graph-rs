use graph_core::{Graph, GraphError, NodeId};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Result of a Dijkstra shortest-path search from one source node.
///
/// Contains shortest distances from the source and the parent map needed
/// to reconstruct any shortest path via [`reconstruct_path`].
///
/// [`reconstruct_path`]: crate::dijkstra::reconstruct_path
pub struct DijkstraResult {
    /// Map from `NodeId` to the shortest distance from the source.
    ///
    /// Only nodes reachable from the source have an entry. Unreachable
    /// nodes are absent (not `f64::INFINITY`).
    pub distances: HashMap<NodeId, f64>,
    /// Parent of each node in the shortest-path tree.
    ///
    /// Follow `parents[v] → parents[parents[v]] → … → source` to recover
    /// the shortest path. The source itself has no parent entry.
    pub parents: HashMap<NodeId, NodeId>,
}

/// Runs Dijkstra's algorithm from `source` and returns shortest distances and
/// the parent map.
///
/// Requires all edge weights to be **non-negative**. Negative weights will
/// produce incorrect results without error (use [`bellman_ford`] instead).
///
/// Uses a binary min-heap (`BinaryHeap<Reverse<…>>`) with lazy deletion:
/// stale (distance, node) entries are skipped rather than updated in place.
///
/// # Errors
///
/// Returns [`GraphError::NodeNotFound`] if `source` is not in the graph.
///
/// # Complexity
///
/// O((V + E) log V) with a binary heap.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_shortest_path::dijkstra;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 2.0).unwrap();
/// g.add_edge(a, c, 10.0).unwrap();
///
/// let result = dijkstra(&g, a).unwrap();
/// assert_eq!(result.distances[&c], 3.0); // a→b→c is cheaper than a→c
/// assert_eq!(result.parents[&c], b);
/// ```
///
/// [`bellman_ford`]: crate::bellman_ford::bellman_ford
pub fn dijkstra<G>(graph: &G, source: NodeId) -> Result<DijkstraResult, GraphError>
where
    G: Graph<Weight = f64>,
{
    if !graph.contains_node(source) {
        return Err(GraphError::NodeNotFound(source));
    }

    let mut distances: HashMap<NodeId, f64> = HashMap::new();
    let mut parents: HashMap<NodeId, NodeId> = HashMap::new();

    // Min-heap of (distance, node). `Reverse` makes BinaryHeap a min-heap.
    // OrderedFloat gives us `Ord` on f64 (NaN is treated as largest).
    let mut heap: BinaryHeap<Reverse<(OrderedFloat<f64>, NodeId)>> = BinaryHeap::new();

    distances.insert(source, 0.0);
    heap.push(Reverse((OrderedFloat(0.0), source)));

    while let Some(Reverse((OrderedFloat(dist), node))) = heap.pop() {
        // Lazy deletion: if we have already found a shorter path, skip this entry.
        if let Some(&best) = distances.get(&node) {
            if dist > best {
                continue;
            }
        }

        for (neighbour, &weight) in graph.neighbors(node) {
            let candidate = dist + weight;
            let current_best = distances.get(&neighbour).copied().unwrap_or(f64::INFINITY);

            if candidate < current_best {
                distances.insert(neighbour, candidate);
                parents.insert(neighbour, node);
                heap.push(Reverse((OrderedFloat(candidate), neighbour)));
            }
        }
    }

    Ok(DijkstraResult { distances, parents })
}

/// Reconstructs the shortest path from `start` to `end` using the parent map
/// produced by [`dijkstra`].
///
/// Returns `Some((path, total_distance))` where `path[0] == start` and
/// `path.last() == end`, or `None` if `end` is unreachable from `start`.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_shortest_path::dijkstra;
/// use graph_shortest_path::dijkstra::reconstruct_path;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 2.0).unwrap();
///
/// let result = dijkstra(&g, a).unwrap();
/// let (path, dist) = reconstruct_path(&result, a, c).unwrap();
/// assert_eq!(path, vec![a, b, c]);
/// assert_eq!(dist, 3.0);
/// ```
pub fn reconstruct_path(
    result: &DijkstraResult,
    start: NodeId,
    end: NodeId,
) -> Option<(Vec<NodeId>, f64)> {
    if start == end {
        let dist = *result.distances.get(&start)?;
        return Some((vec![start], dist));
    }

    // Check end is reachable.
    let total_dist = *result.distances.get(&end)?;

    let mut path = vec![end];
    let mut current = end;

    loop {
        let prev = *result.parents.get(&current)?;
        path.push(prev);
        if prev == start {
            break;
        }
        current = prev;
    }

    path.reverse();
    Some((path, total_dist))
}
