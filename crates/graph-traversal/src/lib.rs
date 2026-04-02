#![warn(missing_docs)]

//! # graph-traversal
//!
//! Traversal algorithms built on top of [`graph_core`]'s [`Graph`] trait.
//!
//! Every function accepts any type that implements `Graph`, so it works with
//! both [`AdjacencyList`] and [`AdjacencyMatrix`] out of the box.
//!
//! ## Algorithms
//!
//! | Module | Algorithms |
//! |--------|-----------|
//! | [`mod@dfs`] | Recursive DFS, iterative DFS, DFS tree |
//! | [`mod@bfs`] | BFS with distances, BFS tree |
//! | [`mod@topo`] | Topological sort (DFS-based and Kahn's) |
//! | [`mod@cycle`] | Cycle detection (directed and undirected) |
//! | [`mod@components`] | Connected components |
//! | [`mod@bipartite`] | Bipartite check and 2-colouring |
//! | [`mod@paths`] | Path reconstruction from parent maps |
//!
//! [`Graph`]: graph_core::Graph
//! [`AdjacencyList`]: graph_core::AdjacencyList
//! [`AdjacencyMatrix`]: graph_core::AdjacencyMatrix

/// Breadth-first search with distances and BFS tree.
pub mod bfs;
/// Bipartite check and 2-colouring via BFS.
pub mod bipartite;
/// Connected components via BFS seeding.
pub mod components;
/// Cycle detection for directed and undirected graphs.
pub mod cycle;
/// Recursive and iterative depth-first search.
pub mod dfs;
/// Path reconstruction from BFS/DFS parent maps.
pub mod paths;
/// Topological sort: DFS finish-order and Kahn's algorithm.
pub mod topo;

pub use bfs::{bfs, bfs_tree};
pub use bipartite::is_bipartite;
pub use components::connected_components;
pub use cycle::{has_cycle_directed, has_cycle_undirected};
pub use dfs::{dfs_iterative, dfs_recursive};
pub use paths::reconstruct_path;
pub use topo::{topological_sort_dfs, topological_sort_kahn};
