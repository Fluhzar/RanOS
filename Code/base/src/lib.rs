//! # Base
//!
//! This project contains the base code used in the RanOS project.

#![warn(missing_docs)]
#![warn(clippy::all)]

#[cfg(any(feature = "pi_draw", feature = "term_draw"))]
#[macro_use]
extern crate lazy_static;

pub mod animation;
pub mod collections;
pub mod draw;
pub mod util;
