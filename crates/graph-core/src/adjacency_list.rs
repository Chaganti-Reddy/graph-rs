use crate::{Graph, GraphError, NodeId};
use std::slice;

/// A sparse graph representation backed by an adjacency list.
///
/// Node data is stored in a `Vec<N>`. Outgoing edges for each node are stored
/// in a `Vec<Vec<(NodeId, W)>>` indexed by node position.
///
/// # Directed vs Undirected
///
/// Use [`AdjacencyList::directed`] or [`AdjacencyList::undirected`].
/// For an undirected graph, `add_edge(u, v, w)` inserts both `v` into `u`'s
/// list and `u` into `v`'s list automatically.
///
/// # Complexity
///
/// | Operation       | Time            |
/// |-----------------|-----------------|
/// | `add_node`      | O(1) amortised  |
/// | `add_edge`      | O(1) amortised  |
/// | `neighbors`     | O(out-degree)   |
/// | `contains_edge` | O(out-degree)   |
/// | `node_count`    | O(1)            |
/// | `edge_count`    | O(1)            |
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
///
/// let mut g: AdjacencyList<&str, f64> = AdjacencyList::directed();
/// let a = g.add_node("A");
/// let b = g.add_node("B");
/// let c = g.add_node("C");
///
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 2.0).unwrap();
///
/// assert_eq!(g.node_count(), 3);
/// assert_eq!(g.edge_count(), 2);
/// assert!(g.contains_edge(a, b));
/// assert!(!g.contains_edge(c, a));
/// ```
#[derive(Debug, Clone)]
pub struct AdjacencyList<N, W = f64> {
    nodes: Vec<N>,
    /// `adjacency[i]` holds `(target, weight)` for every outgoing edge from node `i`.
    adjacency: Vec<Vec<(NodeId, W)>>,
    edge_count: usize,
    directed: bool,
}

// ── Construction ──────────────────────────────────────────────────────────────

impl<N, W> AdjacencyList<N, W> {
    /// Creates an empty **directed** adjacency-list graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyList, Graph};
    ///
    /// let g: AdjacencyList<()> = AdjacencyList::directed();
    /// assert!(g.is_empty());
    /// ```
    pub fn directed() -> Self {
        Self {
            nodes: Vec::new(),
            adjacency: Vec::new(),
            edge_count: 0,
            directed: true,
        }
    }

    /// Creates an empty **undirected** adjacency-list graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyList, Graph};
    ///
    /// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
    /// let u = g.add_node(());
    /// let v = g.add_node(());
    /// g.add_edge(u, v, 1.0).unwrap();
    ///
    /// // Both directions exist.
    /// assert!(g.contains_edge(u, v));
    /// assert!(g.contains_edge(v, u));
    /// ```
    pub fn undirected() -> Self {
        Self {
            nodes: Vec::new(),
            adjacency: Vec::new(),
            edge_count: 0,
            directed: false,
        }
    }

    /// Returns `true` if this graph is directed.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::AdjacencyList;
    ///
    /// let d: AdjacencyList<()> = AdjacencyList::directed();
    /// assert!(d.is_directed());
    ///
    /// let u: AdjacencyList<()> = AdjacencyList::undirected();
    /// assert!(!u.is_directed());
    /// ```
    #[must_use]
    #[inline]
    pub fn is_directed(&self) -> bool {
        self.directed
    }
}

// ── Node data access ──────────────────────────────────────────────────────────

impl<N, W> AdjacencyList<N, W> {
    /// Returns a reference to the data stored at `id`, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyList, Graph};
    ///
    /// let mut g: AdjacencyList<&str> = AdjacencyList::directed();
    /// let id = g.add_node("hello");
    /// assert_eq!(g.node_data(id), Some(&"hello"));
    /// ```
    #[must_use]
    #[inline]
    pub fn node_data(&self, id: NodeId) -> Option<&N> {
        self.nodes.get(id.index())
    }

    /// Returns a mutable reference to the data stored at `id`, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyList, Graph};
    ///
    /// let mut g: AdjacencyList<u32> = AdjacencyList::directed();
    /// let id = g.add_node(1);
    /// *g.node_data_mut(id).unwrap() = 42;
    /// assert_eq!(g.node_data(id), Some(&42));
    /// ```
    #[inline]
    pub fn node_data_mut(&mut self, id: NodeId) -> Option<&mut N> {
        self.nodes.get_mut(id.index())
    }
}

// ── Graph trait ───────────────────────────────────────────────────────────────

/// Iterator over all `NodeId`s in an [`AdjacencyList`].
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for NodeIter {}

/// Iterator over `(NodeId, &W)` neighbour pairs for one node.
pub struct NeighborIter<'a, W> {
    inner: slice::Iter<'a, (NodeId, W)>,
}

impl<'a, W> Iterator for NeighborIter<'a, W> {
    type Item = (NodeId, &'a W);

    #[inline]
    fn next(&mut self) -> Option<(NodeId, &'a W)> {
        self.inner.next().map(|(id, w)| (*id, w))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<N, W: Clone> Graph for AdjacencyList<N, W> {
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
        let id = NodeId::new(self.nodes.len());
        self.nodes.push(data);
        self.adjacency.push(Vec::new());
        id
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId, weight: W) -> Result<(), GraphError> {
        if !self.contains_node(from) {
            return Err(GraphError::NodeNotFound(from));
        }
        if !self.contains_node(to) {
            return Err(GraphError::NodeNotFound(to));
        }
        self.adjacency[from.index()].push((to, weight.clone()));
        if !self.directed && from != to {
            self.adjacency[to.index()].push((from, weight));
        }
        self.edge_count += 1;
        Ok(())
    }

    fn remove_node(&mut self, id: NodeId) -> Option<N> {
        let idx = id.index();
        if idx >= self.nodes.len() {
            return None;
        }
        // Remove edges pointing to this node from all adjacency lists.
        for adj in &mut self.adjacency {
            adj.retain(|(target, _)| *target != id);
        }
        // Count outgoing edges being removed.
        let outgoing = self.adjacency[idx].len();
        self.edge_count = self.edge_count.saturating_sub(outgoing);

        // We mark the slot as invalid by leaving a hole (swap-remove would
        // renumber later nodes, breaking existing NodeIds). Instead we remove
        // by replacing with a sentinel. For simplicity in this educational
        // implementation we use swap_remove and accept that NodeIds beyond
        // the removed index are invalidated — callers should rebuild if needed.
        self.adjacency.remove(idx);
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

    fn contains_edge(&self, from: NodeId, to: NodeId) -> bool {
        if !self.contains_node(from) {
            return false;
        }
        self.adjacency[from.index()]
            .iter()
            .any(|(target, _)| *target == to)
    }

    fn degree(&self, id: NodeId) -> usize {
        self.adjacency[id.index()].len()
    }

    fn nodes(&self) -> NodeIter {
        NodeIter {
            current: 0,
            total: self.nodes.len(),
        }
    }

    fn neighbors(&self, id: NodeId) -> NeighborIter<'_, W> {
        NeighborIter {
            inner: self.adjacency[id.index()].iter(),
        }
    }
}
