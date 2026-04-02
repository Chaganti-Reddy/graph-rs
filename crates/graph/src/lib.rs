#![warn(missing_docs)]

//! # graph
//!
//! A complete, workspace-wide re-export of all graph data structures and
//! algorithms in the `graph-rs` library.
//!
//! Import this crate and use the [`prelude`] to get everything in scope:
//!
//! ```
//! use graph::prelude::*;
//!
//! let mut g: AdjacencyList<&str> = AdjacencyList::directed();
//! let a = g.add_node("A");
//! let b = g.add_node("B");
//! g.add_edge(a, b, 1.0).unwrap();
//!
//! let result = dijkstra(&g, a).unwrap();
//! assert!(result.distances.contains_key(&b));
//! ```
//!
//! ## Crates in this workspace
//!
//! | Crate                 | Contents                                          |
//! |-----------------------|---------------------------------------------------|
//! | `graph-collections`   | Stack, Queue, Deque, MinHeap, PriorityQueue, UnionFind |
//! | `graph-core`          | Graph trait, AdjacencyList, AdjacencyMatrix       |
//! | `graph-traversal`     | DFS, BFS, topological sort, cycle detection       |
//! | `graph-shortest-path` | Dijkstra, Bellman-Ford, Floyd-Warshall, A\*       |
//! | `graph-spanning`      | Kruskal, Prim, bridges, articulation points       |
//! | `graph-flow`          | Ford-Fulkerson, Edmonds-Karp, min-cut, Hopcroft-Karp |
//! | `graph-advanced`      | Tarjan SCC, Kosaraju SCC, Euler, Hamiltonian, TSP |

/// Brings the entire `graph-rs` public API into scope.
///
/// ```
/// use graph::prelude::*;
/// ```
pub mod prelude {
    pub use graph_advanced::*;
    pub use graph_collections::*;
    pub use graph_core::*;
    pub use graph_flow::*;
    pub use graph_shortest_path::*;
    // graph_spanning re-exports DisjointSet which conflicts with
    // graph_collections::DisjointSet — export spanning items explicitly.
    pub use graph_spanning::{articulation_points, bridges, kruskal, prim, SpanningTree};
    pub use graph_traversal::*;
}
