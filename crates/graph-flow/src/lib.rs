#![warn(missing_docs)]

//! # graph-flow
//!
//! Maximum flow and bipartite matching algorithms for graph-rs.
//!
//! Flow algorithms require a **residual graph** — a mutable internal
//! representation that tracks remaining capacity and sent flow on each edge.
//! Because this requires mutable aliasing patterns that conflict with Rust's
//! borrow checker when using the generic [`Graph`] trait directly, this crate
//! defines its own index-based [`FlowGraph`] structure for flow computation.
//!
//! ## Algorithms
//!
//! | Module              | Algorithm                              | Complexity      |
//! |---------------------|----------------------------------------|-----------------|
//! | [`mod@flow_graph`]      | Residual graph representation          | —               |
//! | [`mod@ford_fulkerson`]  | Ford-Fulkerson (DFS augmentation)      | O(E · max_flow) |
//! | [`mod@edmonds_karp`]    | Edmonds-Karp (BFS augmentation)        | O(V · E²)       |
//! | [`mod@min_cut`]         | Min-cut from a completed max-flow      | O(V + E)        |
//! | [`mod@hopcroft_karp`]   | Hopcroft-Karp bipartite matching       | O(E · √V)       |
//!
//! ## Design note: why a separate `FlowGraph`?
//!
//! The residual graph needs to update both the forward edge **and** its
//! corresponding reverse edge in a single pass. With Rust's borrow checker,
//! mutating two elements of the same `Vec` simultaneously requires either
//! index-based access (what we use here) or `unsafe`. Using indices is the
//! idiomatic safe solution: we store the index of each edge's reverse in the
//! adjacency list entry, giving O(1) residual updates with no aliasing.
//!
//! [`Graph`]: graph_core::Graph

/// Edmonds-Karp maximum flow using BFS augmenting paths.
pub mod edmonds_karp;
/// Index-based flow graph (residual graph) used by all flow algorithms.
pub mod flow_graph;
/// Ford-Fulkerson maximum flow using DFS augmenting paths.
pub mod ford_fulkerson;
/// Hopcroft-Karp maximum bipartite matching.
pub mod hopcroft_karp;
/// Minimum s-t cut extracted from a completed max-flow computation.
pub mod min_cut;

pub use edmonds_karp::edmonds_karp;
pub use flow_graph::FlowGraph;
pub use ford_fulkerson::ford_fulkerson;
pub use hopcroft_karp::hopcroft_karp;
pub use min_cut::{min_cut, MinCut};
