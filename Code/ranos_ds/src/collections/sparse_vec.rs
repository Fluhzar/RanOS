//! # Sparse Vec
//!
//! Re-exports of the `vectors` crate's types for simplicity, organization, and ease of use.

extern crate vectors;

pub use vectors::sparse::heap::SparseVector as SparseVecHeap;
pub use vectors::sparse::stack::SparseVector as SparseVecStack;
