use graph_core::{Graph, GraphError, NodeId};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type AStarHeapEntry = Reverse<(OrderedFloat<f64>, OrderedFloat<f64>, NodeId)>;
type AStarHeap = BinaryHeap<AStarHeapEntry>;

/// Finds the shortest path from `start` to `goal` using the A\* search
/// algorithm with a caller-supplied heuristic function.
///
/// A\* extends Dijkstra by adding a heuristic `h(node)` that estimates the
/// remaining cost to `goal`. The priority of a node is `g(node) + h(node)`
/// where `g` is the known shortest distance from `start`. When the heuristic
/// is **admissible** (never overestimates the true cost), A\* is guaranteed to
/// find the optimal path.
///
/// # Parameters
///
/// - `graph` — any graph with `f64` weights.
/// - `start` — source node.
/// - `goal` — target node.
/// - `h` — heuristic closure: `h(node) -> f64`. Must satisfy `h(node) ≤
///   true_distance(node, goal)`. Pass `|_| 0.0` to degrade to Dijkstra.
///
/// # Returns
///
/// `Some((path, total_cost))` where `path[0] == start` and
/// `path.last() == &goal`, or `None` if no path exists.
///
/// # Errors
///
/// Returns [`GraphError::NodeNotFound`] if `start` or `goal` is not in the
/// graph.
///
/// # Complexity
///
/// O(E log V) with a good heuristic; degrades to O((V+E) log V) with `h=0`.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph, NodeId};
/// use graph_shortest_path::astar;
///
/// // Simple grid: four nodes in a line.
/// // 0 --1-- 1 --1-- 2 --1-- 3
/// let mut g: AdjacencyList<u32> = AdjacencyList::directed();
/// let n: Vec<_> = (0u32..4).map(|i| g.add_node(i)).collect();
/// for i in 0..3 {
///     g.add_edge(n[i], n[i + 1], 1.0).unwrap();
/// }
///
/// // Heuristic: remaining index distance (admissible for unit-weight grid).
/// let goal_idx = 3usize;
/// let (path, cost) = astar(&g, n[0], n[3], |id| {
///     (goal_idx as f64) - (id.index() as f64)
/// })
/// .unwrap()
/// .unwrap();
///
/// assert_eq!(cost, 3.0);
/// assert_eq!(path, n);
/// ```
pub fn astar<G, H>(
    graph: &G,
    start: NodeId,
    goal: NodeId,
    h: H,
) -> Result<Option<(Vec<NodeId>, f64)>, GraphError>
where
    G: Graph<Weight = f64>,
    H: Fn(NodeId) -> f64,
{
    if !graph.contains_node(start) {
        return Err(GraphError::NodeNotFound(start));
    }
    if !graph.contains_node(goal) {
        return Err(GraphError::NodeNotFound(goal));
    }

    // g_score[node] = known shortest distance from start.
    let mut g_score: HashMap<NodeId, f64> = HashMap::new();
    g_score.insert(start, 0.0);

    // Parent map for path reconstruction.
    let mut parents: HashMap<NodeId, NodeId> = HashMap::new();

    // Min-heap ordered by f = g + h.
    // Entries: Reverse((f_score, g_score, node)) — g_score is the tiebreaker.
    let mut open: AStarHeap = AStarHeap::new();

    let start_h = h(start);
    open.push(Reverse((OrderedFloat(start_h), OrderedFloat(0.0), start)));

    while let Some(Reverse((_, OrderedFloat(g), node))) = open.pop() {
        // Goal reached.
        if node == goal {
            let path = rebuild_path(&parents, start, goal);
            return Ok(Some((path, g)));
        }

        // Lazy deletion: skip if we have already settled this node with a
        // lower g-score.
        if let Some(&best_g) = g_score.get(&node) {
            if g > best_g {
                continue;
            }
        }

        for (neighbour, &weight) in graph.neighbors(node) {
            let tentative_g = g + weight;
            let current_best = g_score.get(&neighbour).copied().unwrap_or(f64::INFINITY);

            if tentative_g < current_best {
                g_score.insert(neighbour, tentative_g);
                parents.insert(neighbour, node);

                let f = tentative_g + h(neighbour);
                open.push(Reverse((
                    OrderedFloat(f),
                    OrderedFloat(tentative_g),
                    neighbour,
                )));
            }
        }
    }

    // Open list exhausted without reaching goal.
    Ok(None)
}

fn rebuild_path(parents: &HashMap<NodeId, NodeId>, start: NodeId, end: NodeId) -> Vec<NodeId> {
    if start == end {
        return vec![start];
    }
    let mut path = vec![end];
    let mut current = end;
    while let Some(&prev) = parents.get(&current) {
        path.push(prev);
        if prev == start {
            break;
        }
        current = prev;
    }
    path.reverse();
    path
}
