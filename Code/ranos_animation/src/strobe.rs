//! # Strobe

use std::time::Duration;

use serde::{Serialize, Deserialize};

use ranos_ds::{
    const_val::ConstVal,
    rgb::{RGBOrder, RGB},
};

use super::*;

/// Builder for the [`Strobe`](Strobe) animation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Strobe")]
pub struct StrobeBuilder {
    runtime: Duration,
    period: Duration,
    duty: f64,
    color: RGB,
}

impl StrobeBuilder {
    /// Sets the length of time the animation should run for.
    pub fn runtime(mut self: Box<Self>, runtime: Duration) -> Box<Self> {
        self.runtime = runtime;

        self
    }

    /// Sets the period, the amount of time before the strobe pattern repeats.
    pub fn period(mut self: Box<Self>, period: Duration) -> Box<Self> {
        self.period = period;

        self
    }

    /// Sets the duty cycle, a value in the range of [0, 1) representing the
    /// percentage of time that the LEDs are on within the `period`.
    pub fn duty(mut self: Box<Self>, duty: f64) -> Box<Self> {
        let duty = duty.min(1.0).max(0.0);
        self.duty = duty;

        self
    }

    /// Sets the color that will be strobing.
    pub fn color(mut self: Box<Self>, color: RGB) -> Box<Self> {
        self.color = color;

        self
    }

    /// Constructs a [`Strobe`](Strobe) object.
    pub fn build(self: Box<Self>) -> Strobe {
        Strobe::from_builder(self)
    }
}

#[typetag::serde]
impl AnimationBuilder for StrobeBuilder {
    fn build(self: Box<Self>) -> Box<dyn Animation> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use std::time::Duration;
    use ranos_ds::rgb::{RGB, RGBOrder};
    use crate::Strobe;
    use super::StrobeBuilder;

    #[test]
    fn test_serialize() {
        let builder = Strobe::builder();

        let data = serde_json::ser::to_string(&builder).unwrap();

        let expected = r#"{"runtime":{"secs":8,"nanos":0},"period":{"secs":0,"nanos":500000000},"duty":0.25,"color":[255,255,255]}"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"{"runtime":{"secs":8,"nanos":0},"period":{"secs":0,"nanos":500000000},"duty":0.25,"color":[255,255,255]}"#;

        let data: StrobeBuilder = serde_json::de::from_str(input).unwrap();

        assert_eq!(data.runtime, Duration::from_secs(8));
        assert_eq!(data.period, Duration::from_secs_f64(1.0 / ((1 << 1) as f64)));
        assert_eq!(data.duty, 1.0 / ((1 << 2) as f64));
        assert_eq!(data.color, RGB::from_code(0xFFFFFF, RGBOrder::RGB));
    }
}

/// Struct for animating a flickering light similar to the strobe lights one
/// might see at concerts or otherwise.
///
/// Allows for PWM-like control with the ability to define the period and
/// "duty cycle" of the strobe.
///
/// The `period` is simply the amount of time before the strobe pattern repeats,
/// and the `duty cycle` being a value in the range of [0, 1) representing the
/// percentage of time that the LEDs are on within the `period`.
#[derive(Debug)]
pub struct Strobe {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,

    period: ConstVal<f64>,
    duty: ConstVal<f64>,

    color: ConstVal<RGB>,

    time: f64,
}

impl Strobe {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<StrobeBuilder> {
        Box::new(
            StrobeBuilder {
                runtime: Duration::from_secs(8),
                period: Duration::from_secs_f64(1.0 / ((1 << 1) as f64)),
                duty: 1.0 / ((1 << 2) as f64),
                color: RGB::from_code(0xFFFFFF, RGBOrder::RGB),
            }
        )
    }

    fn from_builder(builder: Box<StrobeBuilder>) -> Self {
        Self::new(builder.runtime, builder.period, builder.duty, builder.color)
    }

    fn new(runtime: Duration, period: Duration, duty: f64, color: RGB, ) -> Self {
        let duty = duty.min(1.0).max(0.0);

        Self {
            runtime: ConstVal::new(runtime),
            time_remaining: runtime,

            period: ConstVal::new(period.as_secs_f64()),
            duty: ConstVal::new(duty),

            color: ConstVal::new(color),

            time: 0.0,
        }
    }
}

impl Animation for Strobe {
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> AnimationState {
        // Accumulate the time, clamping it to a range of [0, self.period)
        self.time = (self.time + dt.as_secs_f64()) % self.period.get();

        // Convert the time to a fraction in the range [0, 1)
        let r = self.time / self.period.get();

        // Set the current color, based on how long it's been in the current cycle
        let color = if r < *self.duty.get() {
            *self.color.get()
        } else {
            RGB::new()
        };

        // Copy the colors to the frame
        for led in frame.iter_mut() {
            *led = color;
        }

        let mut res = AnimationState::Continue;

        self.time_remaining = if let Some(d) = self.time_remaining.checked_sub(dt) {
            d
        } else {
            res = AnimationState::Last;

            Duration::new(0, 0)
        };

        res
    }

    fn time_remaining(&self) -> Duration {
        self.time_remaining
    }

    fn reset(&mut self) {
        self.time_remaining = *self.runtime.get();
        self.time = 0.0;
    }
}
