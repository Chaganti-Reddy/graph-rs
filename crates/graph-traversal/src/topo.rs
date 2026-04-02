use crate::cycle::has_cycle_directed;
use crate::dfs::dfs_full;
use graph_collections::Queue;
use graph_core::{Graph, GraphError, NodeId};
use std::collections::HashMap;

/// Returns a topological ordering of the nodes using **DFS finish order**.
///
/// Works only on directed acyclic graphs (DAGs). If the graph contains a
/// cycle, returns [`GraphError::InvalidOperation`].
///
/// The reverse of the DFS finish order is a valid topological sort: a node
/// appears before all nodes that depend on it.
///
/// # Errors
///
/// Returns `Err(GraphError::InvalidOperation)` if the graph has a cycle.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::topological_sort_dfs;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 1.0).unwrap();
///
/// let order = topological_sort_dfs(&g).unwrap();
/// // a must come before b, b before c
/// let pos: std::collections::HashMap<_, _> =
///     order.iter().enumerate().map(|(i, &n)| (n, i)).collect();
/// assert!(pos[&a] < pos[&b]);
/// assert!(pos[&b] < pos[&c]);
/// ```
pub fn topological_sort_dfs<G: Graph>(graph: &G) -> Result<Vec<NodeId>, GraphError> {
    if has_cycle_directed(graph) {
        return Err(GraphError::InvalidOperation(
            "topological sort requires a DAG — cycle detected",
        ));
    }

    let mut finish_order = dfs_full(graph, &mut |_| {});
    finish_order.reverse();
    Ok(finish_order)
}

/// Returns a topological ordering using **Kahn's algorithm** (in-degree BFS).
///
/// Processes nodes with in-degree 0 first; decrementing neighbours' in-degrees
/// as each node is emitted. If not all nodes are emitted the graph has a cycle.
///
/// # Errors
///
/// Returns `Err(GraphError::InvalidOperation)` if a cycle is detected.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::topological_sort_kahn;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(a, c, 1.0).unwrap();
/// g.add_edge(b, c, 1.0).unwrap();
///
/// let order = topological_sort_kahn(&g).unwrap();
/// let pos: std::collections::HashMap<_, _> =
///     order.iter().enumerate().map(|(i, &n)| (n, i)).collect();
/// assert!(pos[&a] < pos[&b]);
/// assert!(pos[&b] < pos[&c]);
/// ```
pub fn topological_sort_kahn<G: Graph>(graph: &G) -> Result<Vec<NodeId>, GraphError> {
    // Compute in-degrees.
    let mut in_degree: HashMap<NodeId, usize> = graph.nodes().map(|n| (n, 0usize)).collect();

    for node in graph.nodes() {
        for (neighbour, _) in graph.neighbors(node) {
            *in_degree.get_mut(&neighbour).unwrap() += 1;
        }
    }

    // Seed queue with all zero-in-degree nodes.
    let mut queue: Queue<NodeId> = Queue::new();
    for (&node, &deg) in &in_degree {
        if deg == 0 {
            queue.enqueue(node);
        }
    }

    let mut order: Vec<NodeId> = Vec::new();

    while let Some(node) = queue.dequeue() {
        order.push(node);
        for (neighbour, _) in graph.neighbors(node) {
            let deg = in_degree.get_mut(&neighbour).unwrap();
            *deg -= 1;
            if *deg == 0 {
                queue.enqueue(neighbour);
            }
        }
    }

    if order.len() != graph.node_count() {
        return Err(GraphError::InvalidOperation(
            "topological sort requires a DAG — cycle detected",
        ));
    }

    Ok(order)
}
