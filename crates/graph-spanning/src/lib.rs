//! # graph-spanning
//!
//! Minimum spanning tree and connectivity algorithms built on [`graph_core`]'s
//! [`Graph`] trait.
//!
//! ## Algorithms
//!
//! | Module               | Algorithm                               | Complexity     |
//! |----------------------|-----------------------------------------|----------------|
//! | [`mod@disjoint_set`]     | Union-Find with path compression        | O(α(n))        |
//! | [`mod@kruskal`]          | Kruskal's MST                           | O(E log E)     |
//! | [`mod@prim`]             | Prim's MST                              | O((V+E) log V) |
//! | [`mod@bridges`]          | Tarjan's bridge finding                 | O(V+E)         |
//! | [`mod@articulation`]     | Articulation points (cut vertices)      | O(V+E)         |
//!
//! [`Graph`]: graph_core::Graph

/// Articulation points (cut vertices) via DFS disc/low-link values.
pub mod articulation;
/// Bridge finding (cut edges) via Tarjan's DFS disc/low-link algorithm.
pub mod bridges;
/// Union-Find with union-by-rank and path compression.
pub mod disjoint_set;
/// Kruskal's minimum spanning tree: sort edges, union-find cycle check.
pub mod kruskal;
/// Prim's minimum spanning tree: priority-queue edge relaxation.
pub mod prim;

pub use articulation::articulation_points;
pub use bridges::bridges;
pub use disjoint_set::DisjointSet;
pub use kruskal::{kruskal, SpanningTree};
pub use prim::prim;
