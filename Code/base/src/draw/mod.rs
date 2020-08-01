//! # Draw
//! 
//! This module contains the types that will "draw" to the LEDs.
//! 
//! There are two drawers defined in this module, one being the actual drawer
//! that will draw the colors to physical LEDs connected to a Raspberry Pi, and
//! the second is an emulated LED setup that draws to "LEDs" on the terminal
//! with a configurable number of "LEDs" per row.

use crate::util::rgb::RGB;

#[cfg(feature = "pi_draw")]
pub mod pi_draw;

#[cfg(feature = "term_draw")]
pub mod term_draw;

/// Trait defining the ability to draw a frame of colors to LEDs.
pub trait Draw {
    /// Draws the internal frame to its destination.
    fn write_frame(&mut self);

    /// Returns the internal frame as a immutable slice.
    fn as_slice(&self) -> &[RGB];

    /// Returns the internal frame as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [RGB];
}
