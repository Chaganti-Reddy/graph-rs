#![warn(missing_docs)]

//! # graph-core
//!
//! Core graph abstractions for graph-rs: type-safe node/edge identifiers,
//! the central [`Graph`] trait, and two primary representations —
//! [`AdjacencyList`] and [`AdjacencyMatrix`].
//!

mod adjacency_list;
mod adjacency_matrix;
mod builder;
mod edge;
mod error;
mod graph;
mod id;

pub use adjacency_list::AdjacencyList;
pub use adjacency_matrix::AdjacencyMatrix;
pub use builder::GraphBuilder;
pub use edge::{Edge, UnweightedEdge, WeightedEdge};
pub use error::GraphError;
pub use graph::Graph;
pub use id::{EdgeId, NodeId};
