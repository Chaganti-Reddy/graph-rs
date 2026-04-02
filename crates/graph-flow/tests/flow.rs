use graph_flow::{edmonds_karp, ford_fulkerson, hopcroft_karp, min_cut, FlowGraph};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Simple two-path network:
///   0 --10-- 1 --10-- 3
///   |                 |
///   +---5--- 2 ---5---+
/// Max flow = 15 (10 via 1, 5 via 2).
fn two_path_graph() -> FlowGraph {
    let mut g = FlowGraph::new(4);
    g.add_edge(0, 1, 10.0);
    g.add_edge(0, 2, 5.0);
    g.add_edge(1, 3, 10.0);
    g.add_edge(2, 3, 5.0);
    g
}

/// Classic CLRS-style 6-node flow network (Figure 26.1).
///
/// Nodes: s=0, v1=1, v2=2, v3=3, v4=4, t=5
/// Known max flow from s to t = 23.
fn clrs_flow_graph() -> FlowGraph {
    let mut g = FlowGraph::new(6);
    // s=0
    g.add_edge(0, 1, 16.0);
    g.add_edge(0, 2, 13.0);
    // middle
    g.add_edge(1, 2, 10.0);
    g.add_edge(1, 3, 12.0);
    g.add_edge(2, 1, 4.0);
    g.add_edge(2, 4, 14.0);
    g.add_edge(3, 2, 9.0);
    g.add_edge(3, 5, 20.0);
    g.add_edge(4, 3, 7.0);
    g.add_edge(4, 5, 4.0);
    // t=5
    g
}

// ── FlowGraph ─────────────────────────────────────────────────────────────────

#[test]
fn flow_graph_node_count() {
    let g = FlowGraph::new(7);
    assert_eq!(g.node_count(), 7);
}

#[test]
fn flow_graph_add_edge_inserts_forward_and_reverse() {
    let mut g = FlowGraph::new(2);
    g.add_edge(0, 1, 10.0);
    // Forward edge 0→1 in adjacency[0].
    assert_eq!(g.adjacency[0].len(), 1);
    assert_eq!(g.adjacency[0][0].to, 1);
    assert_eq!(g.adjacency[0][0].capacity, 10.0);
    assert_eq!(g.adjacency[0][0].flow, 0.0);
    // Reverse edge 1→0 in adjacency[1] with zero capacity.
    assert_eq!(g.adjacency[1].len(), 1);
    assert_eq!(g.adjacency[1][0].to, 0);
    assert_eq!(g.adjacency[1][0].capacity, 0.0);
}

#[test]
fn flow_graph_residual_is_capacity_minus_flow() {
    let mut g = FlowGraph::new(2);
    g.add_edge(0, 1, 10.0);
    g.push_flow(0, 0, 3.0);
    assert_eq!(g.adjacency[0][0].residual(), 7.0);
    // Reverse edge residual is the flow sent (can push back).
    assert_eq!(g.adjacency[1][0].residual(), 3.0);
}

#[test]
fn flow_graph_reset_flow() {
    let mut g = two_path_graph();
    edmonds_karp(&mut g, 0, 3);
    g.reset_flow();
    for adj in &g.adjacency {
        for edge in adj {
            assert_eq!(edge.flow, 0.0);
        }
    }
}

// ── Ford-Fulkerson ────────────────────────────────────────────────────────────

#[test]
fn ford_fulkerson_two_path_graph() {
    let mut g = two_path_graph();
    assert_eq!(ford_fulkerson(&mut g, 0, 3), 15.0);
}

#[test]
fn ford_fulkerson_clrs_network() {
    let mut g = clrs_flow_graph();
    assert_eq!(ford_fulkerson(&mut g, 0, 5), 23.0);
}

#[test]
fn ford_fulkerson_single_edge() {
    let mut g = FlowGraph::new(2);
    g.add_edge(0, 1, 42.0);
    assert_eq!(ford_fulkerson(&mut g, 0, 1), 42.0);
}

