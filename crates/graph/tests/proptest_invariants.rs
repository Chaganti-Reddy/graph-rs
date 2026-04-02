//! Property-based invariant tests for graph-rs (Step 7.3).
//!
//! Every property here is an algorithm invariant that must hold for *any*
//! valid input, verified across thousands of randomly generated graphs by
//! proptest.
//!
//! Run with:
//!   cargo test -p graph proptest
//!
//! To increase the default number of cases set the env var:
//!   PROPTEST_CASES=10000 cargo test -p graph proptest

use graph::prelude::*;
use proptest::prelude::*;

// ── Graph generators ─────────────────────────────────────────────────────────

/// Proptest strategy: a list of directed weighted edges with node count `n`.
/// Edges are (source, target, weight) where source ≠ target.
fn directed_edges(n: usize, max_edges: usize) -> impl Strategy<Value = Vec<(usize, usize, f64)>> {
    prop::collection::vec((0..n, 0..n, 0.1f64..100.0f64), 0..=max_edges)
        .prop_map(move |edges| edges.into_iter().filter(|&(u, v, _)| u != v).collect())
}

/// Proptest strategy: a list of undirected weighted edges guaranteed to form
/// a *connected* graph on `n` nodes (spanning chain + random extras).
fn connected_undirected_edges(
    n: usize,
    max_extra: usize,
) -> impl Strategy<Value = Vec<(usize, usize, f64)>> {
    prop::collection::vec((0..n, 0..n, 0.1f64..100.0f64), 0..=max_extra).prop_map(
        move |mut extras| {
            // Deterministic spanning chain: 0-1, 1-2, …, (n-2)-(n-1).
            let mut edges: Vec<(usize, usize, f64)> = (0..n - 1).map(|i| (i, i + 1, 1.0)).collect();
            for (u, v, w) in extras.drain(..) {
                if u != v {
                    edges.push((u, v, w));
                }
            }
            edges
        },
    )
}

/// Build a directed AdjacencyList from an edge list.
fn build_directed(n: usize, edges: &[(usize, usize, f64)]) -> AdjacencyList<(), f64> {
    let mut g: AdjacencyList<(), f64> = AdjacencyList::directed();
    let nodes: Vec<NodeId> = (0..n).map(|_| g.add_node(())).collect();
    for &(u, v, w) in edges {
        let _ = g.add_edge(nodes[u], nodes[v], w);
    }
    g
}

/// Build an undirected AdjacencyList from an edge list.
fn build_undirected(n: usize, edges: &[(usize, usize, f64)]) -> AdjacencyList<(), f64> {
    let mut g: AdjacencyList<(), f64> = AdjacencyList::undirected();
    let nodes: Vec<NodeId> = (0..n).map(|_| g.add_node(())).collect();
    for &(u, v, w) in edges {
        let _ = g.add_edge(nodes[u], nodes[v], w);
    }
    g
}

// ── Property 1: Kruskal and Prim agree on total MST weight ───────────────────
//
// For any connected undirected graph both algorithms must produce an MST of
// identical total weight (different edge sets are fine — multiple MSTs may
// exist — but the weight must be the same).

proptest! {
    #[test]
    fn prop_kruskal_prim_weight_agrees(
        n in 2usize..=15,
        edges in connected_undirected_edges(15, 30),
    ) {
        // Trim edges to nodes that exist.
        let edges: Vec<_> = edges.into_iter().filter(|&(u, v, _)| u < n && v < n).collect();
        let g = build_undirected(n, &edges);

        let k = kruskal(&g);
        let p = prim(&g);

        // Both must agree on whether a spanning tree exists.
        prop_assert_eq!(
            k.is_some(),
            p.is_some(),
            "kruskal={:?} prim={:?} disagree on connectivity",
            k.as_ref().map(|t| t.total_weight),
            p.as_ref().map(|t| t.total_weight),
        );

        if let (Some(kt), Some(pt)) = (k, p) {
            prop_assert!(
                (kt.total_weight - pt.total_weight).abs() < 1e-9,
                "weight mismatch: kruskal={} prim={}",
                kt.total_weight,
                pt.total_weight,
            );
        }
    }
}

