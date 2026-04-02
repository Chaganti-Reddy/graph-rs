use graph_core::{Graph, NodeId};
use std::collections::HashMap;

/// Finds all **Strongly Connected Components** (SCCs) using Tarjan's algorithm.
///
/// A strongly connected component is a maximal set of nodes where every node
/// is reachable from every other node via directed edges.
///
/// # Algorithm
///
/// A single DFS pass assigns each node a **discovery time** (`disc`) and a
/// **low-link value** (`low[u]`): the smallest discovery time reachable from
/// the subtree rooted at `u` via tree edges and at most one back-edge.
///
/// Nodes are pushed onto a stack as they are discovered. When we finish
/// processing node `u` and find `low[u] == disc[u]`, `u` is the root of an
/// SCC: we pop the stack until we reach `u`, and all popped nodes form one SCC.
///
/// # Returns
///
/// A `Vec` of SCCs, each SCC being a `Vec<NodeId>`. Components are returned
/// in **reverse topological order** of the condensed DAG: if there is an edge
/// from SCC A to SCC B, then B appears before A in the result.
///
/// # Complexity
///
/// O(V + E) — a single DFS pass over all nodes and edges.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_advanced::tarjan_scc;
///
/// // Two-node cycle: {0, 1} form one SCC; {2} is its own.
/// //   0 → 1 → 0
/// //   1 → 2
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let n: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[0], 1.0).unwrap();
/// g.add_edge(n[1], n[2], 1.0).unwrap();
///
/// let sccs = tarjan_scc(&g);
/// assert_eq!(sccs.len(), 2);
/// ```
pub fn tarjan_scc<G>(graph: &G) -> Vec<Vec<NodeId>>
where
    G: Graph<Weight = f64>,
{
    let mut state = TarjanState {
        disc: HashMap::new(),
        low: HashMap::new(),
        on_stack: HashMap::new(),
        stack: Vec::new(),
        timer: 0,
        result: Vec::new(),
    };

    for node in graph.nodes() {
        if !state.disc.contains_key(&node) {
            dfs_tarjan(graph, node, &mut state);
        }
    }

    state.result
}

struct TarjanState {
    disc: HashMap<NodeId, usize>,
    low: HashMap<NodeId, usize>,
    on_stack: HashMap<NodeId, bool>,
    stack: Vec<NodeId>,
    timer: usize,
    result: Vec<Vec<NodeId>>,
}

fn dfs_tarjan<G>(graph: &G, node: NodeId, state: &mut TarjanState)
where
    G: Graph<Weight = f64>,
{
    state.disc.insert(node, state.timer);
    state.low.insert(node, state.timer);
    state.timer += 1;
    state.on_stack.insert(node, true);
    state.stack.push(node);

    for (neighbour, _) in graph.neighbors(node) {
        if !state.disc.contains_key(&neighbour) {
            // Tree edge: recurse and pull up low-link.
            dfs_tarjan(graph, neighbour, state);
            let child_low = state.low[&neighbour];
            let node_low = state.low.get_mut(&node).unwrap();
            if child_low < *node_low {
                *node_low = child_low;
            }
        } else if *state.on_stack.get(&neighbour).unwrap_or(&false) {
            // Back edge to a node still on the stack (in current SCC).
            let neighbour_disc = state.disc[&neighbour];
            let node_low = state.low.get_mut(&node).unwrap();
            if neighbour_disc < *node_low {
                *node_low = neighbour_disc;
            }
        }
    }

    // If node is the root of an SCC, pop the stack to collect the component.
    if state.low[&node] == state.disc[&node] {
        let mut scc = Vec::new();
        loop {
            let w = state
                .stack
                .pop()
                .expect("stack must not be empty during SCC pop");
            state.on_stack.insert(w, false);
            scc.push(w);
            if w == node {
                break;
            }
        }
        state.result.push(scc);
    }
}
