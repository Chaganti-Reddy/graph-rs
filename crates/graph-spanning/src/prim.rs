use crate::kruskal::SpanningTree;
use graph_core::{Edge, Graph, NodeId};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Computes a **Minimum Spanning Tree** using Prim's algorithm.
///
/// Prim's grows the MST from an arbitrary starting node, always adding the
/// minimum-weight edge that connects the current tree to a new node. It uses a
/// binary min-heap (lazy deletion variant) similar to Dijkstra's algorithm.
///
/// Works on both directed and undirected graphs; for directed graphs the
/// algorithm finds the MST of the underlying undirected graph by considering
/// edges in both directions.
///
/// # Returns
///
/// `Some(SpanningTree)` if the graph is connected, or `None` if the graph is
/// disconnected or has no nodes.
///
/// # Complexity
///
/// O((V + E) log V) with a binary heap.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_spanning::prim;
///
/// let mut g: AdjacencyList<&str> = AdjacencyList::undirected();
/// let a = g.add_node("A");
/// let b = g.add_node("B");
/// let c = g.add_node("C");
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 3.0).unwrap();
/// g.add_edge(a, c, 2.0).unwrap();
///
/// let mst = prim(&g).unwrap();
/// assert_eq!(mst.edges.len(), 2);
/// assert_eq!(mst.total_weight, 3.0); // A-B(1) + A-C(2)
/// ```
pub fn prim<G>(graph: &G) -> Option<SpanningTree>
where
    G: Graph<Weight = f64>,
{
    let n = graph.node_count();
    if n == 0 {
        return None;
    }

    // Start from the first node in the graph.
    let start = graph.nodes().next()?;

    // key[node] = minimum edge weight connecting node to the current tree.
    let mut key: HashMap<NodeId, f64> = graph.nodes().map(|n| (n, f64::INFINITY)).collect();
    // parent_edge[node] = the edge (source, target, weight) that connects it.
    let mut parent_edge: HashMap<NodeId, (NodeId, f64)> = HashMap::new();
    let mut in_mst: HashSet<NodeId> = HashSet::new();

    key.insert(start, 0.0);

    // Min-heap: Reverse((key_value, node)).
    let mut heap: BinaryHeap<Reverse<(OrderedFloat<f64>, NodeId)>> = BinaryHeap::new();
    heap.push(Reverse((OrderedFloat(0.0), start)));

    let mut mst_edges: Vec<Edge<f64>> = Vec::with_capacity(n - 1);
    let mut total_weight = 0.0f64;

    while let Some(Reverse((OrderedFloat(w), node))) = heap.pop() {
        // Already committed this node to the MST.
        if in_mst.contains(&node) {
            continue;
        }
        // Lazy deletion: skip stale heap entries.
        if w > *key.get(&node).unwrap_or(&f64::INFINITY) {
            continue;
        }

        in_mst.insert(node);

        // Record the edge that pulled this node into the MST (skip the root).
        if let Some((parent, edge_w)) = parent_edge.get(&node) {
            mst_edges.push(Edge::new(*parent, node, *edge_w));
            total_weight += edge_w;
        }

        // Relax outgoing edges.
        for (neighbour, &weight) in graph.neighbors(node) {
            if !in_mst.contains(&neighbour)
                && weight < *key.get(&neighbour).unwrap_or(&f64::INFINITY)
            {
                key.insert(neighbour, weight);
                parent_edge.insert(neighbour, (node, weight));
                heap.push(Reverse((OrderedFloat(weight), neighbour)));
            }
        }
    }

    // For undirected graphs all edges are visited; for directed we may miss
    // some nodes. Check connectivity.
    if in_mst.len() < n {
        return None;
    }

    Some(SpanningTree {
        edges: mst_edges,
        total_weight,
    })
}
