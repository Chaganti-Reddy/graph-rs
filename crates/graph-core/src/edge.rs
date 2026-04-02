use crate::NodeId;

/// A directed edge from `source` to `target` carrying a weight of type `W`.
///
/// For unweighted graphs use `W = ()` (the default). For weighted graphs use
/// any numeric type, most commonly `f64`.
///
/// # Examples
///
/// ```
/// use graph_core::{Edge, NodeId};
///
/// let u = NodeId::new(0);
/// let v = NodeId::new(1);
///
/// // Weighted edge
/// let weighted = Edge::new(u, v, 3.5_f64);
/// assert_eq!(weighted.weight, 3.5);
///
/// // Unweighted edge (weight = ())
/// let unweighted: Edge<()> = Edge::new(u, v, ());
/// assert_eq!(unweighted.source, u);
/// assert_eq!(unweighted.target, v);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Edge<W = ()> {
    /// The node this edge originates from.
    pub source: NodeId,
    /// The node this edge points to.
    pub target: NodeId,
    /// The edge weight. Use `()` for unweighted graphs.
    pub weight: W,
}

// ── Construction ──────────────────────────────────────────────────────────────

impl<W> Edge<W> {
    /// Creates a new directed edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{Edge, NodeId};
    ///
    /// let e = Edge::new(NodeId::new(0), NodeId::new(1), 2.0_f64);
    /// assert_eq!(e.weight, 2.0);
    /// ```
    #[inline]
    pub fn new(source: NodeId, target: NodeId, weight: W) -> Self {
        Edge {
            source,
            target,
            weight,
        }
    }

    /// Returns `true` if this is a self-loop (source == target).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{Edge, NodeId};
    ///
    /// let looped = Edge::new(NodeId::new(3), NodeId::new(3), ());
    /// assert!(looped.is_self_loop());
    ///
    /// let normal = Edge::new(NodeId::new(0), NodeId::new(1), ());
    /// assert!(!normal.is_self_loop());
    /// ```
    #[must_use]
    #[inline]
    pub fn is_self_loop(&self) -> bool {
        self.source == self.target
    }

    /// Returns the reversed edge (source and target swapped, weight unchanged).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::{Edge, NodeId};
    ///
    /// let e = Edge::new(NodeId::new(0), NodeId::new(1), 5u32);
    /// let r = e.reversed();
    /// assert_eq!(r.source, NodeId::new(1));
    /// assert_eq!(r.target, NodeId::new(0));
    /// assert_eq!(r.weight, 5);
    /// ```
    #[must_use]
    pub fn reversed(self) -> Self
    where
        W: Clone,
    {
        Edge {
            source: self.target,
            target: self.source,
            weight: self.weight,
        }
    }
}

// ── Type aliases ──────────────────────────────────────────────────────────────

/// A weighted directed edge with `f64` weights.
///
/// # Examples
///
/// ```
/// use graph_core::{WeightedEdge, NodeId};
///
/// let e = WeightedEdge::new(NodeId::new(0), NodeId::new(2), 1.5);
/// assert_eq!(e.weight, 1.5);
/// ```
pub type WeightedEdge = Edge<f64>;

/// An unweighted directed edge (weight type `()`).
///
/// # Examples
///
/// ```
/// use graph_core::{UnweightedEdge, NodeId};
///
/// let e = UnweightedEdge::new(NodeId::new(0), NodeId::new(1), ());
/// assert_eq!(e.source, NodeId::new(0));
/// ```
pub type UnweightedEdge = Edge<()>;
