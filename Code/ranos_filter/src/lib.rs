use std::{fmt::Debug, time::Duration};

use ranos_ds::collections::Frame;

/// Enum denoting different end-states that an [`Filter`] object may return.
///
/// The `ErrRetry` state is given for use in statistical tracking and more
/// complex operations that could fail, but still be able to continue (e.g. file
/// I/O).
pub enum FilterState {
    /// Denotes that the operation was successful and the object can operate for more iterations.
    Continue,
    /// Denotes that the operation was successful and the object has nothing more to operate on.
    Last,
    /// Denotes that an error occurred but the object can continue to operate.
    ErrRetry,
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
    fn reset(self: Box<Self>) -> Box<dyn Filter>;
}

/// Trait for building filter types.
#[typetag::serde(tag = "type", content = "value")]
pub trait FilterBuilder: std::fmt::Debug {
    /// Creates a new filter object from the builder.
    fn build(self: Box<Self>) -> Box<dyn Filter>;
}
