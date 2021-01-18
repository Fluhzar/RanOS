//! Draws a solid color to all pixels.

use std::time::Duration;
use serde::{Deserialize, Serialize};
use ranos_ds::{const_val::ConstVal, rgb::{RGB, RGBOrder}};
use super::*;

/// Builder for the [`Solid`] animation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Solid")]
pub struct SolidBuilder {
    runtime: Duration,
    color: RGB,
}

impl SolidBuilder {
    /// Sets the length of time the animation should run for.
    pub fn runtime(mut self: Box<Self>, runtime: Duration) -> Box<Self> {
        self.runtime = runtime;

        self
    }

    /// The color to draw.
    pub fn color(mut self: Box<Self>, color: RGB) -> Box<Self> {
        self.color = color;

        self
    }

    /// Constructs a [`Solid`] object.
    pub fn build(self: Box<Self>) -> Solid {
        Solid::from_builder(self)
    }
}

#[typetag::serde]
impl AnimationBuilder for SolidBuilder {
    fn build(self: Box<Self>) -> Box<dyn Animation> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use super::*;
    use ranos_ds::rgb::RGB;
    use std::time::Duration;

    #[test]
    fn test_serialize() {
        let builder = Solid::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(runtime:(secs:8,nanos:0),color:(0,255,255))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(runtime:(secs:8,nanos:0),color:(0,255,255))"#;

        let data: SolidBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.runtime, Duration::from_secs(8));
        assert_eq!(data.color, RGB::from_tuple((0, 255, 255), RGBOrder::RGB));
    }
}

/// Struct for a simple solid color to be displayed.
#[derive(Debug)]
pub struct Solid {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,

    color: ConstVal<RGB>,
}

impl Solid {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<SolidBuilder> {
        Box::new(SolidBuilder {
            runtime: Duration::from_secs(8),
            color: RGB::from_tuple((0, 255, 255), RGBOrder::RGB),
        })
    }

    fn from_builder(builder: Box<SolidBuilder>) -> Self {
        Self::new(builder.runtime, builder.color)
    }

    fn new(runtime: Duration, color: RGB) -> Self {
        Self {
            runtime: ConstVal::new(runtime),
            time_remaining: runtime,

            color: ConstVal::new(color),
        }
    }
}

impl Animation for Solid {
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> AnimationState {
        for led in frame.iter_mut() {
            *led = *self.color.get();
        }

        let(remaining, state) = if let Some(d) = self.time_remaining.checked_sub(dt) {
            (d, AnimationState::Continue)
        } else {
            (Duration::new(0, 0), AnimationState::Last)
        };

        self.time_remaining = remaining;

        state
    }

    fn reset(mut self: Box<Self>) -> Box<dyn Animation> {
        self.time_remaining = *self.runtime.get();

        self
    }
}
