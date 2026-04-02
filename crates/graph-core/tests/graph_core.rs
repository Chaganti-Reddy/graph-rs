use graph_core::{AdjacencyList, AdjacencyMatrix, Graph, GraphBuilder, GraphError, NodeId};

// ── AdjacencyList – directed ──────────────────────────────────────────────────

#[test]
fn adj_list_directed_empty_on_creation() {
    let g: AdjacencyList<()> = AdjacencyList::directed();
    assert!(g.is_empty());
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(g.is_directed());
}

#[test]
fn adj_list_add_nodes_increments_count() {
    let mut g: AdjacencyList<u32> = AdjacencyList::directed();
    let a = g.add_node(1);
    let b = g.add_node(2);
    assert_eq!(g.node_count(), 2);
    assert_eq!(a.index(), 0);
    assert_eq!(b.index(), 1);
}

#[test]
fn adj_list_add_edge_directed() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();

    assert_eq!(g.edge_count(), 1);
    assert!(g.contains_edge(a, b));
    assert!(!g.contains_edge(b, a)); // directed: reverse does not exist
}

#[test]
fn adj_list_add_edge_missing_node_returns_error() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let ghost = NodeId::new(99);

    assert!(matches!(
        g.add_edge(a, ghost, 1.0),
        Err(GraphError::NodeNotFound(_))
    ));
}

#[test]
fn adj_list_undirected_adds_both_directions() {
    let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    let u = g.add_node(());
    let v = g.add_node(());
    g.add_edge(u, v, 2.5).unwrap();

    assert!(g.contains_edge(u, v));
    assert!(g.contains_edge(v, u));
}

#[test]
fn adj_list_degree_counts_outgoing() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(a, c, 1.0).unwrap();

    assert_eq!(g.degree(a), 2);
    assert_eq!(g.degree(b), 0);
}

#[test]
fn adj_list_nodes_iter_covers_all() {
    let mut g: AdjacencyList<u32> = AdjacencyList::directed();
    for i in 0..5 {
        g.add_node(i);
    }
    let ids: Vec<NodeId> = g.nodes().collect();
    assert_eq!(ids.len(), 5);
    for (i, id) in ids.iter().enumerate() {
        assert_eq!(id.index(), i);
    }
}

#[test]
fn adj_list_neighbors_returns_correct_targets() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(a, c, 2.0).unwrap();

    let neighbors: Vec<NodeId> = g.neighbors(a).map(|(id, _)| id).collect();
    assert!(neighbors.contains(&b));
    assert!(neighbors.contains(&c));
    assert_eq!(neighbors.len(), 2);
}

#[test]
fn adj_list_neighbors_includes_weights() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, 3.7).unwrap();

    let (_, w) = g.neighbors(a).next().unwrap();
    assert!((w - 3.7_f64).abs() < f64::EPSILON);
}

#[test]
fn adj_list_all_edges_correct_count() {
    let mut g: AdjacencyList<()> = AdjacencyList::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, b, 1.0).unwrap();
    g.add_edge(b, c, 2.0).unwrap();
    g.add_edge(a, c, 3.0).unwrap();

    assert_eq!(g.all_edges().len(), 3);
}

#[test]
fn adj_list_node_data_access() {
    let mut g: AdjacencyList<String> = AdjacencyList::directed();
    let id = g.add_node("hello".to_string());
    assert_eq!(g.node_data(id), Some(&"hello".to_string()));
}

#[test]
fn adj_list_node_data_mut_updates_value() {
    let mut g: AdjacencyList<u32> = AdjacencyList::directed();
    let id = g.add_node(0u32);
    *g.node_data_mut(id).unwrap() = 99;
    assert_eq!(g.node_data(id), Some(&99));
}

// ── AdjacencyMatrix ───────────────────────────────────────────────────────────

#[test]
fn adj_matrix_directed_empty_on_creation() {
    let g: AdjacencyMatrix<()> = AdjacencyMatrix::directed();
    assert!(g.is_empty());
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn adj_matrix_add_edge_o1_lookup() {
    let mut g: AdjacencyMatrix<()> = AdjacencyMatrix::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    g.add_edge(a, b, 7.0f64).unwrap();

    assert!(g.contains_edge(a, b));
    assert!(!g.contains_edge(b, a));
    assert_eq!(g.edge_weight(a, b), Some(&7.0));
}

#[test]
fn adj_matrix_undirected_symmetric() {
    let mut g: AdjacencyMatrix<()> = AdjacencyMatrix::undirected();
    let u = g.add_node(());
    let v = g.add_node(());
    g.add_edge(u, v, 1.0).unwrap();

    assert!(g.contains_edge(u, v));
    assert!(g.contains_edge(v, u));
}

#[test]
fn adj_matrix_missing_node_error() {
    let mut g: AdjacencyMatrix<()> = AdjacencyMatrix::directed();
    let a = g.add_node(());
    let ghost = NodeId::new(99);
    assert!(g.add_edge(a, ghost, 1.0).is_err());
}

#[test]
fn adj_matrix_neighbors_only_present_edges() {
    let mut g: AdjacencyMatrix<()> = AdjacencyMatrix::directed();
    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    g.add_edge(a, c, 1.0).unwrap();

    let neighbors: Vec<NodeId> = g.neighbors(a).map(|(id, _)| id).collect();
    assert_eq!(neighbors, vec![c]);
    assert!(!neighbors.contains(&b));
}

// ── GraphBuilder ──────────────────────────────────────────────────────────────

#[test]
fn builder_directed_adjacency_list() {
    let g = GraphBuilder::<&str, f64>::new()
        .directed()
        .node("A")
        .node("B")
        .node("C")
        .edge(0, 1, 1.0)
        .edge(1, 2, 2.0)
        .build_adjacency_list();

    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn builder_undirected_adjacency_list() {
    let g = GraphBuilder::<(), f64>::new()
        .undirected()
        .node(())
        .node(())
        .edge(0, 1, 1.0)
        .build_adjacency_list();

    assert!(g.contains_edge(0usize.into(), 1usize.into()));
    assert!(g.contains_edge(1usize.into(), 0usize.into()));
}

#[test]
fn builder_adjacency_matrix() {
    let g = GraphBuilder::<&str, f64>::new()
        .node("X")
        .node("Y")
        .edge(0, 1, 9.0)
        .build_adjacency_matrix();

    assert!(g.contains_edge(0usize.into(), 1usize.into()));
}

// ── GraphError ────────────────────────────────────────────────────────────────

#[test]
fn graph_error_display_node_not_found() {
    let err = GraphError::NodeNotFound(NodeId::new(5));
    let msg = err.to_string();
    assert!(msg.contains("5"));
}

#[test]
fn graph_error_display_negative_cycle() {
    let err = GraphError::NegativeCycle;
    assert!(err.to_string().contains("negative"));
}

// ── NodeId / EdgeId ───────────────────────────────────────────────────────────

#[test]
fn node_id_ordering() {
    let a = NodeId::new(0);
    let b = NodeId::new(1);
    assert!(a < b);
}

#[test]
fn node_id_from_usize() {
    let id: NodeId = 7usize.into();
    assert_eq!(id.index(), 7);
}
