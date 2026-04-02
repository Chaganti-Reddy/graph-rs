use crate::{AdjacencyList, AdjacencyMatrix, Graph, NodeId};

/// A fluent builder for constructing graphs from either representation.
///
/// Call [`GraphBuilder::new`], chain configuration and node/edge additions,
/// then finalise with [`build_adjacency_list`](GraphBuilder::build_adjacency_list)
/// or [`build_adjacency_matrix`](GraphBuilder::build_adjacency_matrix).
///
/// # Examples
///
/// ```
/// use graph_core::{GraphBuilder, Graph};
///
/// let g = GraphBuilder::<&str, f64>::new()
///     .directed()
///     .node("A")
///     .node("B")
///     .node("C")
///     .edge(0, 1, 1.5)
///     .edge(1, 2, 2.0)
///     .build_adjacency_list();
///
/// assert_eq!(g.node_count(), 3);
/// assert_eq!(g.edge_count(), 2);
/// ```
#[derive(Debug)]
pub struct GraphBuilder<N, W = f64> {
    nodes: Vec<N>,
    edges: Vec<(usize, usize, W)>,
    directed: bool,
}

// ── Construction ──────────────────────────────────────────────────────────────

impl<N, W> GraphBuilder<N, W> {
    /// Creates a new builder with no nodes or edges (directed by default).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::GraphBuilder;
    ///
    /// let b: GraphBuilder<(), f64> = GraphBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            directed: true,
        }
    }
}

impl<N, W> Default for GraphBuilder<N, W> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Configuration ─────────────────────────────────────────────────────────────

impl<N, W> GraphBuilder<N, W> {
    /// Configures the graph to be **directed** (default).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::GraphBuilder;
    ///
    /// let b: GraphBuilder<(), f64> = GraphBuilder::new().directed();
    /// ```
    #[inline]
    pub fn directed(mut self) -> Self {
        self.directed = true;
        self
    }

    /// Configures the graph to be **undirected**.
    ///
    /// Each call to [`edge`](Self::edge) will insert edges in both directions.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{GraphBuilder, Graph};
    ///
    /// let g = GraphBuilder::<(), f64>::new()
    ///     .undirected()
    ///     .node(())
    ///     .node(())
    ///     .edge(0, 1, 1.0)
    ///     .build_adjacency_list();
    ///
    /// assert!(g.contains_edge(0usize.into(), 1usize.into()));
    /// assert!(g.contains_edge(1usize.into(), 0usize.into()));
    /// ```
    #[inline]
    pub fn undirected(mut self) -> Self {
        self.directed = false;
        self
    }
}

// ── Node / edge accumulation ──────────────────────────────────────────────────

impl<N, W> GraphBuilder<N, W> {
    /// Adds a node with the given data and returns `self` for chaining.
    ///
    /// Nodes are assigned indices in insertion order (0, 1, 2 …).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::GraphBuilder;
    ///
    /// let b = GraphBuilder::<&str, f64>::new()
    ///     .node("X")
    ///     .node("Y");
    /// ```
    pub fn node(mut self, data: N) -> Self {
        self.nodes.push(data);
        self
    }

    /// Adds an edge from `from` to `to` with `weight` and returns `self`.
    ///
    /// Node indices are the order they were added via [`node`](Self::node).
    /// Invalid indices will panic at build time.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::GraphBuilder;
    ///
    /// let b = GraphBuilder::<(), f64>::new()
    ///     .node(())
    ///     .node(())
    ///     .edge(0, 1, 3.0);
    /// ```
    pub fn edge(mut self, from: usize, to: usize, weight: W) -> Self {
        self.edges.push((from, to, weight));
        self
    }
}

// ── Build ─────────────────────────────────────────────────────────────────────

impl<N, W: Clone> GraphBuilder<N, W> {
    /// Consumes the builder and produces an [`AdjacencyList`].
    ///
    /// # Panics
    ///
    /// Panics if any edge references a node index that is out of range.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{GraphBuilder, Graph};
    ///
    /// let g = GraphBuilder::<&str, f64>::new()
    ///     .node("A")
    ///     .node("B")
    ///     .edge(0, 1, 5.0)
    ///     .build_adjacency_list();
    ///
    /// assert_eq!(g.edge_count(), 1);
    /// ```
    pub fn build_adjacency_list(self) -> AdjacencyList<N, W> {
        let mut g = if self.directed {
            AdjacencyList::directed()
        } else {
            AdjacencyList::undirected()
        };
        let mut ids: Vec<NodeId> = Vec::with_capacity(self.nodes.len());
        for data in self.nodes {
            ids.push(g.add_node(data));
        }
        for (from, to, weight) in self.edges {
            g.add_edge(ids[from], ids[to], weight)
                .expect("builder edge references valid nodes");
        }
        g
    }

    /// Consumes the builder and produces an [`AdjacencyMatrix`].
    ///
    /// # Panics
    ///
    /// Panics if any edge references a node index that is out of range.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{GraphBuilder, Graph};
    ///
    /// let g = GraphBuilder::<&str, f64>::new()
    ///     .node("A")
    ///     .node("B")
    ///     .edge(0, 1, 2.0)
    ///     .build_adjacency_matrix();
    ///
    /// assert!(g.contains_edge(0usize.into(), 1usize.into()));
    /// ```
    pub fn build_adjacency_matrix(self) -> AdjacencyMatrix<N, W> {
        let mut g = if self.directed {
            AdjacencyMatrix::directed()
        } else {
            AdjacencyMatrix::undirected()
        };
        let mut ids: Vec<NodeId> = Vec::with_capacity(self.nodes.len());
        for data in self.nodes {
            ids.push(g.add_node(data));
        }
        for (from, to, weight) in self.edges {
            g.add_edge(ids[from], ids[to], weight)
                .expect("builder edge references valid nodes");
        }
        g
    }
}
