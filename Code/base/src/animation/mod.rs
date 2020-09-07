//! # Animation

pub mod breath;
pub mod rainbow;
pub mod strobe;

pub use breath::{Breath, BreathInfo};
pub use rainbow::{Rainbow, RainbowInfo};
pub use strobe::{Strobe, StrobeInfo};

use crate::ds::collections::frame::Frame;
use std::time::Duration;

/// Trait for types that implement animations that sets the LEDs to a given
/// frame of the animation before being drawn.
pub trait Animation: std::fmt::Debug {
    /// Updates the frame with the next frame of the animation given the input `dt`.
    fn update(&mut self, dt: Duration);

    /// Returns an immutable reference to the frame stored within the animation.
    fn frame(&self) -> &Frame;

    /// Returns the amount of time remaining for this animation to run before
    /// the drawer to continue to the next animation.
    fn time_remaining(&self) -> Duration;
}

/// Returns a string containing info about the animations implemented in this
/// module.
///
/// # Parameters
///
/// - `max_line` - The maximum line length for the info.
pub fn animations_info(max_line: usize) -> String {
    use crate::util::{info, Info};
    info::format_info(
        &[BreathInfo::new(), RainbowInfo::new(), StrobeInfo::new()],
        max_line,
    )
}
