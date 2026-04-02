/// A type-safe node identifier.
///
/// Wraps a `usize` index so a [`NodeId`] can never be accidentally passed
/// where an [`EdgeId`] (or a raw `usize`) is expected. The wrapper is
/// zero-cost — the compiler erases it entirely.
///
/// # Examples
///
/// ```
/// use graph_core::NodeId;
///
/// let a = NodeId::new(0);
/// let b = NodeId::new(1);
/// assert_ne!(a, b);
/// assert_eq!(a.index(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(usize);

/// A type-safe edge identifier.
///
/// Wraps a `usize` index so an [`EdgeId`] can never be accidentally passed
/// where a [`NodeId`] is expected.
///
/// # Examples
///
/// ```
/// use graph_core::EdgeId;
///
/// let e = EdgeId::new(3);
/// assert_eq!(e.index(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeId(usize);

// ── NodeId ────────────────────────────────────────────────────────────────────

impl NodeId {
    /// Creates a new [`NodeId`] from a raw index.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::NodeId;
    ///
    /// let id = NodeId::new(42);
    /// assert_eq!(id.index(), 42);
    /// ```
    #[inline]
    pub fn new(idx: usize) -> Self {
        NodeId(idx)
    }

    /// Returns the underlying `usize` index.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::NodeId;
    ///
    /// assert_eq!(NodeId::new(7).index(), 7);
    /// ```
    #[must_use]
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for NodeId {
    /// Converts a `usize` into a [`NodeId`].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::NodeId;
    ///
    /// let id: NodeId = 5usize.into();
    /// assert_eq!(id.index(), 5);
    /// ```
    #[inline]
    fn from(i: usize) -> Self {
        NodeId(i)
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({})", self.0)
    }
}

// ── EdgeId ────────────────────────────────────────────────────────────────────

impl EdgeId {
    /// Creates a new [`EdgeId`] from a raw index.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::EdgeId;
    ///
    /// let id = EdgeId::new(0);
    /// assert_eq!(id.index(), 0);
    /// ```
    #[inline]
    pub fn new(idx: usize) -> Self {
        EdgeId(idx)
    }

    /// Returns the underlying `usize` index.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::EdgeId;
    ///
    /// assert_eq!(EdgeId::new(2).index(), 2);
    /// ```
    #[must_use]
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for EdgeId {
    /// Converts a `usize` into an [`EdgeId`].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_core::EdgeId;
    ///
    /// let id: EdgeId = 9usize.into();
    /// assert_eq!(id.index(), 9);
    /// ```
    #[inline]
    fn from(i: usize) -> Self {
        EdgeId(i)
    }
}

impl std::fmt::Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EdgeId({})", self.0)
    }
}
