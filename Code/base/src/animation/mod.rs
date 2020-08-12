//! # Runner

pub mod breath;
pub mod rainbow;

use crate::util::frame::Frame;
use std::time::Duration;

/// Trait for types that implement animations that sets the LEDs to a given
/// frame of the animation before being drawn.
pub trait Animation:
    std::fmt::Debug
{
    /// Updates the frame with the next frame of the animation given the input `dt`.
    fn update(&mut self, dt: Duration);

    /// Returns an immutable reference to the frame stored within the animation.
    fn frame(&self) -> &Frame;

    /// Returns the amount of time remaining for this animation to run before
    /// the drawer to continue to the next animation.
    fn time_remaining(&self) -> Duration;
}
