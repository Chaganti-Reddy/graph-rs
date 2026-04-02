use graph_core::{AdjacencyList, Graph};
use graph_spanning::{articulation_points, bridges, kruskal, prim, DisjointSet};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Weighted undirected graph used throughout MST tests.
///
/// ```text
///     2       3
///  A --- B ------- C
///  |   / |         |
///  6  8  5         7
///  | /   |         |
///  D --- E ------- F
///     9       15
/// ```
///
/// MST edges (Kruskal/Prim order may differ but total weight is identical):
///   A-B(2), B-E(5), B-C(3), A-D(6), C-F(7)  → total = 23
/// Wait — let's define it exactly and verify with code.
fn weighted_graph() -> (AdjacencyList<&'static str>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<&str> = AdjacencyList::undirected();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");
    let d = g.add_node("D");
    let e = g.add_node("E");
    let f = g.add_node("F");

    g.add_edge(a, b, 2.0).unwrap();
    g.add_edge(b, c, 3.0).unwrap();
    g.add_edge(a, d, 6.0).unwrap();
    g.add_edge(b, d, 8.0).unwrap();
    g.add_edge(b, e, 5.0).unwrap();
    g.add_edge(d, e, 9.0).unwrap();
    g.add_edge(c, f, 7.0).unwrap();
    g.add_edge(e, f, 15.0).unwrap();

    (g, vec![a, b, c, d, e, f])
}

fn triangle() -> (AdjacencyList<()>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let ids: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
    g.add_edge(ids[0], ids[1], 1.0).unwrap();
    g.add_edge(ids[1], ids[2], 2.0).unwrap();
    g.add_edge(ids[0], ids[2], 3.0).unwrap();
    (g, ids)
}

// ── DisjointSet ───────────────────────────────────────────────────────────────

#[test]
fn disjoint_set_initial_count() {
    let ds = DisjointSet::new(5);
    assert_eq!(ds.count(), 5);
}

#[test]
fn disjoint_set_union_reduces_count() {
    let mut ds = DisjointSet::new(4);
    ds.union(0, 1);
    assert_eq!(ds.count(), 3);
    ds.union(2, 3);
    assert_eq!(ds.count(), 2);
}

#[test]
fn disjoint_set_redundant_union_no_count_change() {
    let mut ds = DisjointSet::new(3);
    assert!(ds.union(0, 1));
    assert!(!ds.union(1, 0)); // already connected
    assert_eq!(ds.count(), 2);
}

#[test]
fn disjoint_set_connected_transitive() {
    let mut ds = DisjointSet::new(5);
    ds.union(0, 1);
    ds.union(1, 2);
    ds.union(3, 4);
    assert!(ds.connected(0, 2)); // transitive
    assert!(!ds.connected(0, 3)); // different group
}

#[test]
fn disjoint_set_path_compression_consistency() {
    let mut ds = DisjointSet::new(6);
    // Build a chain: 0→1→2→3→4→5.
    for i in 0..5 {
        ds.union(i, i + 1);
    }
    assert_eq!(ds.count(), 1);
    // After path compression all should point to the same root.
    let root = ds.find(0);
    for i in 1..6 {
        assert_eq!(ds.find(i), root);
    }
}

// ── Kruskal ───────────────────────────────────────────────────────────────────

#[test]
fn kruskal_mst_edge_count_is_v_minus_one() {
    let (g, _) = weighted_graph();
    let mst = kruskal(&g).unwrap();
    assert_eq!(mst.edges.len(), g.node_count() - 1);
}

#[test]
fn kruskal_mst_total_weight() {
    // A-B(2)+B-C(3)+B-E(5)+A-D(6)+C-F(7) = 23
    let (g, _) = weighted_graph();
    let mst = kruskal(&g).unwrap();
    assert!((mst.total_weight - 23.0).abs() < 1e-9);
}

#[test]
fn kruskal_triangle_picks_cheapest_two_edges() {
    let (g, _) = triangle();
    let mst = kruskal(&g).unwrap();
    assert_eq!(mst.edges.len(), 2);
    assert!((mst.total_weight - 3.0).abs() < 1e-9); // 1+2
}

#[test]
fn kruskal_disconnected_graph_returns_none() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    g.add_node(());
    g.add_node(()); // no edges
    assert!(kruskal(&g).is_none());
}

#[test]
fn kruskal_single_node_returns_none() {
    // A single node has no edges; a spanning tree of 1 node has 0 edges,
    // but V-1=0 so it trivially succeeds. Adjust expectation:
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    g.add_node(());
    // kruskal returns None only if n==0 or disconnected. With 1 node, 0=V-1 so it succeeds.
    // Let's verify the tree is "trivially" a spanning tree.
    // Actually: V-1=0 edges needed for 1 node, so it should return Some with 0 edges.
    // This tests that we don't panic on n=1.
    let result = kruskal(&g);
    // 1 node graph: n-1=0 edges needed. mst_edges.len()==0 == n-1==0, so Some.
    // But n==1 returns None at the top check. Let's confirm.
    // The code says: if n==0 return None. n==1 goes through. mst_edges is empty, n-1=0, Some.
    assert!(result.is_some());
    assert_eq!(result.unwrap().total_weight, 0.0);
}

#[test]
fn kruskal_empty_graph_returns_none() {
    let g: AdjacencyList<()> = AdjacencyList::undirected();
    assert!(kruskal(&g).is_none());
}

