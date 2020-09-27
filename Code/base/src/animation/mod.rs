//! # Animation

pub use breath::{Breath, BreathInfo};
pub use rainbow::{Rainbow, RainbowInfo};
use std::time::Duration;
pub use strobe::{Strobe, StrobeInfo};

use crate::ds::collections::frame::Frame;
use crate::util::Info;

pub mod breath;
pub mod rainbow;
pub mod strobe;

/// Trait for types that implement animations that sets the LEDs to a given
/// frame of the animation before being drawn.
pub trait Animation: std::fmt::Debug {
    /// Updates the frame with the next frame of the animation given the input `dt`.
    fn update(&mut self, dt: Duration);

    /// Exposes the ability to dynamically set the brightness.
    fn set_brightness(&mut self, brightness: f32);

    /// Returns an immutable reference to the frame stored within the animation.
    fn frame(&self) -> &Frame;

    /// Returns the amount of time remaining for this animation to run before
    /// the drawer to continue to the next animation.
    fn time_remaining(&self) -> Duration;

    /// Resets the animation to its pre-run state, appearing as if it were never
    /// run before.
    fn reset(&mut self);
}

/// Returns a `Vec` of animation `Info` objects.
pub fn animation_info() -> Vec<Box<dyn Info>> {
    vec![
        BreathInfo::new(),
        RainbowInfo::new(),
        StrobeInfo::new(),
    ]
}

/// Attempts to parse the given `String` into an `Animation` object, returning
/// `None` on failure.
pub fn match_animation<T>(s: T) -> Option<Box<dyn Animation>>
where
    T: std::ops::Deref<Target=str>
{
    let s = s.to_lowercase();

    if s == BreathInfo::new().name().to_lowercase() {
        Some(Box::new(Breath::default()))
    } else if s == RainbowInfo::new().name().to_lowercase() {
        Some(Box::new(Rainbow::default()))
    } else if s == StrobeInfo::new().name().to_lowercase() {
        Some(Box::new(Strobe::default()))
    } else {
        None
    }
}
