use graph_advanced::{
    condensation, euler_circuit, euler_path, hamiltonian_path, kosaraju_scc, tarjan_scc,
    tsp_held_karp, EulerError,
};
use graph_core::{AdjacencyList, Graph};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Directed graph with two cycles sharing a node.
///
/// ```text
/// 0 → 1 → 2 → 0   (SCC: {0,1,2})
/// 2 → 3 → 4 → 3   (SCC: {3,4})
/// ```
fn two_scc_graph() -> (AdjacencyList<()>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..5).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[0], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();
    g.add_edge(n[3], n[4], 1.0).unwrap();
    g.add_edge(n[4], n[3], 1.0).unwrap();
    (g, n)
}

/// Simple triangle (undirected) — even degrees, Euler circuit exists.
fn triangle() -> (AdjacencyList<()>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[0], 1.0).unwrap();
    (g, n)
}

/// Path graph 0-1-2-3 (undirected) — two odd-degree nodes, Euler path exists.
fn path_graph() -> (AdjacencyList<()>, Vec<graph_core::NodeId>) {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();
    (g, n)
}

// ── Tarjan SCC ────────────────────────────────────────────────────────────────

#[test]
fn tarjan_single_node() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n = g.add_node(());
    let sccs = tarjan_scc(&g);
    assert_eq!(sccs.len(), 1);
    assert_eq!(sccs[0], vec![n]);
}

#[test]
fn tarjan_two_scc_graph() {
    let (g, _) = two_scc_graph();
    let sccs = tarjan_scc(&g);
    assert_eq!(sccs.len(), 2);
    let sizes: Vec<usize> = {
        let mut s: Vec<usize> = sccs.iter().map(|c| c.len()).collect();
        s.sort();
        s
    };
    assert_eq!(sizes, vec![2, 3]);
}

#[test]
fn tarjan_linear_chain_all_singletons() {
    // 0→1→2→3: no back edges, every node is its own SCC.
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();
    let sccs = tarjan_scc(&g);
    assert_eq!(sccs.len(), 4);
    assert!(sccs.iter().all(|c| c.len() == 1));
}

#[test]
fn tarjan_full_cycle_one_scc() {
    // Complete cycle: every node reachable from every other → one SCC.
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..5).map(|_| g.add_node(())).collect();
    for i in 0..5 {
        g.add_edge(n[i], n[(i + 1) % 5], 1.0).unwrap();
    }
    let sccs = tarjan_scc(&g);
    assert_eq!(sccs.len(), 1);
    assert_eq!(sccs[0].len(), 5);
}

// ── Kosaraju SCC ──────────────────────────────────────────────────────────────

#[test]
fn kosaraju_single_node() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n = g.add_node(());
    let sccs = kosaraju_scc(&g);
    assert_eq!(sccs.len(), 1);
    assert_eq!(sccs[0], vec![n]);
}

#[test]
fn kosaraju_two_scc_graph() {
    let (g, _) = two_scc_graph();
    let sccs = kosaraju_scc(&g);
    assert_eq!(sccs.len(), 2);
    let sizes: Vec<usize> = {
        let mut s: Vec<usize> = sccs.iter().map(|c| c.len()).collect();
        s.sort();
        s
    };
    assert_eq!(sizes, vec![2, 3]);
}

#[test]
fn kosaraju_linear_chain_all_singletons() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();
    let sccs = kosaraju_scc(&g);
    assert_eq!(sccs.len(), 4);
    assert!(sccs.iter().all(|c| c.len() == 1));
}

// ── Tarjan and Kosaraju agree ─────────────────────────────────────────────────

#[test]
fn tarjan_and_kosaraju_agree_on_scc_count_and_sizes() {
    let (g, _) = two_scc_graph();
    let t_sccs = tarjan_scc(&g);
    let k_sccs = kosaraju_scc(&g);

    let mut t_sizes: Vec<usize> = t_sccs.iter().map(|c| c.len()).collect();
    let mut k_sizes: Vec<usize> = k_sccs.iter().map(|c| c.len()).collect();
    t_sizes.sort();
    k_sizes.sort();

    assert_eq!(
        t_sizes, k_sizes,
        "SCC sizes must match between Tarjan and Kosaraju"
    );
}

// ── Condensation ──────────────────────────────────────────────────────────────

#[test]
fn condensation_two_scc_graph() {
    let (g, _) = two_scc_graph();
    let cg = condensation(&g);
    assert_eq!(cg.node_count(), 2);
    // There must be exactly one inter-SCC edge.
    let total_edges: usize = cg.edges.iter().map(|e| e.len()).sum();
    assert_eq!(total_edges, 1);
}

#[test]
fn condensation_dag_input_unchanged() {
    // A DAG has each node as its own SCC; condensation produces same # nodes.
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();
    let cg = condensation(&g);
    assert_eq!(cg.node_count(), 4);
}

#[test]
fn condensation_full_cycle_one_supernode() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    for i in 0..4 {
        g.add_edge(n[i], n[(i + 1) % 4], 1.0).unwrap();
    }
    let cg = condensation(&g);
    assert_eq!(cg.node_count(), 1);
    assert!(cg.edges[0].is_empty());
}

// ── Euler Circuit ─────────────────────────────────────────────────────────────

#[test]
fn euler_circuit_triangle() {
    let (g, _) = triangle();
    let circuit = euler_circuit(&g).unwrap();
    assert_eq!(circuit.len(), 4); // 3 edges + return
    assert_eq!(circuit.first(), circuit.last());
}

