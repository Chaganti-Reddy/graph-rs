#![warn(missing_docs)]

//! # graph-advanced
//!
//! Advanced graph algorithms built on [`graph_core`]'s [`Graph`] trait.
//!
//! ## Algorithms
//!
//! | Module              | Algorithm                                  | Complexity        |
//! |---------------------|--------------------------------------------|-------------------|
//! | [`mod@tarjan_scc`]      | Tarjan's Strongly Connected Components     | O(V + E)          |
//! | [`mod@kosaraju_scc`]    | Kosaraju's SCC (two-pass DFS)              | O(V + E)          |
//! | [`mod@condensation`]    | DAG condensation of SCC graph              | O(V + E)          |
//! | [`mod@euler`]           | Euler path / circuit (Hierholzer's)        | O(E)              |
//! | [`mod@hamiltonian`]     | Hamiltonian path (backtracking)            | O(V!)             |
//! | [`mod@tsp`]             | Travelling Salesman — Held-Karp bitmask DP | O(2^V · V²)       |
//!
//! ## Design note: SCC algorithms
//!
//! Both [`tarjan_scc()`] and [`kosaraju_scc()`] solve the same problem in O(V + E).
//! Tarjan's algorithm completes in a single DFS pass using a stack and low-link
//! values. Kosaraju's algorithm is conceptually simpler — two DFS passes on the
//! original and transposed graph — but requires building a transposed graph.
//! Implement both and compare their output on the same graphs to verify they agree.
//!
//! [`Graph`]: graph_core::Graph

/// DAG condensation: contract each SCC into a single super-node.
pub mod condensation;
/// Euler path and circuit via Hierholzer's algorithm.
pub mod euler;
/// Hamiltonian path via backtracking (exact, exponential).
pub mod hamiltonian;
/// Kosaraju's two-pass SCC algorithm.
pub mod kosaraju_scc;
/// Tarjan's single-pass SCC algorithm using DFS low-link values.
pub mod tarjan_scc;
/// Travelling Salesman Problem via Held-Karp bitmask DP.
pub mod tsp;

pub use condensation::{condensation, CondensedGraph};
pub use euler::{euler_circuit, euler_path, EulerError};
pub use hamiltonian::hamiltonian_path;
pub use kosaraju_scc::kosaraju_scc;
pub use tarjan_scc::tarjan_scc;
pub use tsp::tsp_held_karp;
