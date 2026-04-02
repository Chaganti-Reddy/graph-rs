use crate::{Graph, GraphError, NodeId};

/// A dense graph representation backed by a 2-D adjacency matrix.
///
/// Edge existence and weight are stored in a `Vec<Vec<Option<W>>>` matrix
/// where `matrix[i][j]` is `Some(weight)` if there is an edge from `i` to
/// `j`, or `None` otherwise.
///
/// # When to prefer this over [`AdjacencyList`](crate::AdjacencyList)
///
/// | Property     | AdjacencyList | AdjacencyMatrix |
/// |--------------|---------------|-----------------|
/// | Space        | O(V + E)      | O(V²)           |
/// | `add_edge`   | O(1)          | O(1)            |
/// | `contains_edge` | O(deg)   | **O(1)**        |
/// | `neighbors`  | O(deg)        | O(V)            |
///
/// Use `AdjacencyMatrix` for dense graphs (many edges relative to V²) or
/// when O(1) edge lookup is critical (e.g. Floyd-Warshall).
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyMatrix, Graph};
///
/// let mut g: AdjacencyMatrix<&str, u32> = AdjacencyMatrix::directed();
/// let a = g.add_node("A");
/// let b = g.add_node("B");
/// g.add_edge(a, b, 5).unwrap();
///
/// assert!(g.contains_edge(a, b));
/// assert!(!g.contains_edge(b, a));
/// ```
#[derive(Debug, Clone)]
pub struct AdjacencyMatrix<N, W = f64> {
    nodes: Vec<N>,
    /// `matrix[i][j]` = `Some(weight)` for edge i→j, `None` for no edge.
    matrix: Vec<Vec<Option<W>>>,
    edge_count: usize,
    directed: bool,
}

// ── Construction ──────────────────────────────────────────────────────────────

impl<N, W> AdjacencyMatrix<N, W> {
    /// Creates an empty **directed** adjacency-matrix graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyMatrix, Graph};
    ///
    /// let g: AdjacencyMatrix<()> = AdjacencyMatrix::directed();
    /// assert!(g.is_empty());
    /// ```
    pub fn directed() -> Self {
        Self {
            nodes: Vec::new(),
            matrix: Vec::new(),
            edge_count: 0,
            directed: true,
        }
    }

    /// Creates an empty **undirected** adjacency-matrix graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyMatrix, Graph};
    ///
    /// let mut g: AdjacencyMatrix<()> = AdjacencyMatrix::undirected();
    /// let u = g.add_node(());
    /// let v = g.add_node(());
    /// g.add_edge(u, v, 1.0).unwrap();
    /// assert!(g.contains_edge(v, u)); // symmetric
    /// ```
    pub fn undirected() -> Self {
        Self {
            nodes: Vec::new(),
            matrix: Vec::new(),
            edge_count: 0,
            directed: false,
        }
    }

    /// Returns `true` if this graph is directed.
    #[must_use]
    #[inline]
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Returns a reference to the weight of edge `from → to`, or `None` if
    /// no such edge exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyMatrix, Graph};
    ///
    /// let mut g: AdjacencyMatrix<()> = AdjacencyMatrix::directed();
    /// let a = g.add_node(());
    /// let b = g.add_node(());
    /// g.add_edge(a, b, 3.0).unwrap();
    /// assert_eq!(g.edge_weight(a, b), Some(&3.0));
    /// assert_eq!(g.edge_weight(b, a), None);
    /// ```
    #[must_use]
    #[inline]
    pub fn edge_weight(&self, from: NodeId, to: NodeId) -> Option<&W> {
        self.matrix.get(from.index())?.get(to.index())?.as_ref()
    }
}

// ── Graph trait ───────────────────────────────────────────────────────────────

/// Iterator over all `NodeId`s in an [`AdjacencyMatrix`].
pub struct NodeIter {
    current: usize,
    total: usize,
}

impl Iterator for NodeIter {
    type Item = NodeId;

    #[inline]
    fn next(&mut self) -> Option<NodeId> {
        if self.current < self.total {
            let id = NodeId::new(self.current);
            self.current += 1;
            Some(id)
        } else {
            None
        }
    }
}

/// Iterator over `(NodeId, &W)` neighbour pairs for one row of the matrix.
pub struct NeighborIter<'a, W> {
    row: &'a [Option<W>],
    current: usize,
}

impl<'a, W> Iterator for NeighborIter<'a, W> {
    type Item = (NodeId, &'a W);

    fn next(&mut self) -> Option<(NodeId, &'a W)> {
        while self.current < self.row.len() {
            let idx = self.current;
            self.current += 1;
            if let Some(ref w) = self.row[idx] {
                return Some((NodeId::new(idx), w));
            }
        }
        None
    }
}

impl<N, W: Clone> Graph for AdjacencyMatrix<N, W> {
    type NodeData = N;
    type Weight = W;
    type NodeIter<'a>
        = NodeIter
    where
        Self: 'a;
    type NeighborIter<'a>
        = NeighborIter<'a, W>
    where
        Self: 'a;

    fn add_node(&mut self, data: N) -> NodeId {
        let idx = self.nodes.len();
        self.nodes.push(data);
        // Extend existing rows with a new `None` column.
        for row in &mut self.matrix {
            row.push(None);
        }
        // Add a new row of all `None`.
        self.matrix.push(vec![None; idx + 1]);
        NodeId::new(idx)
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId, weight: W) -> Result<(), GraphError> {
        if !self.contains_node(from) {
            return Err(GraphError::NodeNotFound(from));
        }
        if !self.contains_node(to) {
            return Err(GraphError::NodeNotFound(to));
        }
        self.matrix[from.index()][to.index()] = Some(weight.clone());
        if !self.directed && from != to {
            self.matrix[to.index()][from.index()] = Some(weight);
        }
        self.edge_count += 1;
        Ok(())
    }

    fn remove_node(&mut self, id: NodeId) -> Option<N> {
        let idx = id.index();
        if idx >= self.nodes.len() {
            return None;
        }
        // Remove the row.
        self.matrix.remove(idx);
        // Remove the column from every remaining row.
        for row in &mut self.matrix {
            row.remove(idx);
        }
        // Recount edges (simple but correct).
        self.edge_count = self
            .matrix
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_some())
            .count();
        Some(self.nodes.remove(idx))
    }

    #[inline]
    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    fn edge_count(&self) -> usize {
        self.edge_count
    }

    #[inline]
    fn contains_node(&self, id: NodeId) -> bool {
        id.index() < self.nodes.len()
    }

    #[inline]
    fn contains_edge(&self, from: NodeId, to: NodeId) -> bool {
        self.matrix
            .get(from.index())
            .and_then(|row| row.get(to.index()))
            .map(|cell| cell.is_some())
            .unwrap_or(false)
    }

    fn degree(&self, id: NodeId) -> usize {
        self.matrix[id.index()]
            .iter()
            .filter(|cell| cell.is_some())
            .count()
    }

    fn nodes(&self) -> NodeIter {
        NodeIter {
            current: 0,
            total: self.nodes.len(),
        }
    }

    fn neighbors(&self, id: NodeId) -> NeighborIter<'_, W> {
        NeighborIter {
            row: &self.matrix[id.index()],
            current: 0,
        }
    }
}
