//! # Animation

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

extern crate ranos_core;
extern crate ranos_ds;

use std::time::Duration;

pub use breath::{Breath, BreathInfo};
pub use cycle::{Cycle, CycleInfo};
pub use rainbow::{Rainbow, RainbowInfo};
pub use strobe::{Strobe, StrobeInfo};

use ranos_ds::collections::frame::Frame;
use ranos_core::Info;

pub mod breath;
pub mod cycle;
pub mod rainbow;
pub mod strobe;

/// Enum denoting different end-states that an [`Animation`][crate::Animation]
/// object may return.
///
/// The `ErrRetry` state is given for use in statistical tracking and more
/// complex operations that could fail, but still be able to continue (e.g. file
/// I/O).
pub enum AnimationState {
    /// Denotes that the operation was successful and the object can operate for more iterations.
    Continue,
    /// Denotes that the operation was successful and the object has nothing more to operate on.
    Last,
    /// Denotes that an error occurred but the object can continue to operate.
    ErrRetry,
    /// Denotes that an error occurred and cannot be recovered from.
    ErrFatal,
}

/// Trait for types that implement animations that sets the LEDs to a given
/// frame of the animation before being drawn.
pub trait Animation: std::fmt::Debug {
    /// Renders the frame with the next frame of the animation given the input `dt`.
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> AnimationState;

    /// Returns the amount of time remaining for this animation to run before
    /// the drawer to continue to the next animation.
    fn time_remaining(&self) -> Duration;

    /// Resets the animation to its pre-run state, appearing as if it were never
    /// run before.
    fn reset(&mut self);
}

/// Returns a `Vec` of animation `Info` objects.
pub fn animation_info() -> Vec<Box<dyn Info>> {
    vec![BreathInfo::new(), CycleInfo::new(), RainbowInfo::new(), StrobeInfo::new()]
}

/// Attempts to parse the given `String` into an `Animation` object, returning
/// `None` on failure.
pub fn match_animation<T>(s: T) -> Option<Box<dyn Animation>>
where
    T: std::ops::Deref<Target = str>,
{
    let s = s.to_lowercase();

    if s == BreathInfo::new().name().to_lowercase() {
        Some(Box::new(Breath::default()))
    } else if s == CycleInfo::new().name().to_lowercase() {
        Some(Box::new(Cycle::default()))
    } else if s == RainbowInfo::new().name().to_lowercase() {
        Some(Box::new(Rainbow::default()))
    } else if s == StrobeInfo::new().name().to_lowercase() {
        Some(Box::new(Strobe::default()))
    } else {
        None
    }
}
