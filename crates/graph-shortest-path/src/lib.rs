//! # graph-shortest-path
//!
//! Shortest-path algorithms built on top of [`graph_core`]'s [`Graph`] trait.
//!
//! Every algorithm accepts any type implementing `Graph<Weight = f64>`, so it
//! works with both [`AdjacencyList`] and [`AdjacencyMatrix`] out of the box.
//!
//! ## Algorithms
//!
//! | Module            | Algorithm                          | Complexity         |
//! |-------------------|------------------------------------|--------------------|
//! | [`mod@dijkstra`]      | Dijkstra (single-source)           | O((V+E) log V)     |
//! | [`mod@bellman_ford`]  | Bellman-Ford (negative weights)    | O(V·E)             |
//! | [`mod@floyd_warshall`]| Floyd-Warshall (all-pairs)         | O(V³)              |
//! | [`mod@astar`]         | A* (goal-directed, heuristic)      | O(E log V)         |
//!
//! [`Graph`]: graph_core::Graph
//! [`AdjacencyList`]: graph_core::AdjacencyList
//! [`AdjacencyMatrix`]: graph_core::AdjacencyMatrix

/// A* goal-directed shortest path with a caller-supplied heuristic.
pub mod astar;
/// Bellman-Ford single-source shortest path; handles negative weights.
pub mod bellman_ford;
/// Dijkstra's single-source shortest path for non-negative weights.
pub mod dijkstra;
/// Floyd-Warshall all-pairs shortest path.
pub mod floyd_warshall;

pub use astar::astar;
pub use bellman_ford::{bellman_ford, BellmanFordResult};
pub use dijkstra::{dijkstra, DijkstraResult};
pub use floyd_warshall::floyd_warshall;
