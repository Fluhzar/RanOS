//! Module of collections used within this project.

pub use frame::Frame;
pub use sparse_vec::SparseVecHeap as SparseVec; // Choose heap-allocation as the default `SparseVec` type

pub mod frame;
pub mod sparse_vec;
