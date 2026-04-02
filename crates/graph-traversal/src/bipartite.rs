use graph_collections::Queue;
use graph_core::{Graph, NodeId};
use std::collections::HashMap;

/// Result of a bipartite check: the two colour partitions.
///
/// `left` and `right` together contain every reachable node from the seed
/// used in [`is_bipartite`].
pub struct BipartitePartitions {
    /// Nodes coloured `0`.
    pub left: Vec<NodeId>,
    /// Nodes coloured `1`.
    pub right: Vec<NodeId>,
}

/// Checks whether the graph is **bipartite** and returns its 2-colouring.
///
/// A graph is bipartite iff its nodes can be split into two sets such that
/// every edge crosses between the sets (no edge is within the same set).
/// Equivalently, a graph is bipartite iff it contains no odd-length cycle.
///
/// Uses BFS with two-colouring. Returns `Some(partitions)` if bipartite,
/// or `None` if a conflict (odd cycle) is found.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::is_bipartite;
///
/// // Even cycle (square) is bipartite.
/// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// let d = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 1.0).unwrap();
/// g.add_edge(c, d, 1.0).unwrap();
/// g.add_edge(d, a, 1.0).unwrap();
/// assert!(is_bipartite(&g).is_some());
///
/// // Odd cycle (triangle) is NOT bipartite.
/// let mut tri: AdjacencyList<()> = AdjacencyList::undirected();
/// let a = tri.add_node(());
/// let b = tri.add_node(());
/// let c = tri.add_node(());
/// tri.add_edge(a, b, 1.0).unwrap();
/// tri.add_edge(b, c, 1.0).unwrap();
/// tri.add_edge(c, a, 1.0).unwrap();
/// assert!(is_bipartite(&tri).is_none());
/// ```
pub fn is_bipartite<G: Graph>(graph: &G) -> Option<BipartitePartitions> {
    // colour map: 0 or 1 per node.
    let mut colour: HashMap<NodeId, u8> = HashMap::new();
    let mut queue: Queue<NodeId> = Queue::new();

    for seed in graph.nodes() {
        if colour.contains_key(&seed) {
            continue;
        }

        colour.insert(seed, 0);
        queue.enqueue(seed);

        while let Some(node) = queue.dequeue() {
            let node_colour = colour[&node];
            for (neighbour, _) in graph.neighbors(node) {
                match colour.get(&neighbour) {
                    Some(&c) if c == node_colour => return None, // conflict
                    None => {
                        colour.insert(neighbour, 1 - node_colour);
                        queue.enqueue(neighbour);
                    }
                    _ => {}
                }
            }
        }
    }

    let mut left = Vec::new();
    let mut right = Vec::new();
    for (node, c) in &colour {
        if *c == 0 {
            left.push(*node);
        } else {
            right.push(*node);
        }
    }

    Some(BipartitePartitions { left, right })
}
