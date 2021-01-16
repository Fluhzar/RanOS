//! # Core
//!
//! This crate contains the core code used in the RanOS project.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

pub use max_line::MaxLine;
pub use timer::Timer;

pub mod curve;
pub mod id;
pub mod max_line;
pub mod timer;
