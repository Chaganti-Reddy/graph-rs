use crate::bfs::bfs;
use graph_core::{Graph, NodeId};
use std::collections::HashSet;

/// Returns all connected components of the graph as a `Vec<Vec<NodeId>>`.
///
/// Each inner `Vec` is one component — the set of nodes reachable from each
/// other. For directed graphs, this treats edges as undirected (weak
/// connectivity). Components are returned in the order their seed node was
/// encountered during iteration.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::connected_components;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(()); // isolated
/// g.add_edge(a, b, 1.0).unwrap();
///
/// let comps = connected_components(&g);
/// assert_eq!(comps.len(), 2);
/// ```
pub fn connected_components<G: Graph>(graph: &G) -> Vec<Vec<NodeId>> {
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut components: Vec<Vec<NodeId>> = Vec::new();

    for node in graph.nodes() {
        if !visited.contains(&node) {
            // BFS from this seed discovers the whole component.
            let dist = bfs(graph, node);
            let component: Vec<NodeId> = dist.keys().copied().collect();
            for &n in &component {
                visited.insert(n);
            }
            components.push(component);
        }
    }

    components
}
