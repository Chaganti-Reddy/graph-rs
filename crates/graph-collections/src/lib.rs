#![warn(missing_docs)]

//! # graph-collections
//!
//! Low-level collections for graph-rs: Stack, Queue, Heap, DisjointSet.

mod deque;
mod queue;
mod stack;

pub use deque::Deque;
pub use queue::Queue;
pub use stack::Stack;
