use crate::DisjointSet;
use graph_core::{Edge, Graph, NodeId};

/// The result of a minimum spanning tree computation.
#[derive(Debug, Clone)]
pub struct SpanningTree {
    /// The edges that form the MST, in the order they were added.
    pub edges: Vec<Edge<f64>>,
    /// Sum of all edge weights in the MST.
    pub total_weight: f64,
}

/// Computes a **Minimum Spanning Tree** using Kruskal's algorithm.
///
/// Kruskal's sorts all edges by weight then greedily adds the cheapest edge
/// that connects two previously disconnected components, using a
/// [`DisjointSet`] to detect cycles in O(α(n)) per edge.
///
/// Works on both directed and undirected graphs. For directed graphs, edges
/// are treated as undirected (the MST is of the underlying undirected graph).
///
/// # Returns
///
/// `Some(SpanningTree)` if the graph is connected (a spanning tree exists),
/// or `None` if the graph is disconnected or empty.
///
/// # Complexity
///
/// O(E log E) dominated by sorting.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_spanning::kruskal;
///
/// //   1       3
/// // A --- B ----- C
/// //  \         /
/// //   ----2----
/// let mut g: AdjacencyList<&str> = AdjacencyList::undirected();
/// let a = g.add_node("A");
/// let b = g.add_node("B");
/// let c = g.add_node("C");
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 3.0).unwrap();
/// g.add_edge(a, c, 2.0).unwrap();
///
/// let mst = kruskal(&g).unwrap();
/// assert_eq!(mst.edges.len(), 2);    // V-1 edges
/// assert_eq!(mst.total_weight, 3.0); // cheapest: A-B(1) + A-C(2)
/// ```
pub fn kruskal<G>(graph: &G) -> Option<SpanningTree>
where
    G: Graph<Weight = f64>,
{
    let n = graph.node_count();
    if n == 0 {
        return None;
    }

    // Collect and sort all edges by weight.
    let mut edges = graph.all_edges();
    edges.sort_by(|a, b| {
        a.weight
            .partial_cmp(&b.weight)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Map NodeId → contiguous index for DisjointSet.
    let node_index = node_index_map(graph);

    let mut ds = DisjointSet::new(n);
    let mut mst_edges: Vec<Edge<f64>> = Vec::with_capacity(n - 1);
    let mut total_weight = 0.0f64;

    for edge in edges {
        let u = node_index[&edge.source];
        let v = node_index[&edge.target];

        // Skip self-loops and edges within the same component.
        if u == v || !ds.union(u, v) {
            continue;
        }

        total_weight += edge.weight;
        mst_edges.push(edge);

        // A spanning tree has exactly V-1 edges.
        if mst_edges.len() == n - 1 {
            break;
        }
    }

    // If we didn't collect V-1 edges the graph is disconnected.
    if mst_edges.len() < n - 1 {
        return None;
    }

    Some(SpanningTree {
        edges: mst_edges,
        total_weight,
    })
}

/// Builds a map from [`NodeId`] to a contiguous `0..n` index for use with
/// [`DisjointSet`].
pub(crate) fn node_index_map<G: Graph>(graph: &G) -> std::collections::HashMap<NodeId, usize> {
    graph.nodes().enumerate().map(|(i, id)| (id, i)).collect()
}
