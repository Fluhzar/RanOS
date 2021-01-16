//! # Audio
//!
//! Provides various functions and algorithms for interpreting audio in functional ways.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

/// Size of the buffer used in all audio calculations.
pub const SIZE: usize = 1 << 10;

pub mod analysis;
pub mod accessor;
pub mod util;

#[cfg(feature = "audio_out")]
pub mod playback;