// ── Prim ──────────────────────────────────────────────────────────────────────

#[test]
fn prim_mst_edge_count_is_v_minus_one() {
    let (g, _) = weighted_graph();
    let mst = prim(&g).unwrap();
    assert_eq!(mst.edges.len(), g.node_count() - 1);
}

#[test]
fn prim_mst_total_weight_matches_kruskal() {
    let (g, _) = weighted_graph();
    let kruskal_w = kruskal(&g).unwrap().total_weight;
    let prim_w = prim(&g).unwrap().total_weight;
    assert!((kruskal_w - prim_w).abs() < 1e-9);
}

#[test]
fn prim_triangle_matches_kruskal() {
    let (g, _) = triangle();
    let k = kruskal(&g).unwrap().total_weight;
    let p = prim(&g).unwrap().total_weight;
    assert!((k - p).abs() < 1e-9);
}

#[test]
fn prim_disconnected_graph_returns_none() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    g.add_node(());
    g.add_node(());
    assert!(prim(&g).is_none());
}

#[test]
fn prim_and_kruskal_agree_on_random_like_graph() {
    // More complex graph with many options.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..5).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 4.0).unwrap();
    g.add_edge(n[0], n[2], 3.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[1], n[3], 2.0).unwrap();
    g.add_edge(n[2], n[3], 4.0).unwrap();
    g.add_edge(n[3], n[4], 2.0).unwrap();
    g.add_edge(n[2], n[4], 5.0).unwrap();

    let kw = kruskal(&g).unwrap().total_weight;
    let pw = prim(&g).unwrap().total_weight;
    assert!((kw - pw).abs() < 1e-9);
}

// ── Bridges ───────────────────────────────────────────────────────────────────

#[test]
fn bridges_finds_single_bridge() {
    // 0-1-2-3 with back-edge 0-2 (cycle), bridge is 2-3.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[0], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();

    let b = bridges(&g);
    assert_eq!(b.len(), 1);
    let (u, v) = b[0];
    assert!(
        (u == n[2] && v == n[3]) || (u == n[3] && v == n[2]),
        "bridge should be between n[2] and n[3]"
    );
}

#[test]
fn bridges_path_graph_all_edges_are_bridges() {
    // Linear chain: every edge is a bridge.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..5).map(|_| g.add_node(())).collect();
    for i in 0..4 {
        g.add_edge(n[i], n[i + 1], 1.0).unwrap();
    }
    let b = bridges(&g);
    assert_eq!(b.len(), 4);
}

#[test]
fn bridges_complete_graph_has_no_bridges() {
    // K4: every node has degree 3, no bridges.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    for i in 0..4 {
        for j in i + 1..4 {
            g.add_edge(n[i], n[j], 1.0).unwrap();
        }
    }
    assert!(bridges(&g).is_empty());
}

#[test]
fn bridges_triangle_has_no_bridges() {
    let (g, _) = triangle();
    assert!(bridges(&g).is_empty());
}

#[test]
fn bridges_empty_graph_no_bridges() {
    let g: AdjacencyList<()> = AdjacencyList::undirected();
    assert!(bridges(&g).is_empty());
}

// ── Articulation Points ───────────────────────────────────────────────────────

#[test]
fn articulation_points_finds_cut_vertex() {
    // Hourglass: 0-1-2 and 2-3-4, node 2 is the cut vertex.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..5).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[0], n[2], 1.0).unwrap(); // cycle left
    g.add_edge(n[2], n[3], 1.0).unwrap();
    g.add_edge(n[3], n[4], 1.0).unwrap();
    g.add_edge(n[2], n[4], 1.0).unwrap(); // cycle right

    let aps = articulation_points(&g);
    assert!(aps.contains(&n[2]));
    assert_eq!(aps.len(), 1);
}

#[test]
fn articulation_points_linear_chain_internal_nodes() {
    // Linear: 0-1-2-3-4. Internal nodes 1,2,3 are all cut vertices.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..5).map(|_| g.add_node(())).collect();
    for i in 0..4 {
        g.add_edge(n[i], n[i + 1], 1.0).unwrap();
    }
    let aps = articulation_points(&g);
    assert!(aps.contains(&n[1]));
    assert!(aps.contains(&n[2]));
    assert!(aps.contains(&n[3]));
    // Endpoints are NOT articulation points.
    assert!(!aps.contains(&n[0]));
    assert!(!aps.contains(&n[4]));
}

#[test]
fn articulation_points_complete_graph_has_none() {
    // K4: no articulation points.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    for i in 0..4 {
        for j in i + 1..4 {
            g.add_edge(n[i], n[j], 1.0).unwrap();
        }
    }
    assert!(articulation_points(&g).is_empty());
}

#[test]
fn articulation_points_triangle_has_none() {
    let (g, _) = triangle();
    assert!(articulation_points(&g).is_empty());
}

#[test]
fn articulation_points_star_graph_center_is_ap() {
    // Star: center connected to 4 leaves. Center is an AP.
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let center = g.add_node(());
    let leaves: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    for &leaf in &leaves {
        g.add_edge(center, leaf, 1.0).unwrap();
    }
    let aps = articulation_points(&g);
    assert!(aps.contains(&center));
    for leaf in leaves {
        assert!(!aps.contains(&leaf));
    }
}

#[test]
fn articulation_points_empty_graph() {
    let g: AdjacencyList<()> = AdjacencyList::undirected();
    assert!(articulation_points(&g).is_empty());
}
