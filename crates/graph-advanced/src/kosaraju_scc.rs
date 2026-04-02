use graph_core::{Graph, NodeId};
use std::collections::{HashMap, HashSet};

/// Finds all **Strongly Connected Components** (SCCs) using Kosaraju's algorithm.
///
/// Kosaraju's algorithm uses two DFS passes:
/// 1. Run DFS on the **original** graph, recording nodes in finish order.
/// 2. Run DFS on the **transposed** graph in reverse finish order — each DFS
///    tree in this pass is one SCC.
///
/// # Returns
///
/// A `Vec` of SCCs, each SCC being a `Vec<NodeId>`. SCCs are returned in
/// **topological order** of the condensed DAG (the first SCC has no incoming
/// edges from other SCCs in the condensed graph).
///
/// # Complexity
///
/// O(V + E) — two DFS passes plus one graph transposition.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_advanced::kosaraju_scc;
///
/// // 0 → 1 → 0  (cycle),  1 → 2  (singleton)
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let n: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[0], 1.0).unwrap();
/// g.add_edge(n[1], n[2], 1.0).unwrap();
///
/// let sccs = kosaraju_scc(&g);
/// assert_eq!(sccs.len(), 2);
/// ```
pub fn kosaraju_scc<G>(graph: &G) -> Vec<Vec<NodeId>>
where
    G: Graph<Weight = f64>,
{
    // ── Pass 1: DFS on original graph, record finish order ───────────────────
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut finish_order: Vec<NodeId> = Vec::new();

    for node in graph.nodes() {
        if !visited.contains(&node) {
            dfs_finish(graph, node, &mut visited, &mut finish_order);
        }
    }

    // ── Build transposed graph ────────────────────────────────────────────────
    // Map each NodeId to a dense index for the transpose adjacency list.
    let node_list: Vec<NodeId> = graph.nodes().collect();
    let index_of: HashMap<NodeId, usize> = node_list
        .iter()
        .enumerate()
        .map(|(i, &id)| (id, i))
        .collect();
    let n = node_list.len();

    // transpose[i] = list of nodes that have an edge TO node_list[i].
    let mut transpose: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &u in &node_list {
        for (v, _) in graph.neighbors(u) {
            let ui = index_of[&u];
            let vi = index_of[&v];
            transpose[vi].push(ui); // edge u→v in original becomes v→u in transpose
        }
    }

    // ── Pass 2: DFS on transposed graph in reverse finish order ──────────────
    let mut visited2: HashSet<usize> = HashSet::new();
    let mut result: Vec<Vec<NodeId>> = Vec::new();

    for node in finish_order.into_iter().rev() {
        let idx = index_of[&node];
        if !visited2.contains(&idx) {
            let mut scc_indices: Vec<usize> = Vec::new();
            dfs_transpose(idx, &transpose, &mut visited2, &mut scc_indices);
            let scc = scc_indices.iter().map(|&i| node_list[i]).collect();
            result.push(scc);
        }
    }

    result
}

/// DFS on the original graph; appends nodes to `finish_order` in post-order.
fn dfs_finish<G>(
    graph: &G,
    node: NodeId,
    visited: &mut HashSet<NodeId>,
    finish_order: &mut Vec<NodeId>,
) where
    G: Graph<Weight = f64>,
{
    if !visited.insert(node) {
        return;
    }
    for (neighbour, _) in graph.neighbors(node) {
        dfs_finish(graph, neighbour, visited, finish_order);
    }
    finish_order.push(node);
}

/// DFS on the transposed graph (stored as index-based adjacency list).
fn dfs_transpose(
    node: usize,
    transpose: &[Vec<usize>],
    visited: &mut HashSet<usize>,
    scc: &mut Vec<usize>,
) {
    if !visited.insert(node) {
        return;
    }
    scc.push(node);
    for &neighbour in &transpose[node] {
        dfs_transpose(neighbour, transpose, visited, scc);
    }
}
