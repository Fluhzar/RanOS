//! # Animation

pub mod breath;
pub mod rainbow;
pub mod strobe;

pub use breath::{Breath, BreathInfo};
pub use rainbow::{Rainbow, RainbowInfo};
pub use strobe::{Strobe, StrobeInfo};

use crate::ds::collections::frame::Frame;
use crate::util::Info;

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

/// Returns a `Vec` of animation `Info` objects.
pub fn animation_info() -> Vec<Box<dyn Info>> {
    vec![
        BreathInfo::new(),
        RainbowInfo::new(),
        StrobeInfo::new(),
    ]
}

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