#[test]
fn ford_fulkerson_no_path_is_zero() {
    let mut g = FlowGraph::new(3);
    g.add_edge(0, 1, 10.0);
    // Node 2 is disconnected from 0 and 1.
    assert_eq!(ford_fulkerson(&mut g, 0, 2), 0.0);
}

#[test]
fn ford_fulkerson_source_equals_sink_is_zero() {
    let mut g = two_path_graph();
    assert_eq!(ford_fulkerson(&mut g, 0, 0), 0.0);
}

#[test]
fn ford_fulkerson_bottleneck_limits_flow() {
    // 0 --100-- 1 --1-- 2 --100-- 3
    // Bottleneck is the middle edge (capacity 1).
    let mut g = FlowGraph::new(4);
    g.add_edge(0, 1, 100.0);
    g.add_edge(1, 2, 1.0);
    g.add_edge(2, 3, 100.0);
    assert_eq!(ford_fulkerson(&mut g, 0, 3), 1.0);
}

// ── Edmonds-Karp ──────────────────────────────────────────────────────────────

#[test]
fn edmonds_karp_two_path_graph() {
    let mut g = two_path_graph();
    assert_eq!(edmonds_karp(&mut g, 0, 3), 15.0);
}

#[test]
fn edmonds_karp_clrs_network() {
    let mut g = clrs_flow_graph();
    assert_eq!(edmonds_karp(&mut g, 0, 5), 23.0);
}

#[test]
fn edmonds_karp_matches_ford_fulkerson() {
    let mut g1 = clrs_flow_graph();
    let mut g2 = clrs_flow_graph();
    let ek = edmonds_karp(&mut g1, 0, 5);
    let ff = ford_fulkerson(&mut g2, 0, 5);
    assert!((ek - ff).abs() < 1e-9);
}

#[test]
fn edmonds_karp_single_edge() {
    let mut g = FlowGraph::new(2);
    g.add_edge(0, 1, 7.0);
    assert_eq!(edmonds_karp(&mut g, 0, 1), 7.0);
}

#[test]
fn edmonds_karp_no_path_is_zero() {
    let mut g = FlowGraph::new(3);
    g.add_edge(0, 1, 5.0);
    assert_eq!(edmonds_karp(&mut g, 0, 2), 0.0);
}

#[test]
fn edmonds_karp_parallel_paths() {
    // Three parallel paths of capacity 4 each.
    let mut g = FlowGraph::new(4);
    g.add_edge(0, 1, 4.0);
    g.add_edge(0, 2, 4.0);
    g.add_edge(0, 3, 4.0); // reuse node 3 as intermediate for simplicity
                           // Actually build proper parallel: source=0, sink=5.
    let mut g = FlowGraph::new(6);
    g.add_edge(0, 1, 4.0);
    g.add_edge(0, 2, 4.0);
    g.add_edge(0, 3, 4.0);
    g.add_edge(1, 5, 4.0);
    g.add_edge(2, 5, 4.0);
    g.add_edge(3, 5, 4.0);
    assert_eq!(edmonds_karp(&mut g, 0, 5), 12.0);
}

// ── Min-Cut ───────────────────────────────────────────────────────────────────

#[test]
fn min_cut_capacity_equals_max_flow() {
    let mut g = two_path_graph();
    let max_flow = edmonds_karp(&mut g, 0, 3);
    let cut = min_cut(&g, 0);
    assert!((cut.capacity - max_flow).abs() < 1e-9);
}

#[test]
fn min_cut_clrs_network() {
    let mut g = clrs_flow_graph();
    let max_flow = edmonds_karp(&mut g, 0, 5);
    let cut = min_cut(&g, 0);
    assert!((cut.capacity - max_flow).abs() < 1e-9);
    assert_eq!(cut.capacity, 23.0);
}

#[test]
fn min_cut_source_in_source_side() {
    let mut g = two_path_graph();
    edmonds_karp(&mut g, 0, 3);
    let cut = min_cut(&g, 0);
    assert!(cut.source_side.contains(&0));
    assert!(!cut.sink_side.contains(&0));
}

