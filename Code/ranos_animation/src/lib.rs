//! # Animation

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

extern crate ranos_core;
extern crate ranos_ds;

use std::time::Duration;

use ranos_ds::collections::frame::Frame;

pub use breath::Breath;
pub use color_order::ColorOrder;
pub use cycle::Cycle;
pub use rainbow::Rainbow;
pub use strobe::Strobe;

pub mod breath;
pub mod color_order;
pub mod cycle;
pub mod rainbow;
pub mod strobe;

/// Enum denoting different end-states that an [`Animation`](crate::Animation)
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

/// Trait for building animation types.
#[typetag::serde(tag = "type", content = "value")]
pub trait AnimationBuilder: std::fmt::Debug {
    /// Creates a new animation object from the builder.
    fn build(self: Box<Self>) -> Box<dyn Animation>;
}

#[cfg(test)]
mod builder_test {
    use crate::{AnimationBuilder, Cycle};

    #[test]
    fn test_serialize() {
        let builder: Box<dyn AnimationBuilder> = Cycle::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(type:"CycleBuilder",value:(runtime:(secs:16,nanos:363636363),cycle_period:(secs:0,nanos:363636363),order:Ordered([(255,0,0),(0,255,0),(0,0,255)])))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(type:"CycleBuilder",value:(runtime:(secs:16,nanos:363636363),cycle_period:(secs:0,nanos:363636363),order:Ordered([(255,0,0),(0,255,0),(0,0,255)])))"#;

        assert_eq!(
            ron::ser::to_string(
                &ron::de::from_str::<Box<dyn AnimationBuilder>>(input).unwrap()
            )
            .unwrap(),
            input
        );
    }
}
