use graph_core::{AdjacencyList, Graph};
use graph_shortest_path::{
    astar, bellman_ford, dijkstra,
    dijkstra::reconstruct_path,
    floyd_warshall,
    floyd_warshall::{floyd_warshall_with_paths, reconstruct_fw_path},
};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Classic 6-node weighted directed graph (CLRS style).
///
/// Nodes: s=0, t=1, x=2, y=3, z=4
/// Edges and weights:
///   s→t:10, s→y:5
///   t→x:1,  t→y:2
///   x→z:4
///   y→t:3,  y→x:9,  y→z:2
///   z→s:7,  z→x:6
fn clrs_graph() -> (AdjacencyList<&'static str>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<&str> = AdjacencyList::directed();
    let s = g.add_node("s");
    let t = g.add_node("t");
    let x = g.add_node("x");
    let y = g.add_node("y");
    let z = g.add_node("z");

    g.add_edge(s, t, 10.0).unwrap();
    g.add_edge(s, y, 5.0).unwrap();
    g.add_edge(t, x, 1.0).unwrap();
    g.add_edge(t, y, 2.0).unwrap();
    g.add_edge(x, z, 4.0).unwrap();
    g.add_edge(y, t, 3.0).unwrap();
    g.add_edge(y, x, 9.0).unwrap();
    g.add_edge(y, z, 2.0).unwrap();
    g.add_edge(z, s, 7.0).unwrap();
    g.add_edge(z, x, 6.0).unwrap();

    (g, vec![s, t, x, y, z])
}

/// Simple linear chain: 0→1→2→…→(n-1), each edge weight 1.0.
fn chain(n: usize) -> (AdjacencyList<()>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let ids: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    for i in 0..n - 1 {
        g.add_edge(ids[i], ids[i + 1], 1.0).unwrap();
    }
    (g, ids)
}

// ── Dijkstra ──────────────────────────────────────────────────────────────────

#[test]
fn dijkstra_distances_clrs_from_s() {
    let (g, ids) = clrs_graph();
    let [s, t, x, y, z] = ids[..] else { panic!() };
    let result = dijkstra(&g, s).unwrap();

    assert_eq!(result.distances[&s], 0.0);
    assert_eq!(result.distances[&t], 8.0); // s→y→t: 5+3
    assert_eq!(result.distances[&x], 9.0); // s→y→t→x: 5+3+1
    assert_eq!(result.distances[&y], 5.0); // s→y
    assert_eq!(result.distances[&z], 7.0); // s→y→z: 5+2
}

#[test]
fn dijkstra_source_distance_is_zero() {
    let (g, ids) = chain(5);
    let result = dijkstra(&g, ids[0]).unwrap();
    assert_eq!(result.distances[&ids[0]], 0.0);
}

#[test]
fn dijkstra_unreachable_absent() {
    // Directed chain: 0→1→2, node 3 is isolated.
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let _c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();

    let result = dijkstra(&g, a).unwrap();
    assert!(result.distances.contains_key(&b));
    assert!(!result.distances.contains_key(&_c));
}

#[test]
fn dijkstra_returns_error_for_missing_source() {
    let (g, _) = chain(3);
    let ghost = graph_core::NodeId::new(999);
    assert!(dijkstra(&g, ghost).is_err());
}

#[test]
fn dijkstra_reconstruct_path_clrs() {
    let (g, ids) = clrs_graph();
    let [s, _t, x, _y, _z] = ids[..] else {
        panic!()
    };
    let result = dijkstra(&g, s).unwrap();
    let (path, cost) = reconstruct_path(&result, s, x).unwrap();
    // Optimal: s→y→t→x costs 9
    assert_eq!(cost, 9.0);
    assert_eq!(path[0], s);
    assert_eq!(*path.last().unwrap(), x);
}

#[test]
fn dijkstra_reconstruct_path_unreachable_is_none() {
    let (g, ids) = chain(3);
    let result = dijkstra(&g, ids[2]).unwrap(); // from last node
    assert!(reconstruct_path(&result, ids[2], ids[0]).is_none());
}

#[test]
fn dijkstra_single_node_graph() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let result = dijkstra(&g, a).unwrap();
    assert_eq!(result.distances[&a], 0.0);
    assert_eq!(result.distances.len(), 1);
}

// ── Bellman-Ford ──────────────────────────────────────────────────────────────

#[test]
fn bellman_ford_matches_dijkstra_on_non_negative() {
    let (g, ids) = clrs_graph();
    let s = ids[0];
    let dijk = dijkstra(&g, s).unwrap();
    let bf = bellman_ford(&g, s).unwrap();

    for node in g.nodes() {
        let d = dijk.distances.get(&node).copied();
        let b = bf.distances.get(&node).copied();
        // Both agree on reachable nodes (within floating-point tolerance).
        match (d, b) {
            (Some(dv), Some(bv)) => assert!((dv - bv).abs() < 1e-9, "mismatch at {node:?}"),
            (None, None) => {}
            _ => panic!("reachability disagreement at {node:?}"),
        }
    }
}