// ── Property 2: MST has exactly V-1 edges ────────────────────────────────────
//
// A spanning tree on V nodes always has exactly V-1 edges, no exceptions.

proptest! {
    #[test]
    fn prop_mst_has_v_minus_one_edges(
        n in 2usize..=20,
        edges in connected_undirected_edges(20, 40),
    ) {
        let edges: Vec<_> = edges.into_iter().filter(|&(u, v, _)| u < n && v < n).collect();
        let g = build_undirected(n, &edges);

        if let Some(mst) = kruskal(&g) {
            prop_assert_eq!(
                mst.edges.len(),
                n - 1,
                "MST on {} nodes has {} edges, expected {}",
                n, mst.edges.len(), n - 1
            );
        }
    }
}

// ── Property 3: Dijkstra distances ≤ direct edge weight ──────────────────────
//
// For every edge (u → v, w) in the graph, the shortest-path distance from
// source to v must be ≤ dist(source, u) + w.  This is the triangle inequality
// / relaxation soundness condition.

proptest! {
    #[test]
    fn prop_dijkstra_triangle_inequality(
        n in 2usize..=20,
        edges in directed_edges(20, 60),
    ) {
        let edges: Vec<_> = edges.into_iter().filter(|&(u, v, _)| u < n && v < n).collect();
        if edges.is_empty() { return Ok(()); }

        let g = build_directed(n, &edges);
        let src = g.nodes().next().unwrap();
        let result = dijkstra(&g, src).unwrap();

        for &(u, v, w) in &edges {
            let nu = NodeId::new(u);
            let nv = NodeId::new(v);
            if let Some(&du) = result.distances.get(&nu) {
                // If u is reachable, v must be reachable with dist ≤ du + w.
                let dv = result.distances.get(&nv).copied().unwrap_or(f64::INFINITY);
                prop_assert!(
                    dv <= du + w + 1e-9,
                    "triangle violation: dist[{}]={} > dist[{}]={} + edge_w={}",
                    v, dv, u, du, w
                );
            }
        }
    }
}

// ── Property 4: BFS hop-distance ≤ Dijkstra distance ────────────────────────
//
// Since BFS counts hops (each hop = 1) and Dijkstra counts weight (each edge
// weight ≥ 0.1 in our generators), BFS distance * min_weight ≤ Dijkstra dist.
// More simply: if Dijkstra says a node is unreachable, BFS must agree.

proptest! {
    #[test]
    fn prop_bfs_reachability_matches_dijkstra(
        n in 2usize..=20,
        edges in directed_edges(20, 60),
    ) {
        let edges: Vec<_> = edges.into_iter().filter(|&(u, v, _)| u < n && v < n).collect();
        if edges.is_empty() { return Ok(()); }

        let g = build_directed(n, &edges);
        let src = g.nodes().next().unwrap();

        let dijk = dijkstra(&g, src).unwrap();
        let bfs_dist = bfs(&g, src);

        // Every node reachable via BFS must also be reachable via Dijkstra.
        for (node, _hop) in &bfs_dist {
            prop_assert!(
                dijk.distances.contains_key(node),
                "BFS reached node {:?} but Dijkstra did not",
                node
            );
        }

        // Every node reachable via Dijkstra must also be reachable via BFS.
        for (node, _dist) in &dijk.distances {
            prop_assert!(
                bfs_dist.contains_key(node),
                "Dijkstra reached node {:?} but BFS did not",
                node
            );
        }
    }
}

// ── Property 5: Connected components count is consistent ─────────────────────
//
// The number of connected components must be at least 1 and at most n.
// If the graph has a spanning chain it must be exactly 1.

