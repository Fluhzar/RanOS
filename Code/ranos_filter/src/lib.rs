use std::{fmt::Debug, time::Duration};

use ranos_ds::collections::Frame;

pub use breath::Breath;
pub use strobe::Strobe;

pub mod breath;
pub mod strobe;

/// Enum denoting different end-states that an [`Filter`] object may return.
///
/// The `ErrRetry` state is given for use in statistical tracking and more
/// complex operations that could fail, but still be able to continue (e.g. file
/// I/O).
pub enum FilterState {
    /// Denotes that the operation was successful.
    Ok,
    /// Denotes that an error occurred but the object can retry the operation.
    ErrRetry,
    /// Denotes that an error occurred that is not recoverable for this frame, but will not be fatal for following frames.
    ErrSkip,
    /// Denotes that an error occurred and cannot be recovered from.
    ErrFatal,
}

/// Trait for types that implement algorithms that filter the data already present within a frame.
pub trait Filter: Debug {
    /// Filters the provided frame of pixels.
    ///
    /// This function is called successively for each new frame with `dt` given as the time since the previous call.
    fn filter_frame(&mut self, frame: &mut Frame, dt: Duration) -> FilterState;

    /// Resets the filter to its pre-run state, operating as if it were never run before
    fn reset(&mut self);
}

/// Trait for building filter types.
#[typetag::serde(tag = "type", content = "value")]
pub trait FilterBuilder: std::fmt::Debug {
    /// Creates a new filter object from the builder.
    fn build(self: Box<Self>) -> Box<dyn Filter>;
}

#[cfg(test)]
mod builder_test {
    use crate::{Breath, FilterBuilder};

    #[test]
    fn test_serialize() {
        let builder: Box<dyn FilterBuilder> = Breath::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(type:"BreathBuilder",value:(breath_duration:(secs:3,nanos:0)))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(type:"BreathBuilder",value:(breath_duration:(secs:3,nanos:0)))"#;

        assert_eq!(
            ron::ser::to_string(&ron::de::from_str::<Box<dyn FilterBuilder>>(input).unwrap())
                .unwrap(),
            input
        );
    }
}
