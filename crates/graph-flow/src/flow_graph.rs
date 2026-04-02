//! Index-based residual graph for flow algorithms.
//!
//! All flow algorithms in this crate operate on [`FlowGraph`] rather than the
//! generic [`Graph`] trait. See the [crate-level documentation](crate) for the
//! design rationale.
//!
//! [`Graph`]: graph_core::Graph

/// A single directed edge in a flow network.
///
/// Each edge stores its target node, capacity, current flow, and — crucially —
/// the index of its **reverse edge** in the adjacency list of the target node.
/// This reverse-index pattern is the standard safe Rust approach to residual
/// graph updates: instead of holding two mutable references simultaneously, we
/// use indices to locate and update both the forward and backward edges.
#[derive(Debug, Clone)]
pub struct FlowEdge {
    /// Destination node index.
    pub to: usize,
    /// Maximum capacity of this edge.
    pub capacity: f64,
    /// Current flow along this edge.
    pub flow: f64,
    /// Index of the reverse (residual) edge in `adjacency[to]`.
    ///
    /// When we send flow along edge `(u→v)`, we simultaneously increase the
    /// residual capacity of `(v→u)` by indexing: `adjacency[to][rev]`.
    pub rev: usize,
}

impl FlowEdge {
    /// Returns the **residual capacity** of this edge: how much more flow can
    /// be pushed along it.
    ///
    /// For a forward edge this is `capacity - flow`. For a reverse (residual)
    /// edge this equals the flow already sent on the corresponding forward
    /// edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_flow::flow_graph::FlowEdge;
    ///
    /// let edge = FlowEdge { to: 1, capacity: 10.0, flow: 3.0, rev: 0 };
    /// assert_eq!(edge.residual(), 7.0);
    /// ```
    #[inline]
    pub fn residual(&self) -> f64 {
        self.capacity - self.flow
    }
}

/// An index-based directed flow network supporting residual graph operations.
///
/// Nodes are identified by `usize` indices `0..n`. Call [`add_edge`] to add a
/// directed edge; the reverse residual edge is inserted automatically.
///
/// # Examples
///
/// ```
/// use graph_flow::FlowGraph;
///
/// let mut g = FlowGraph::new(4);
/// g.add_edge(0, 1, 10.0);
/// g.add_edge(0, 2, 5.0);
/// g.add_edge(1, 3, 10.0);
/// g.add_edge(2, 3, 5.0);
///
/// // Node count is what we specified.
/// assert_eq!(g.node_count(), 4);
/// // Each add_edge inserts 2 entries (forward + reverse).
/// assert_eq!(g.adjacency[0].len(), 2);
/// ```
///
/// [`add_edge`]: FlowGraph::add_edge
#[derive(Debug, Clone)]
pub struct FlowGraph {
    /// Adjacency list: `adjacency[u]` is the list of edges leaving node `u`.
    ///
    /// Both real (forward) edges and zero-capacity reverse (residual) edges
    /// are stored here. Use [`FlowEdge::residual`] to check if an edge can
    /// carry more flow.
    pub adjacency: Vec<Vec<FlowEdge>>,
}

impl FlowGraph {
    /// Creates a new flow graph with `n` nodes and no edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_flow::FlowGraph;
    ///
    /// let g = FlowGraph::new(5);
    /// assert_eq!(g.node_count(), 5);
    /// ```
    pub fn new(n: usize) -> Self {
        Self {
            adjacency: vec![Vec::new(); n],
        }
    }

    /// Returns the number of nodes in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_flow::FlowGraph;
    ///
    /// let g = FlowGraph::new(3);
    /// assert_eq!(g.node_count(), 3);
    /// ```
    #[inline]
    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Adds a directed edge from `u` to `v` with the given `capacity`, and
    /// automatically inserts the corresponding zero-capacity reverse edge
    /// from `v` to `u`.
    ///
    /// The reverse edge index is stored in each [`FlowEdge::rev`] field so
    /// that augmenting flow along `(u→v)` can update the residual `(v→u)`
    /// in O(1) using only safe index arithmetic.
    ///
    /// # Panics
    ///
    /// Panics if `u` or `v` is out of bounds (`>= node_count()`).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_flow::FlowGraph;
    ///
    /// let mut g = FlowGraph::new(2);
    /// g.add_edge(0, 1, 15.0);
    ///
    /// // Forward edge from 0 to 1.
    /// assert_eq!(g.adjacency[0][0].to, 1);
    /// assert_eq!(g.adjacency[0][0].capacity, 15.0);
    ///
    /// // Reverse edge from 1 to 0 (zero capacity).
    /// assert_eq!(g.adjacency[1][0].to, 0);
    /// assert_eq!(g.adjacency[1][0].capacity, 0.0);
    /// ```
    pub fn add_edge(&mut self, u: usize, v: usize, capacity: f64) {
        // Index of the forward edge in adjacency[u].
        let forward_idx = self.adjacency[u].len();
        // Index of the reverse edge in adjacency[v].
        let reverse_idx = self.adjacency[v].len();

        self.adjacency[u].push(FlowEdge {
            to: v,
            capacity,
            flow: 0.0,
            rev: reverse_idx,
        });
        self.adjacency[v].push(FlowEdge {
            to: u,
            capacity: 0.0, // reverse edge starts with zero capacity
            flow: 0.0,
            rev: forward_idx,
        });
    }

    /// Sends `amount` of flow along the edge at `adjacency[u][edge_idx]` and
    /// simultaneously updates the corresponding reverse edge.
    ///
    /// This is the primitive used by augmenting-path algorithms after
    /// identifying the bottleneck capacity of a path.
    ///
    /// # Panics
    ///
    /// Panics if `u` or `edge_idx` is out of bounds.
    pub fn push_flow(&mut self, u: usize, edge_idx: usize, amount: f64) {
        let rev_node = self.adjacency[u][edge_idx].to;
        let rev_idx = self.adjacency[u][edge_idx].rev;

        self.adjacency[u][edge_idx].flow += amount;
        self.adjacency[rev_node][rev_idx].flow -= amount;
    }

    /// Resets all flow values to zero, restoring the graph to its initial
    /// state with full capacity on every forward edge.
    ///
    /// Useful for running multiple flow algorithms on the same graph without
    /// rebuilding it from scratch.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_flow::{FlowGraph, ford_fulkerson};
    ///
    /// let mut g = FlowGraph::new(2);
    /// g.add_edge(0, 1, 10.0);
    ///
    /// let flow = ford_fulkerson(&mut g, 0, 1);
    /// assert_eq!(flow, 10.0);
    ///
    /// g.reset_flow();
    /// assert_eq!(g.adjacency[0][0].flow, 0.0);
    /// ```
    pub fn reset_flow(&mut self) {
        for adj in &mut self.adjacency {
            for edge in adj {
                edge.flow = 0.0;
            }
        }
    }
}
