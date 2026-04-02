use crate::{Edge, NodeId};

/// The central graph abstraction that every algorithm in graph-rs operates on.
///
/// `N` is the node-data type and `W` is the edge-weight type.
///
/// Implementors provide two associated iterator types — `NodeIter<'a>` and
/// `NeighborIter<'a>` — that borrow from `&self`. These are **Generic
/// Associated Types** (GATs, stable since Rust 1.65): associated types that
/// are themselves generic over a lifetime, enabling zero-allocation iterators
/// that reference graph internals directly.
///
/// # Implementing `Graph`
///
/// ```ignore
/// use graph_core::{Graph, NodeId, Edge};
///
/// struct MyGraph { /* ... */ }
///
/// impl Graph for MyGraph {
///     type NodeData = String;
///     type Weight   = f64;
///     // ... (see AdjacencyList for a complete example)
/// }
/// ```
pub trait Graph {
    /// Data stored at each node (e.g. `String`, `()`, coordinates).
    type NodeData;

    /// Edge-weight type (e.g. `f64`, `u32`, `()`).
    type Weight;

    /// Iterator over all [`NodeId`]s in the graph.
    type NodeIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Iterator over `(neighbour_id, &weight)` pairs for one node.
    type NeighborIter<'a>: Iterator<Item = (NodeId, &'a Self::Weight)>
    where
        Self: 'a;

    // ── Mutation ──────────────────────────────────────────────────────────────

    /// Adds a node with the given data and returns its [`NodeId`].
    fn add_node(&mut self, data: Self::NodeData) -> NodeId;

    /// Adds a directed edge from `from` to `to` with `weight`.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError`](crate::GraphError) if either node does not exist.
    fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        weight: Self::Weight,
    ) -> Result<(), crate::GraphError>;

    /// Removes the node at `id` and all edges incident to it.
    ///
    /// Returns the node data if found, or `None` if the node did not exist.
    fn remove_node(&mut self, id: NodeId) -> Option<Self::NodeData>;

    // ── Query ─────────────────────────────────────────────────────────────────

    /// Returns the number of nodes.
    fn node_count(&self) -> usize;

    /// Returns the number of directed edges.
    fn edge_count(&self) -> usize;

    /// Returns `true` if the graph contains a node with this id.
    fn contains_node(&self, id: NodeId) -> bool;

    /// Returns `true` if there is a directed edge from `from` to `to`.
    fn contains_edge(&self, from: NodeId, to: NodeId) -> bool;

    /// Returns the out-degree of node `id` (number of outgoing edges).
    ///
    /// # Panics
    ///
    /// Panics if `id` does not exist in the graph.
    fn degree(&self, id: NodeId) -> usize;

    // ── Iteration ─────────────────────────────────────────────────────────────

    /// Returns an iterator over all [`NodeId`]s in the graph.
    fn nodes(&self) -> Self::NodeIter<'_>;

    /// Returns an iterator over `(neighbour_id, &weight)` for all outgoing
    /// edges of `id`.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not exist in the graph.
    fn neighbors(&self, id: NodeId) -> Self::NeighborIter<'_>;

    // ── Provided helpers ──────────────────────────────────────────────────────

    /// Returns all edges in the graph as [`Edge<W>`] values.
    ///
    /// Default implementation iterates over all nodes and their neighbours.
    /// Implementations may override this for efficiency.
    fn all_edges(&self) -> Vec<Edge<Self::Weight>>
    where
        Self::Weight: Clone,
    {
        let mut edges = Vec::new();
        for u in self.nodes() {
            for (v, w) in self.neighbors(u) {
                edges.push(Edge::new(u, v, w.clone()));
            }
        }
        edges
    }

    /// Returns `true` if the graph has no nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{AdjacencyList, Graph};
    ///
    /// let g: AdjacencyList<()> = AdjacencyList::directed();
    /// assert!(g.is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        self.node_count() == 0
    }
}