#[test]
fn euler_circuit_uses_all_edges() {
    // Construct a connected graph where every node has even degree.
    // Use two triangles sharing an edge: 0-1-2-0 plus 0-2-3-0.
    // Degrees: 0→3+1=4? No — let's be precise:
    //   edges: (0,1),(1,2),(2,0),(2,3),(3,0)  → degrees: 0=3,1=2,2=3,3=2  (odd at 0,2)
    // Simplest even-degree connected graph: the "house" is hard to reason about.
    // Use the complete graph K3 (triangle) plus an extra parallel path:
    //   0-1, 1-2, 2-0  (triangle, all degree 2, even) plus
    //   0-3, 3-1  (adds degree 1 to 0,3,3,1 → degrees 0=3,1=3 odd)
    // Easiest: just use two disjoint triangles joined at a node — but that
    // makes degrees 4 at the shared node and 2 elsewhere.
    // Concretely: 4-cycle 0-1-2-3-0 (all degree 2, even).
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();
    g.add_edge(n[3], n[0], 1.0).unwrap();
    let circuit = euler_circuit(&g).unwrap();
    assert_eq!(circuit.len(), g.edge_count() + 1);
    assert_eq!(circuit.first(), circuit.last());
}

#[test]
fn euler_circuit_fails_on_odd_degree() {
    // Path graph has two odd-degree nodes — no circuit.
    let (g, _) = path_graph();
    assert_eq!(euler_circuit(&g), Err(EulerError::NoCircuit));
}

// ── Euler Path ────────────────────────────────────────────────────────────────

#[test]
fn euler_path_on_path_graph() {
    let (g, _) = path_graph();
    let path = euler_path(&g).unwrap();
    assert_eq!(path.len(), 4); // 3 edges → 4 nodes
}

#[test]
fn euler_path_fails_on_triangle() {
    // Triangle: all nodes have even degree → 0 odd-degree nodes → NoPath.
    let (g, _) = triangle();
    assert_eq!(euler_path(&g), Err(EulerError::NoPath));
}

// ── Hamiltonian Path ──────────────────────────────────────────────────────────

#[test]
fn hamiltonian_path_on_chain() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    g.add_edge(n[1], n[2], 1.0).unwrap();
    g.add_edge(n[2], n[3], 1.0).unwrap();
    let path = hamiltonian_path(&g, n[0]).unwrap();
    assert_eq!(path.len(), 4);
    assert_eq!(path[0], n[0]);
}

#[test]
fn hamiltonian_path_visits_each_node_once() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..5).map(|_| g.add_node(())).collect();
    // Complete directed cycle.
    for i in 0..5 {
        g.add_edge(n[i], n[(i + 1) % 5], 1.0).unwrap();
        g.add_edge(n[i], n[(i + 2) % 5], 1.0).unwrap();
    }
    let path = hamiltonian_path(&g, n[0]).unwrap();
    let unique: std::collections::HashSet<_> = path.iter().collect();
    assert_eq!(path.len(), 5);
    assert_eq!(unique.len(), 5);
}

#[test]
fn hamiltonian_path_none_when_impossible() {
    // Graph with isolated node: no Hamiltonian path exists from 0.
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let n: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
    g.add_edge(n[0], n[1], 1.0).unwrap();
    // n[2] is isolated — unreachable.
    assert!(hamiltonian_path(&g, n[0]).is_none());
}

// ── TSP Held-Karp ─────────────────────────────────────────────────────────────

#[test]
fn tsp_four_node_known_optimal() {
    //        0     1     2     3
    let dist = vec![
        vec![0.0, 10.0, 15.0, 20.0],
        vec![10.0, 0.0, 35.0, 25.0],
        vec![15.0, 35.0, 0.0, 30.0],
        vec![20.0, 25.0, 30.0, 0.0],
    ];
    let (cost, path) = tsp_held_karp(&dist).unwrap();
    assert!((cost - 80.0).abs() < 1e-9, "expected 80 got {cost}");
    assert_eq!(path.first(), Some(&0));
    assert_eq!(path.last(), Some(&0));
    assert_eq!(path.len(), 5);
}

#[test]
fn tsp_single_node() {
    let dist = vec![vec![0.0]];
    let (cost, path) = tsp_held_karp(&dist).unwrap();
    assert_eq!(cost, 0.0);
    assert_eq!(path, vec![0, 0]);
}

#[test]
fn tsp_two_nodes() {
    let dist = vec![vec![0.0, 5.0], vec![5.0, 0.0]];
    let (cost, path) = tsp_held_karp(&dist).unwrap();
    assert!((cost - 10.0).abs() < 1e-9);
    assert_eq!(path.first(), Some(&0));
    assert_eq!(path.last(), Some(&0));
}

#[test]
fn tsp_no_circuit_returns_none() {
    // 0→1 exists but 1→0 is INFINITY: no Hamiltonian circuit.
    let dist = vec![
        vec![0.0, 1.0, f64::INFINITY],
        vec![f64::INFINITY, 0.0, 1.0],
        vec![f64::INFINITY, f64::INFINITY, 0.0],
    ];
    assert!(tsp_held_karp(&dist).is_none());
}

#[test]
fn tsp_path_visits_all_nodes() {
    let dist = vec![
        vec![0.0, 1.0, 2.0, 3.0],
        vec![1.0, 0.0, 1.0, 2.0],
        vec![2.0, 1.0, 0.0, 1.0],
        vec![3.0, 2.0, 1.0, 0.0],
    ];
    let (_, path) = tsp_held_karp(&dist).unwrap();
    let interior: std::collections::HashSet<_> = path[1..path.len() - 1].iter().collect();
    assert_eq!(interior.len(), 3); // nodes 1, 2, 3 each appear once
}
