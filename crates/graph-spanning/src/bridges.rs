use graph_core::{Graph, NodeId};
use std::collections::HashMap;

/// Finds all **bridges** in an undirected graph using Tarjan's algorithm.
///
/// A bridge (also called a cut edge) is an edge whose removal increases the
/// number of connected components — i.e., it is the only path between some
/// pair of nodes.
///
/// # Algorithm
///
/// DFS assigns each node a **discovery time** (`disc`). The **low-link value**
/// (`low[u]`) is the smallest discovery time reachable from the subtree rooted
/// at `u` via back-edges. An edge `(u, v)` is a bridge iff `low[v] > disc[u]`:
/// the subtree at `v` cannot reach `u` or any earlier node without crossing
/// this edge.
///
/// # Returns
///
/// A `Vec` of bridge edges `(source, target)`. For undirected graphs each
/// bridge is returned once (in the direction it was discovered by DFS).
///
/// # Complexity
///
/// O(V + E).
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_spanning::bridges;
///
/// // Graph: 0-1-2-3, with extra edge 0-2 (making 0-1-2 a cycle).
/// // The only bridge is 2-3.
/// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
/// let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[2], 1.0).unwrap();
/// g.add_edge(n[0], n[2], 1.0).unwrap(); // back-edge, closes a cycle
/// g.add_edge(n[2], n[3], 1.0).unwrap(); // bridge
///
/// let b = bridges(&g);
/// assert_eq!(b.len(), 1);
/// assert!((b[0] == (n[2], n[3])) || (b[0] == (n[3], n[2])));
/// ```
pub fn bridges<G>(graph: &G) -> Vec<(NodeId, NodeId)>
where
    G: Graph<Weight = f64>,
{
    let mut state = BridgeState {
        disc: HashMap::new(),
        low: HashMap::new(),
        timer: 0,
        result: Vec::new(),
    };

    for node in graph.nodes() {
        if !state.disc.contains_key(&node) {
            dfs_bridge(graph, node, None, &mut state);
        }
    }

    state.result
}

struct BridgeState {
    disc: HashMap<NodeId, usize>,
    low: HashMap<NodeId, usize>,
    timer: usize,
    result: Vec<(NodeId, NodeId)>,
}

fn dfs_bridge<G>(graph: &G, node: NodeId, parent: Option<NodeId>, state: &mut BridgeState)
where
    G: Graph<Weight = f64>,
{
    state.disc.insert(node, state.timer);
    state.low.insert(node, state.timer);
    state.timer += 1;

    for (neighbour, _) in graph.neighbors(node) {
        if !state.disc.contains_key(&neighbour) {
            // Tree edge: recurse.
            dfs_bridge(graph, neighbour, Some(node), state);

            // Update low[node] from the subtree.
            let child_low = state.low[&neighbour];
            let node_low = state.low.get_mut(&node).unwrap();
            if child_low < *node_low {
                *node_low = child_low;
            }

            // Bridge condition: subtree at neighbour cannot reach node or earlier.
            if state.low[&neighbour] > state.disc[&node] {
                state.result.push((node, neighbour));
            }
        } else if Some(neighbour) != parent {
            // Back edge: update low[node].
            let neighbour_disc = state.disc[&neighbour];
            let node_low = state.low.get_mut(&node).unwrap();
            if neighbour_disc < *node_low {
                *node_low = neighbour_disc;
            }
        }
    }
}

/// Returns true if the graph has no bridges (is 2-edge-connected).
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_spanning::bridges::is_two_edge_connected;
///
/// // Complete graph K4 has no bridges.
/// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
/// let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
/// for i in 0..4 {
///     for j in i+1..4 {
///         g.add_edge(n[i], n[j], 1.0).unwrap();
///     }
/// }
/// assert!(is_two_edge_connected(&g));
/// ```
pub fn is_two_edge_connected<G>(graph: &G) -> bool
where
    G: Graph<Weight = f64>,
{
    graph.node_count() > 0 && bridges(graph).is_empty()
}