#[test]
fn min_cut_sink_in_sink_side() {
    let mut g = two_path_graph();
    edmonds_karp(&mut g, 0, 3);
    let cut = min_cut(&g, 0);
    assert!(cut.sink_side.contains(&3));
    assert!(!cut.source_side.contains(&3));
}

#[test]
fn min_cut_partitions_cover_all_nodes() {
    let mut g = clrs_flow_graph();
    edmonds_karp(&mut g, 0, 5);
    let cut = min_cut(&g, 0);
    let mut all: Vec<usize> = cut
        .source_side
        .iter()
        .chain(cut.sink_side.iter())
        .copied()
        .collect();
    all.sort_unstable();
    assert_eq!(all, vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn min_cut_cut_edges_all_cross_partition() {
    let mut g = two_path_graph();
    edmonds_karp(&mut g, 0, 3);
    let cut = min_cut(&g, 0);
    let source_set: std::collections::HashSet<usize> = cut.source_side.iter().copied().collect();
    let sink_set: std::collections::HashSet<usize> = cut.sink_side.iter().copied().collect();
    for (u, v) in &cut.cut_edges {
        assert!(
            source_set.contains(u),
            "cut edge source {u} must be in source_side"
        );
        assert!(
            sink_set.contains(v),
            "cut edge target {v} must be in sink_side"
        );
    }
}

// ── Hopcroft-Karp ─────────────────────────────────────────────────────────────

#[test]
fn hopcroft_karp_perfect_matching() {
    // 1-to-1 bipartite: left i connects only to right i.
    let adj: Vec<Vec<usize>> = (0..4).map(|i| vec![i]).collect();
    let m = hopcroft_karp(&adj, 4);
    assert_eq!(m.matching_size, 4);
}

#[test]
fn hopcroft_karp_general_matching() {
    // Left 0 → right {0,1}, left 1 → right {1}, left 2 → right {2}.
    // Maximum matching: (0,0),(1,1),(2,2) = size 3.
    let adj = vec![vec![0usize, 1], vec![1], vec![2]];
    let m = hopcroft_karp(&adj, 3);
    assert_eq!(m.matching_size, 3);
}

#[test]
fn hopcroft_karp_no_edges() {
    let adj: Vec<Vec<usize>> = vec![vec![], vec![], vec![]];
    let m = hopcroft_karp(&adj, 3);
    assert_eq!(m.matching_size, 0);
    assert!(m.match_left.iter().all(|x| x.is_none()));
}

#[test]
fn hopcroft_karp_bottleneck_matching() {
    // All left nodes compete for the same right node (only 1 can win).
    let adj = vec![vec![0usize], vec![0], vec![0]];
    let m = hopcroft_karp(&adj, 1);
    assert_eq!(m.matching_size, 1);
}

#[test]
fn hopcroft_karp_k33_complete_bipartite() {
    // K_{3,3}: every left connects to every right. Max matching = 3.
    let adj = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
    let m = hopcroft_karp(&adj, 3);
    assert_eq!(m.matching_size, 3);
}

#[test]
fn hopcroft_karp_match_left_and_right_consistent() {
    let adj = vec![vec![0usize, 1], vec![1], vec![2]];
    let m = hopcroft_karp(&adj, 3);
    // For every matched left node, the right node points back.
    for (l, r_opt) in m.match_left.iter().enumerate() {
        if let Some(r) = r_opt {
            assert_eq!(m.match_right[*r], Some(l));
        }
    }
}

#[test]
fn hopcroft_karp_pairs_length_equals_matching_size() {
    let adj = vec![vec![0usize, 1], vec![1], vec![2]];
    let m = hopcroft_karp(&adj, 3);
    assert_eq!(m.pairs().len(), m.matching_size);
}

#[test]
fn hopcroft_karp_empty_left_partition() {
    let adj: Vec<Vec<usize>> = vec![];
    let m = hopcroft_karp(&adj, 3);
    assert_eq!(m.matching_size, 0);
}
