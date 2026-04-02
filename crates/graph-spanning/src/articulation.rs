use graph_core::{Graph, NodeId};
use std::collections::{HashMap, HashSet};

/// Finds all **articulation points** (cut vertices) in an undirected graph.
///
/// An articulation point is a node whose removal increases the number of
/// connected components. Removing it disconnects at least one pair of nodes
/// that were previously reachable from each other.
///
/// # Algorithm
///
/// DFS with discovery time (`disc`) and low-link value (`low`). A node `u` is
/// an articulation point in two cases:
///
/// 1. `u` is the **DFS root** and has **two or more children** in the DFS tree.
/// 2. `u` is **not** the root and has a child `v` with `low[v] >= disc[u]`
///    (the subtree at `v` cannot bypass `u` via a back-edge).
///
/// # Returns
///
/// A `HashSet<NodeId>` of all articulation points. Empty if the graph has no
/// cut vertices (is 2-vertex-connected or trivially small).
///
/// # Complexity
///
/// O(V + E).
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_spanning::articulation_points;
///
/// // Graph: 0-1-2, with extra edge 0-2 (a cycle, no cut vertices)
/// // Plus node 3 attached only to node 1 — making 1 an articulation point.
/// //
/// //  0 - 1 - 3
/// //   \ /
/// //    2
/// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
/// let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[2], 1.0).unwrap();
/// g.add_edge(n[0], n[2], 1.0).unwrap(); // closes cycle: 0-1-2-0
/// g.add_edge(n[1], n[3], 1.0).unwrap(); // n[3] only reachable via n[1]
///
/// let aps = articulation_points(&g);
/// assert!(aps.contains(&n[1]));
/// assert!(!aps.contains(&n[0]));
/// assert_eq!(aps.len(), 1);
/// ```
pub fn articulation_points<G>(graph: &G) -> HashSet<NodeId>
where
    G: Graph<Weight = f64>,
{
    let mut state = ArticulationState {
        disc: HashMap::new(),
        low: HashMap::new(),
        timer: 0,
        result: HashSet::new(),
    };

    for node in graph.nodes() {
        if !state.disc.contains_key(&node) {
            dfs_ap(graph, node, None, &mut state);
        }
    }

    state.result
}

struct ArticulationState {
    disc: HashMap<NodeId, usize>,
    low: HashMap<NodeId, usize>,
    timer: usize,
    result: HashSet<NodeId>,
}

fn dfs_ap<G>(graph: &G, node: NodeId, parent: Option<NodeId>, state: &mut ArticulationState)
where
    G: Graph<Weight = f64>,
{
    state.disc.insert(node, state.timer);
    state.low.insert(node, state.timer);
    state.timer += 1;

    let mut child_count = 0usize;

    for (neighbour, _) in graph.neighbors(node) {
        if !state.disc.contains_key(&neighbour) {
            child_count += 1;
            dfs_ap(graph, neighbour, Some(node), state);

            // Pull up low-link from child.
            let child_low = state.low[&neighbour];
            let node_low = state.low.get_mut(&node).unwrap();
            if child_low < *node_low {
                *node_low = child_low;
            }

            // Case 2: non-root node whose child cannot bypass it.
            if parent.is_some() && state.low[&neighbour] >= state.disc[&node] {
                state.result.insert(node);
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

    // Case 1: root node with multiple DFS children.
    if parent.is_none() && child_count >= 2 {
        state.result.insert(node);
    }
}