proptest! {
    #[test]
    fn prop_component_count_bounds(
        n in 1usize..=20,
        edges in directed_edges(20, 40),
    ) {
        let edges: Vec<_> = edges.into_iter().filter(|&(u, v, _)| u < n && v < n).collect();
        let g = build_undirected(n, &edges);

        let components = connected_components(&g);
        prop_assert!(
            !components.is_empty() && components.len() <= n,
            "component count {} out of range [1, {}]",
            components.len(), n
        );

        // All nodes must appear in exactly one component.
        let total_nodes: usize = components.iter().map(|c| c.len()).sum();
        prop_assert_eq!(total_nodes, n, "component node count {} ≠ graph node count {}", total_nodes, n);
    }
}

// ── Property 6: Floyd-Warshall diagonal is always zero ───────────────────────
//
// dist[i][i] = 0 for every node (no negative cycles in our positive-weight
// random graphs).

proptest! {
    #[test]
    fn prop_floyd_warshall_diagonal_zero(
        n in 1usize..=12,
        edges in directed_edges(12, 30),
    ) {
        let edges: Vec<_> = edges.into_iter().filter(|&(u, v, _)| u < n && v < n).collect();
        let g = build_directed(n, &edges);

        // Our generated weights are all positive so no negative cycles.
        let dist = floyd_warshall(&g).expect("no negative cycle in positive-weight graph");

        for i in 0..n {
            prop_assert_eq!(
                dist[i][i], 0.0,
                "diagonal dist[{}][{}] = {}, expected 0.0",
                i, i, dist[i][i]
            );
        }
    }
}

// ── Property 7: Topological sort respects all edges ──────────────────────────
//
// In a valid topological ordering, every source node must appear before its
// target node for every directed edge.

proptest! {
    #[test]
    fn prop_topological_sort_respects_edges(
        n in 2usize..=15,
        // Use a DAG-like edge set: only edges u→v where u < v (no back edges).
        raw_edges in prop::collection::vec((0usize..15, 0usize..15, 0.1f64..10.0f64), 0..=30),
    ) {
        // Force DAG structure: only u→v with u < v.
        let edges: Vec<(usize, usize, f64)> = raw_edges
            .into_iter()
            .filter(|&(u, v, _)| u < n && v < n && u < v)
            .collect();

        let g = build_directed(n, &edges);

        let order = topological_sort_dfs(&g)
            .expect("DAG with u<v edges cannot have a cycle");

        // Build position map: node → position in sort order.
        let pos: std::collections::HashMap<NodeId, usize> = order
            .iter()
            .enumerate()
            .map(|(i, &id)| (id, i))
            .collect();

        for &(u, v, _) in &edges {
            let nu = NodeId::new(u);
            let nv = NodeId::new(v);
            let pu = pos[&nu];
            let pv = pos[&nv];
            prop_assert!(
                pu < pv,
                "edge {}→{} violates topo order: pos[{}]={} pos[{}]={}",
                u, v, u, pu, v, pv
            );
        }
    }
}

// ── Property 8: Bellman-Ford and Dijkstra agree on reachable distances ───────
//
// On graphs with non-negative weights both algorithms must produce identical
// shortest distances from the same source.

proptest! {
    #[test]
    fn prop_bellman_ford_matches_dijkstra(
        n in 2usize..=12,
        edges in directed_edges(12, 30),
    ) {
        let edges: Vec<_> = edges.into_iter().filter(|&(u, v, _)| u < n && v < n).collect();
        if edges.is_empty() { return Ok(()); }

        let g = build_directed(n, &edges);
        let src = g.nodes().next().unwrap();

        let dijk = dijkstra(&g, src).unwrap();
        let bf   = bellman_ford(&g, src).expect("no negative cycle in positive-weight graph");

        // Every distance Dijkstra found must match Bellman-Ford.
        for (node, &d_dist) in &dijk.distances {
            let bf_dist = bf.distances.get(node).copied().unwrap_or(f64::INFINITY);
            prop_assert!(
                (d_dist - bf_dist).abs() < 1e-9,
                "node {:?}: dijkstra={} bellman_ford={}",
                node, d_dist, bf_dist
            );
        }
    }
}
