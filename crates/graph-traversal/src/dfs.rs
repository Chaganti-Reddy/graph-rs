use graph_collections::Stack;
use graph_core::{Graph, NodeId};
use std::collections::HashSet;

// ── Recursive DFS ─────────────────────────────────────────────────────────────

/// Runs a **recursive** depth-first search from `start`, calling `visitor` the
/// first time each node is discovered.
///
/// Returns nodes in **finish order** (post-order): a node appears in the
/// output only after all nodes reachable from it have been visited. This
/// ordering is the reverse of a topological sort for DAGs.
///
/// # Caveats
///
/// Recursive DFS uses the call stack. For graphs with millions of nodes this
/// may overflow. Use [`dfs_iterative`] for deep graphs.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::dfs_recursive;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 1.0).unwrap();
///
/// let mut visited = Vec::new();
/// let finish = dfs_recursive(&g, a, &mut |id| visited.push(id));
///
/// assert_eq!(visited[0], a); // a discovered first
/// assert_eq!(finish.last(), Some(&a)); // a finishes last (post-order)
/// ```
pub fn dfs_recursive<G, F>(graph: &G, start: NodeId, visitor: &mut F) -> Vec<NodeId>
where
    G: Graph,
    F: FnMut(NodeId),
{
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut finish_order: Vec<NodeId> = Vec::new();
    dfs_recurse(graph, start, visitor, &mut visited, &mut finish_order);
    finish_order
}

fn dfs_recurse<G, F>(
    graph: &G,
    node: NodeId,
    visitor: &mut F,
    visited: &mut HashSet<NodeId>,
    finish_order: &mut Vec<NodeId>,
) where
    G: Graph,
    F: FnMut(NodeId),
{
    if !visited.insert(node) {
        return;
    }
    visitor(node);
    for (neighbour, _) in graph.neighbors(node) {
        dfs_recurse(graph, neighbour, visitor, visited, finish_order);
    }
    finish_order.push(node);
}

// ── Iterative DFS ─────────────────────────────────────────────────────────────

/// Runs an **iterative** depth-first search from `start` using an explicit
/// [`Stack`], calling `visitor` the first time each node is discovered.
///
/// Safe from stack overflow on arbitrarily deep graphs. Note that visit order
/// may differ slightly from the recursive variant because the stack reverses
/// the neighbour ordering.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::dfs_iterative;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(a, c, 1.0).unwrap();
///
/// let mut visited = Vec::new();
/// dfs_iterative(&g, a, &mut |id| visited.push(id));
///
/// assert_eq!(visited.len(), 3);
/// assert!(visited.contains(&a));
/// assert!(visited.contains(&b));
/// assert!(visited.contains(&c));
/// ```
pub fn dfs_iterative<G, F>(graph: &G, start: NodeId, visitor: &mut F)
where
    G: Graph,
    F: FnMut(NodeId),
{
    let mut stack: Stack<NodeId> = Stack::new();
    let mut visited: HashSet<NodeId> = HashSet::new();

    stack.push(start);

    while let Some(node) = stack.pop() {
        if visited.insert(node) {
            visitor(node);
            for (neighbour, _) in graph.neighbors(node) {
                if !visited.contains(&neighbour) {
                    stack.push(neighbour);
                }
            }
        }
    }
}

// ── DFS over all components ───────────────────────────────────────────────────

/// Runs [`dfs_recursive`] from every unvisited node, covering all connected
/// components. Returns a single finish-order vec spanning the whole graph.
///
/// Used internally by topological sort and SCC algorithms.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::dfs::dfs_full;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(()); // isolated
///
/// let finish = dfs_full(&g, &mut |_| {});
/// assert_eq!(finish.len(), 2);
/// ```
pub fn dfs_full<G, F>(graph: &G, visitor: &mut F) -> Vec<NodeId>
where
    G: Graph,
    F: FnMut(NodeId),
{
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut finish_order: Vec<NodeId> = Vec::new();

    for node in graph.nodes() {
        if !visited.contains(&node) {
            dfs_recurse(graph, node, visitor, &mut visited, &mut finish_order);
        }
    }

    finish_order
}
