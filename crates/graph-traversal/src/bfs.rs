use graph_collections::Queue;
use graph_core::{Graph, NodeId};
use std::collections::HashMap;

/// Runs a breadth-first search from `start` and returns a map of
/// `NodeId → shortest hop distance` from `start`.
///
/// The distance to `start` itself is `0`. Only nodes reachable from `start`
/// appear in the map.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::bfs;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 1.0).unwrap();
///
/// let dist = bfs(&g, a);
/// assert_eq!(dist[&a], 0);
/// assert_eq!(dist[&b], 1);
/// assert_eq!(dist[&c], 2);
/// ```
pub fn bfs<G: Graph>(graph: &G, start: NodeId) -> HashMap<NodeId, usize> {
    let mut queue: Queue<NodeId> = Queue::new();
    let mut dist: HashMap<NodeId, usize> = HashMap::new();

    queue.enqueue(start);
    dist.insert(start, 0);

    while let Some(node) = queue.dequeue() {
        let d = dist[&node];
        for (neighbour, _) in graph.neighbors(node) {
            if let std::collections::hash_map::Entry::Vacant(e) = dist.entry(neighbour) {
                e.insert(d + 1);
                queue.enqueue(neighbour);
            }
        }
    }

    dist
}

/// BFS result containing both hop distances and the parent map for path
/// reconstruction.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::bfs_tree;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
///
/// let tree = bfs_tree(&g, a);
/// assert_eq!(tree.dist[&b], 1);
/// assert_eq!(tree.parent[&b], a);
/// ```
pub struct BfsTree {
    /// Shortest hop distance from the source to each reachable node.
    pub dist: HashMap<NodeId, usize>,
    /// Parent of each node in the BFS tree (source has no parent entry).
    pub parent: HashMap<NodeId, NodeId>,
}

/// Runs BFS from `start` and returns a [`BfsTree`] with distances and parents.
///
/// Use [`reconstruct_path`](crate::reconstruct_path) on `tree.parent` to
/// extract the shortest path to any reachable node.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_traversal::{bfs_tree, reconstruct_path};
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 1.0).unwrap();
///
/// let tree = bfs_tree(&g, a);
/// let path = reconstruct_path(&tree.parent, a, c).unwrap();
/// assert_eq!(path, vec![a, b, c]);
/// ```
pub fn bfs_tree<G: Graph>(graph: &G, start: NodeId) -> BfsTree {
    let mut queue: Queue<NodeId> = Queue::new();
    let mut dist: HashMap<NodeId, usize> = HashMap::new();
    let mut parent: HashMap<NodeId, NodeId> = HashMap::new();

    queue.enqueue(start);
    dist.insert(start, 0);

    while let Some(node) = queue.dequeue() {
        let d = dist[&node];
        for (neighbour, _) in graph.neighbors(node) {
            if let std::collections::hash_map::Entry::Vacant(e) = dist.entry(neighbour) {
                e.insert(d + 1);
                parent.insert(neighbour, node);
                queue.enqueue(neighbour);
            }
        }
    }

    BfsTree { dist, parent }
}
