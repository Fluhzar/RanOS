//! Defines the ability for generators to be rendered.

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
pub use solid::Solid;
pub use strobe::Strobe;

pub mod breath;
pub mod color_order;
pub mod cycle;
pub mod rainbow;
pub mod solid;
pub mod strobe;

/// Enum denoting different end-states that an [`Generator`] object may return.
pub enum GeneratorState {
    /// Denotes that the operation was successful.
    Ok,
    /// Denotes that an error occurred but the object can retry the operation.
    ErrRetry,
    /// Denotes that an error occurred that is not recoverable for this frame, but will not be fatal for following frames.
    ErrSkip,
    /// Denotes that an error occurred and cannot be recovered from.
    ErrFatal,
}

/// Trait for types that implement types that animates the pixels of a frame.
pub trait Generator: std::fmt::Debug {
    /// Returns the id of this generator.
    fn id(&self) -> usize;

    /// Renders the frame with the next frame of the generator given the input `dt`.
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> GeneratorState;

    /// Resets the generator to its pre-run state, operating as if it were never run before.
    fn reset(&mut self);
}

/// Trait for building generator types.
#[typetag::serde(tag = "type", content = "value")]
pub trait GeneratorBuilder: std::fmt::Debug {
    /// Creates a new generator object from the builder.
    fn build(self: Box<Self>) -> Box<dyn Generator>;
}

#[cfg(test)]
mod builder_test {
    use crate::{Cycle, GeneratorBuilder};

    #[test]
    fn test_serialize() {
        let builder: Box<dyn GeneratorBuilder> = Cycle::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(type:"CycleBuilder",value:(cycle_period:(secs:0,nanos:363636363),order:Ordered([(255,0,0),(0,255,0),(0,0,255)])))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(type:"CycleBuilder",value:(cycle_period:(secs:0,nanos:363636363),order:Ordered([(255,0,0),(0,255,0),(0,0,255)])))"#;

        assert_eq!(
            ron::ser::to_string(&ron::de::from_str::<Box<dyn GeneratorBuilder>>(input).unwrap())
                .unwrap(),
            input
        );
    }
}
