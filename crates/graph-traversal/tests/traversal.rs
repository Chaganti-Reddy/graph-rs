use graph_core::{AdjacencyList, Graph};
use graph_traversal::{
    bfs, bfs_tree, connected_components, dfs_iterative, dfs_recursive, has_cycle_directed,
    has_cycle_undirected, is_bipartite, reconstruct_path, topological_sort_dfs,
    topological_sort_kahn,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Builds a directed path graph: 0 → 1 → 2 → … → (n-1)
fn path_graph(n: usize) -> (AdjacencyList<()>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let ids: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    for i in 0..n - 1 {
        g.add_edge(ids[i], ids[i + 1], 1.0).unwrap();
    }
    (g, ids)
}

/// Builds a directed triangle: 0 → 1 → 2 → 0
fn directed_triangle() -> (AdjacencyList<()>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let ids: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
    g.add_edge(ids[0], ids[1], 1.0).unwrap();
    g.add_edge(ids[1], ids[2], 1.0).unwrap();
    g.add_edge(ids[2], ids[0], 1.0).unwrap();
    (g, ids)
}

// ── DFS recursive ─────────────────────────────────────────────────────────────

#[test]
fn dfs_recursive_visits_all_reachable() {
    let (g, ids) = path_graph(4);
    let mut visited = Vec::new();
    dfs_recursive(&g, ids[0], &mut |n| visited.push(n));
    assert_eq!(visited.len(), 4);
}

#[test]
fn dfs_recursive_finish_order_postorder() {
    let (g, ids) = path_graph(3);
    let finish = dfs_recursive(&g, ids[0], &mut |_| {});
    // Leaf finishes first in a linear chain.
    assert_eq!(finish[0], ids[2]);
    assert_eq!(finish[2], ids[0]);
}

#[test]
fn dfs_recursive_isolated_start() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    g.add_node(()); // isolated b
    let mut visited = Vec::new();
    dfs_recursive(&g, a, &mut |n| visited.push(n));
    assert_eq!(visited, vec![a]);
}

// ── DFS iterative ─────────────────────────────────────────────────────────────

#[test]
fn dfs_iterative_visits_same_nodes_as_recursive() {
    let (g, ids) = path_graph(5);
    let mut rec = Vec::new();
    let mut iter = Vec::new();
    dfs_recursive(&g, ids[0], &mut |n| rec.push(n));
    dfs_iterative(&g, ids[0], &mut |n| iter.push(n));
    // Both visit the same set of nodes (order may differ).
    rec.sort();
    iter.sort();
    assert_eq!(rec, iter);
}

#[test]
fn dfs_iterative_does_not_revisit() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    let mut count = 0;
    dfs_iterative(&g, a, &mut |_| count += 1);
    assert_eq!(count, 2);
}

// ── BFS ───────────────────────────────────────────────────────────────────────

#[test]
fn bfs_distances_linear_chain() {
    let (g, ids) = path_graph(4);
    let dist = bfs(&g, ids[0]);
    assert_eq!(dist[&ids[0]], 0);
    assert_eq!(dist[&ids[1]], 1);
    assert_eq!(dist[&ids[2]], 2);
    assert_eq!(dist[&ids[3]], 3);
}

#[test]
fn bfs_does_not_reach_unreachable() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(()); // no edge from a to b
    let dist = bfs(&g, a);
    assert!(dist.contains_key(&a));
    assert!(!dist.contains_key(&b));
}

#[test]
fn bfs_tree_parent_map_correct() {
    let (g, ids) = path_graph(3);
    let tree = bfs_tree(&g, ids[0]);
    assert_eq!(tree.parent[&ids[1]], ids[0]);
    assert_eq!(tree.parent[&ids[2]], ids[1]);
    assert!(!tree.parent.contains_key(&ids[0])); // root has no parent
}

// ── Path reconstruction ───────────────────────────────────────────────────────

#[test]
fn reconstruct_path_linear() {
    let (g, ids) = path_graph(4);
    let tree = bfs_tree(&g, ids[0]);
    let path = reconstruct_path(&tree.parent, ids[0], ids[3]).unwrap();
    assert_eq!(path, ids);
}

#[test]
fn reconstruct_path_same_node() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let tree = bfs_tree(&g, a);
    let path = reconstruct_path(&tree.parent, a, a).unwrap();
    assert_eq!(path, vec![a]);
}

#[test]
fn reconstruct_path_unreachable_returns_none() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let tree = bfs_tree(&g, a);
    assert!(reconstruct_path(&tree.parent, a, b).is_none());
}

// ── Cycle detection – directed ────────────────────────────────────────────────

#[test]
fn no_cycle_in_dag() {
    let (g, _) = path_graph(5);
    assert!(!has_cycle_directed(&g));
}

