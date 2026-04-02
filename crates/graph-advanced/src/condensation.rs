use crate::tarjan_scc;
use graph_core::{Graph, NodeId};
use std::collections::HashMap;

/// The result of condensing a directed graph into its SCC DAG.
///
/// Each node in the condensed graph represents one SCC of the original graph.
/// The condensed graph is always a DAG (directed acyclic graph).
#[derive(Debug, Clone)]
pub struct CondensedGraph {
    /// `components[i]` is the list of original [`NodeId`]s that belong to
    /// super-node `i` in the condensed graph.
    pub components: Vec<Vec<NodeId>>,
    /// Edges of the condensed DAG.
    ///
    /// `edges[i]` is the list of super-node indices that super-node `i` has
    /// a directed edge to. Self-loops (SCC to itself) are not included.
    pub edges: Vec<Vec<usize>>,
}

impl CondensedGraph {
    /// Returns the number of super-nodes (SCCs) in the condensed graph.
    #[inline]
    pub fn node_count(&self) -> usize {
        self.components.len()
    }
}

/// Condenses a directed graph into its **SCC DAG**.
///
/// Each strongly connected component (SCC) is collapsed into a single
/// super-node. The resulting graph is always a DAG — any cycle that existed
/// in the original graph is now internal to a single super-node.
///
/// Uses [`tarjan_scc()`] internally to compute the SCCs.
///
/// # Returns
///
/// A [`CondensedGraph`] where:
/// - `components[i]` lists the original nodes that form super-node `i`.
/// - `edges[i]` lists which super-nodes super-node `i` has edges to.
///
/// # Complexity
///
/// O(V + E) — dominated by the SCC computation.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_advanced::condensation;
///
/// // 0 ↔ 1  (one SCC),  2 is a singleton,  edge 0→2.
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let n: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[0], 1.0).unwrap();
/// g.add_edge(n[0], n[2], 1.0).unwrap();
///
/// let cg = condensation(&g);
/// assert_eq!(cg.node_count(), 2); // two SCCs
/// // There must be exactly one inter-SCC edge.
/// let total_edges: usize = cg.edges.iter().map(|e| e.len()).sum();
/// assert_eq!(total_edges, 1);
/// ```
pub fn condensation<G>(graph: &G) -> CondensedGraph
where
    G: Graph<Weight = f64>,
{
    let sccs = tarjan_scc(graph);

    // Map every original NodeId to its SCC index.
    let mut node_to_scc: HashMap<NodeId, usize> = HashMap::new();
    for (scc_idx, scc) in sccs.iter().enumerate() {
        for &node in scc {
            node_to_scc.insert(node, scc_idx);
        }
    }

    let num_sccs = sccs.len();
    let mut edge_set: Vec<std::collections::HashSet<usize>> =
        vec![std::collections::HashSet::new(); num_sccs];

    for node in graph.nodes() {
        let from_scc = node_to_scc[&node];
        for (neighbour, _) in graph.neighbors(node) {
            let to_scc = node_to_scc[&neighbour];
            if from_scc != to_scc {
                edge_set[from_scc].insert(to_scc);
            }
        }
    }

    let edges: Vec<Vec<usize>> = edge_set
        .into_iter()
        .map(|s| s.into_iter().collect())
        .collect();

    CondensedGraph {
        components: sccs,
        edges,
    }
}
