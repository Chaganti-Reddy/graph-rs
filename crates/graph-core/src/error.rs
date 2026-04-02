use crate::NodeId;

/// Errors that can arise from graph operations.
///
/// Methods that mutate or query a graph return `Result<T, GraphError>`
/// whenever the operation could meaningfully fail (e.g. referencing a
/// node that does not exist).
///
/// # Examples
///
/// ```
/// use graph_core::{GraphError, NodeId};
///
/// let err = GraphError::NodeNotFound(NodeId::new(99));
/// assert!(matches!(err, GraphError::NodeNotFound(_)));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum GraphError {
    /// The referenced node does not exist in the graph.
    NodeNotFound(NodeId),
    /// An edge between these two nodes already exists.
    EdgeAlreadyExists(NodeId, NodeId),
    /// The operation would create a self-loop, which is disallowed.
    SelfLoop(NodeId),
    /// A negative-weight cycle was detected (Bellman-Ford / Floyd-Warshall).
    NegativeCycle,
    /// The graph is not connected where connectivity is required (e.g. MST).
    NotConnected,
    /// A catch-all for operations that are invalid in the current context.
    InvalidOperation(&'static str),
}

// ── Display / Error ───────────────────────────────────────────────────────────

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(id) => write!(f, "node not found: {id}"),
            GraphError::EdgeAlreadyExists(u, v) => {
                write!(f, "edge already exists: {u} -> {v}")
            }
            GraphError::SelfLoop(id) => write!(f, "self-loop not allowed: {id}"),
            GraphError::NegativeCycle => write!(f, "negative-weight cycle detected"),
            GraphError::NotConnected => write!(f, "graph is not connected"),
            GraphError::InvalidOperation(msg) => write!(f, "invalid operation: {msg}"),
        }
    }
}

impl std::error::Error for GraphError {}