#[test]
fn cycle_detected_in_directed_triangle() {
    let (g, _) = directed_triangle();
    assert!(has_cycle_directed(&g));
}

#[test]
fn self_loop_is_cycle_directed() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    g.add_edge(a, a, 1.0).unwrap();
    assert!(has_cycle_directed(&g));
}

#[test]
fn empty_graph_has_no_cycle_directed() {
    let g: AdjacencyList<()> = AdjacencyList::directed();
    assert!(!has_cycle_directed(&g));
}

// ── Cycle detection – undirected ──────────────────────────────────────────────

#[test]
fn no_cycle_in_tree_undirected() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(b, c, 1.0).unwrap();
    assert!(!has_cycle_undirected(&g));
}

#[test]
fn triangle_is_cycle_undirected() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(b, c, 1.0).unwrap();
    g.add_edge(c, a, 1.0).unwrap();
    assert!(has_cycle_undirected(&g));
}

// ── Topological sort ──────────────────────────────────────────────────────────

fn topo_is_valid(order: &[graph_core::NodeId], g: &AdjacencyList<()>) -> bool {
    let pos: std::collections::HashMap<_, _> =
        order.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    for u in g.nodes() {
        for (v, _) in g.neighbors(u) {
            if pos[&u] >= pos[&v] {
                return false;
            }
        }
    }
    true
}

#[test]
fn topo_dfs_valid_on_dag() {
    let (g, _) = path_graph(5);
    let order = topological_sort_dfs(&g).unwrap();
    assert!(topo_is_valid(&order, &g));
}

#[test]
fn topo_kahn_valid_on_dag() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(a, c, 1.0).unwrap();
    g.add_edge(b, c, 1.0).unwrap();
    let order = topological_sort_kahn(&g).unwrap();
    assert!(topo_is_valid(&order, &g));
}

#[test]
fn topo_dfs_returns_error_on_cycle() {
    let (g, _) = directed_triangle();
    assert!(topological_sort_dfs(&g).is_err());
}

#[test]
fn topo_kahn_returns_error_on_cycle() {
    let (g, _) = directed_triangle();
    assert!(topological_sort_kahn(&g).is_err());
}

#[test]
fn topo_dfs_and_kahn_same_length() {
    let (g, _) = path_graph(6);
    let dfs_order = topological_sort_dfs(&g).unwrap();
    let kahn_order = topological_sort_kahn(&g).unwrap();
    assert_eq!(dfs_order.len(), kahn_order.len());
}

// ── Connected components ──────────────────────────────────────────────────────

#[test]
fn single_component_fully_connected() {
    let (g, _) = path_graph(4);
    // BFS treats directed edges, so only one component from node 0's
    // perspective. connected_components seeds each unvisited node.
    let comps = connected_components(&g);
    // In a directed path, each unreached node spawns its own BFS.
    // This tests that all nodes are covered.
    let total: usize = comps.iter().map(|c| c.len()).sum();
    assert_eq!(total, g.node_count());
}

#[test]
fn two_isolated_components() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(c, d, 1.0).unwrap();

    let comps = connected_components(&g);
    assert_eq!(comps.len(), 2);
    let total: usize = comps.iter().map(|c| c.len()).sum();
    assert_eq!(total, 4);
}

#[test]
fn all_isolated_nodes() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    for _ in 0..5 {
        g.add_node(());
    }
    let comps = connected_components(&g);
    assert_eq!(comps.len(), 5);
}

// ── Bipartite check ───────────────────────────────────────────────────────────

#[test]
fn even_cycle_is_bipartite() {
    // Square: a — b — c — d — a
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(b, c, 1.0).unwrap();
    g.add_edge(c, d, 1.0).unwrap();
    g.add_edge(d, a, 1.0).unwrap();

    let result = is_bipartite(&g);
    assert!(result.is_some());
    let parts = result.unwrap();
    assert_eq!(parts.left.len() + parts.right.len(), 4);
}

#[test]
fn odd_cycle_is_not_bipartite() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(b, c, 1.0).unwrap();
    g.add_edge(c, a, 1.0).unwrap();
    assert!(is_bipartite(&g).is_none());
}

#[test]
fn k33_is_bipartite() {
    // Complete bipartite K_{3,3}: left={0,1,2}, right={3,4,5}
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let ids: Vec<_> = (0..6).map(|_| g.add_node(())).collect();
    for &l in &ids[0..3] {
        for &r in &ids[3..6] {
            g.add_edge(l, r, 1.0).unwrap();
        }
    }
    assert!(is_bipartite(&g).is_some());
}

#[test]
fn empty_graph_is_bipartite() {
    let g: AdjacencyList<()> = AdjacencyList::undirected();
    assert!(is_bipartite(&g).is_some());
}

#[test]
fn single_node_is_bipartite() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    g.add_node(());
    assert!(is_bipartite(&g).is_some());
}
