#![warn(missing_docs)]

//! # graph-collections
//!
//! Low-level collections for graph-rs: Stack, Queue, Heap, DisjointSet.

mod queue;
mod stack;

pub use queue::Queue;
pub use stack::Stack;
