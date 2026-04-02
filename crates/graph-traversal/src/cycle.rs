use graph_core::{Graph, NodeId};
use std::collections::{HashMap, HashSet};

// ── Directed cycle detection ──────────────────────────────────────────────────

/// DFS colouring state for directed cycle detection.
#[derive(Clone, PartialEq, Eq)]
enum Colour {
    /// Not yet visited.
    White,
    /// Currently on the DFS stack (in progress).
    Gray,
    /// Fully processed.
    Black,
}

/// Returns `true` if the **directed** graph contains at least one cycle.
///
/// Uses three-colour DFS: a back-edge (an edge to a `Gray` node that is still
/// on the current DFS path) proves a cycle exists.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::has_cycle_directed;
///
/// // Acyclic: A → B → C
/// let mut dag: AdjacencyList<()> = AdjacencyList::directed();
/// let a = dag.add_node(());
/// let b = dag.add_node(());
/// let c = dag.add_node(());
/// dag.add_edge(a, b, 1.0).unwrap();
/// dag.add_edge(b, c, 1.0).unwrap();
/// assert!(!has_cycle_directed(&dag));
///
/// // Cyclic: A → B → A
/// let mut cyclic: AdjacencyList<()> = AdjacencyList::directed();
/// let a = cyclic.add_node(());
/// let b = cyclic.add_node(());
/// cyclic.add_edge(a, b, 1.0).unwrap();
/// cyclic.add_edge(b, a, 1.0).unwrap();
/// assert!(has_cycle_directed(&cyclic));
/// ```
pub fn has_cycle_directed<G: Graph>(graph: &G) -> bool {
    let mut colour: HashMap<NodeId, Colour> = graph.nodes().map(|n| (n, Colour::White)).collect();

    for node in graph.nodes() {
        if colour[&node] == Colour::White && dfs_cycle_directed(graph, node, &mut colour) {
            return true;
        }
    }
    false
}

fn dfs_cycle_directed<G: Graph>(
    graph: &G,
    node: NodeId,
    colour: &mut HashMap<NodeId, Colour>,
) -> bool {
    *colour.get_mut(&node).unwrap() = Colour::Gray;

    for (neighbour, _) in graph.neighbors(node) {
        match colour[&neighbour] {
            Colour::Gray => return true, // back-edge → cycle
            Colour::White => {
                if dfs_cycle_directed(graph, neighbour, colour) {
                    return true;
                }
            }
            Colour::Black => {}
        }
    }

    *colour.get_mut(&node).unwrap() = Colour::Black;
    false
}

// ── Undirected cycle detection ────────────────────────────────────────────────

/// Returns `true` if the **undirected** graph contains at least one cycle.
///
/// Uses DFS with parent tracking. A back-edge to any node other than the
/// immediate DFS parent signals a cycle.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::has_cycle_undirected;
///
/// // Tree (no cycle): A — B — C
/// let mut tree: AdjacencyList<()> = AdjacencyList::undirected();
/// let a = tree.add_node(());
/// let b = tree.add_node(());
/// let c = tree.add_node(());
/// tree.add_edge(a, b, 1.0).unwrap();
/// tree.add_edge(b, c, 1.0).unwrap();
/// assert!(!has_cycle_undirected(&tree));
///
/// // Triangle: A — B — C — A
/// let mut tri: AdjacencyList<()> = AdjacencyList::undirected();
/// let a = tri.add_node(());
/// let b = tri.add_node(());
/// let c = tri.add_node(());
/// tri.add_edge(a, b, 1.0).unwrap();
/// tri.add_edge(b, c, 1.0).unwrap();
/// tri.add_edge(c, a, 1.0).unwrap();
/// assert!(has_cycle_undirected(&tri));
/// ```
pub fn has_cycle_undirected<G: Graph>(graph: &G) -> bool {
    let mut visited: HashSet<NodeId> = HashSet::new();

    for node in graph.nodes() {
        if !visited.contains(&node) && dfs_cycle_undirected(graph, node, None, &mut visited) {
            return true;
        }
    }
    false
}

fn dfs_cycle_undirected<G: Graph>(
    graph: &G,
    node: NodeId,
    parent: Option<NodeId>,
    visited: &mut HashSet<NodeId>,
) -> bool {
    visited.insert(node);

    for (neighbour, _) in graph.neighbors(node) {
        if !visited.contains(&neighbour) {
            if dfs_cycle_undirected(graph, neighbour, Some(node), visited) {
                return true;
            }
        } else if Some(neighbour) != parent {
            // Back-edge to a visited non-parent node.
            return true;
        }
    }
    false
}
