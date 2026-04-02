use graph_core::{Graph, NodeId};
use std::collections::HashSet;

/// Finds a **Hamiltonian path** starting from `start` using backtracking.
///
/// A Hamiltonian path visits every node **exactly once**. Unlike an Euler
/// path (which covers every *edge*), a Hamiltonian path covers every *node*.
///
/// # Algorithm
///
/// Depth-first backtracking: at each step, extend the current path to any
/// unvisited neighbour. Backtrack if no extension is possible. The search
/// terminates as soon as a complete path (visiting all `V` nodes) is found.
///
/// This is an exact algorithm — it finds a solution if one exists and
/// reports `None` otherwise. The worst-case time is O(V!), so this is
/// only practical for small graphs (V ≲ 20).
///
/// # Returns
///
/// `Some(Vec<NodeId>)` — a Hamiltonian path starting at `start`, listing
/// nodes in visit order.  
/// `None` — no Hamiltonian path from `start` exists.
///
/// # Complexity
///
/// O(V!) worst case.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_advanced::hamiltonian_path;
///
/// // Path graph: 0-1-2-3 — Hamiltonian path obviously exists.
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[2], 1.0).unwrap();
/// g.add_edge(n[2], n[3], 1.0).unwrap();
///
/// let path = hamiltonian_path(&g, n[0]).unwrap();
/// assert_eq!(path.len(), 4);
/// assert_eq!(path[0], n[0]);
/// ```
pub fn hamiltonian_path<G>(graph: &G, start: NodeId) -> Option<Vec<NodeId>>
where
    G: Graph<Weight = f64>,
{
    let total_nodes = graph.node_count();
    if total_nodes == 0 {
        return None;
    }

    let mut path = vec![start];
    let mut visited: HashSet<NodeId> = HashSet::new();
    visited.insert(start);

    if backtrack(graph, &mut path, &mut visited, total_nodes) {
        Some(path)
    } else {
        None
    }
}

/// Recursive backtracking helper.
///
/// Returns `true` when a complete Hamiltonian path has been found in `path`.
fn backtrack<G>(
    graph: &G,
    path: &mut Vec<NodeId>,
    visited: &mut HashSet<NodeId>,
    total_nodes: usize,
) -> bool
where
    G: Graph<Weight = f64>,
{
    if path.len() == total_nodes {
        return true; // All nodes visited — path is complete.
    }

    let current = *path.last().unwrap();

    // Collect neighbours first to avoid holding a borrow while mutating path.
    let neighbours: Vec<NodeId> = graph.neighbors(current).map(|(v, _)| v).collect();

    for next in neighbours {
        if !visited.contains(&next) {
            path.push(next);
            visited.insert(next);

            if backtrack(graph, path, visited, total_nodes) {
                return true;
            }

            // Backtrack.
            path.pop();
            visited.remove(&next);
        }
    }

    false
}