#[test]
fn bellman_ford_handles_negative_edge() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 4.0).unwrap();
    g.add_edge(a, c, 2.0).unwrap();
    g.add_edge(c, b, -1.0).unwrap(); // negative weight, no cycle

    let result = bellman_ford(&g, a).unwrap();
    assert_eq!(result.distances[&b], 1.0); // a→c→b: 2 + (-1) = 1
}

#[test]
fn bellman_ford_detects_negative_cycle() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(b, c, -3.0).unwrap();
    g.add_edge(c, a, 1.0).unwrap(); // cycle weight: 1 + (-3) + 1 = -1

    assert!(matches!(
        bellman_ford(&g, a),
        Err(graph_core::GraphError::NegativeCycle)
    ));
}

#[test]
fn bellman_ford_source_distance_zero() {
    let (g, ids) = chain(4);
    let result = bellman_ford(&g, ids[0]).unwrap();
    assert_eq!(result.distances[&ids[0]], 0.0);
}

#[test]
fn bellman_ford_unreachable_absent() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let _b = g.add_node(()); // isolated
    let result = bellman_ford(&g, a).unwrap();
    assert!(!result.distances.contains_key(&_b));
}

// ── Floyd-Warshall ────────────────────────────────────────────────────────────

#[test]
fn floyd_warshall_matches_pairwise_dijkstra() {
    let (g, _ids) = clrs_graph();
    let fw = floyd_warshall(&g).unwrap();

    for src in g.nodes() {
        let dijk = dijkstra(&g, src).unwrap();
        for dst in g.nodes() {
            let fw_dist = fw[src.index()][dst.index()];
            let dijk_dist = dijk.distances.get(&dst).copied().unwrap_or(f64::INFINITY);
            assert!(
                (fw_dist - dijk_dist).abs() < 1e-9,
                "FW[{src:?}][{dst:?}] = {fw_dist}, Dijkstra = {dijk_dist}"
            );
        }
    }
}

#[test]
fn floyd_warshall_diagonal_is_zero() {
    let (g, _) = clrs_graph();
    let fw = floyd_warshall(&g).unwrap();
    for node in g.nodes() {
        assert_eq!(fw[node.index()][node.index()], 0.0);
    }
}

#[test]
fn floyd_warshall_detects_negative_cycle() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(b, a, -3.0).unwrap(); // cycle weight: 1 + (-3) = -2

    assert!(matches!(
        floyd_warshall(&g),
        Err(graph_core::GraphError::NegativeCycle)
    ));
}

#[test]
fn floyd_warshall_no_path_is_infinity() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    // No edges.
    let fw = floyd_warshall(&g).unwrap();
    assert_eq!(fw[a.index()][b.index()], f64::INFINITY);
}

#[test]
fn floyd_warshall_with_paths_reconstruction() {
    let (g, ids) = chain(4);
    let (dist, next) = floyd_warshall_with_paths(&g).unwrap();
    let path = reconstruct_fw_path(&next, ids[0], ids[3]).unwrap();
    assert_eq!(path, ids);
    assert_eq!(dist[0][3], 3.0);
}

#[test]
fn floyd_warshall_with_paths_no_path_is_none() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let (_, next) = floyd_warshall_with_paths(&g).unwrap();
    assert!(reconstruct_fw_path(&next, a, b).is_none());
}

// ── A* ────────────────────────────────────────────────────────────────────────

#[test]
fn astar_finds_shortest_path_chain() {
    let (g, ids) = chain(5);
    let (path, cost) = astar(&g, ids[0], ids[4], |id| {
        (ids[4].index() as f64) - (id.index() as f64)
    })
    .unwrap()
    .unwrap();
    assert_eq!(cost, 4.0);
    assert_eq!(path, ids);
}

#[test]
fn astar_zero_heuristic_matches_dijkstra() {
    let (g, ids) = clrs_graph();
    let s = ids[0];
    let x = ids[2];

    let dijk = dijkstra(&g, s).unwrap();
    let (_, astar_cost) = astar(&g, s, x, |_| 0.0).unwrap().unwrap();

    assert!((dijk.distances[&x] - astar_cost).abs() < 1e-9);
}

#[test]
fn astar_returns_none_when_unreachable() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    // No edges.
    let result = astar(&g, a, b, |_| 0.0).unwrap();
    assert!(result.is_none());
}

#[test]
fn astar_start_equals_goal() {
    let (g, ids) = chain(3);
    let (path, cost) = astar(&g, ids[1], ids[1], |_| 0.0).unwrap().unwrap();
    assert_eq!(cost, 0.0);
    assert_eq!(path, vec![ids[1]]);
}

#[test]
fn astar_error_on_missing_start() {
    let (g, _) = chain(2);
    let ghost = graph_core::NodeId::new(999);
    assert!(astar(&g, ghost, graph_core::NodeId::new(0), |_| 0.0).is_err());
}

#[test]
fn astar_prefers_shorter_path_over_direct() {
    // a→b: 10, a→c→b: 1+1 = 2
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 10.0).unwrap();
    g.add_edge(a, c, 1.0).unwrap();
    g.add_edge(c, b, 1.0).unwrap();

    let (path, cost) = astar(&g, a, b, |_| 0.0).unwrap().unwrap();
    assert_eq!(cost, 2.0);
    assert_eq!(path, vec![a, c, b]);
}
