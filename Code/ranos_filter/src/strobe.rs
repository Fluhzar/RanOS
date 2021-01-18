//! A strobing generator.
use std::time::Duration;

use serde::{Deserialize, Serialize};

use ranos_ds::const_val::ConstVal;

use super::*;

/// Builder for the [`Strobe`] generator.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Strobe")]
pub struct StrobeBuilder {
    frequency: f64,
    duty: f64,
}

impl StrobeBuilder {
    /// Sets the frequency, the number of cycles per second.
    pub fn frequency(mut self: Box<Self>, frequency: f64) -> Box<Self> {
        self.frequency = frequency;

        self
    }

    /// Sets the duty cycle, a value in the range of \[0, 1\) representing the
    /// percentage of time that the LEDs are on within the period of the filter.
    pub fn duty(mut self: Box<Self>, duty: f64) -> Box<Self> {
        let duty = duty.min(1.0).max(0.0);
        self.duty = duty;

        self
    }

    /// Constructs a [`Strobe`](Strobe) object.
    pub fn build(self: Box<Self>) -> Strobe {
        Strobe::from_builder(self)
    }
}

#[typetag::serde]
impl FilterBuilder for StrobeBuilder {
    fn build(self: Box<Self>) -> Box<dyn Filter> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use super::{Strobe, StrobeBuilder};

    #[test]
    fn test_serialize() {
        let builder = Strobe::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(period:(secs:0,nanos:500000000),duty:0.25,color:(255,255,255))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(period:(secs:0,nanos:500000000),duty:0.25,color:(255,255,255))"#;

        let data: StrobeBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.frequency, (1 << 1) as f64);
        assert_eq!(data.duty, 1.0 / ((1 << 2) as f64));
    }
}

/// Struct for animating a flickering light similar to the strobe lights one might see at concerts or otherwise.
///
/// Allows for PWM-like control with the ability to define the period and duty cycle of the strobe effect.
///
/// The `period` is simply the amount of time before the strobe pattern repeats,
/// and the `duty cycle` being a value in the range of \[0, 1) representing the
/// percentage of time that the LEDs are on within the `period`.
#[derive(Debug)]
pub struct Strobe {
    frequency: ConstVal<f64>,
    period: ConstVal<f64>,
    duty: ConstVal<f64>,

    time: f64,
}

impl Strobe {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<StrobeBuilder> {
        Box::new(StrobeBuilder {
            frequency: 2.0,
            duty: 1.0 / ((1 << 2) as f64),
        })
    }

    fn from_builder(builder: Box<StrobeBuilder>) -> Self {
        Self::new(builder.frequency, builder.duty)
    }

    fn new(frequency: f64, duty: f64) -> Self {
        let duty = duty.min(1.0).max(0.0);

        Self {
            frequency: ConstVal::new(frequency),
            period: ConstVal::new(1.0 / frequency),
            duty: ConstVal::new(duty),

            time: 0.0,
        }
    }
}

impl Filter for Strobe {
    fn filter_frame(&mut self, frame: &mut Frame, dt: Duration) -> FilterState {
        // Accumulate the time, clamping it to a range of [0, self.period)
        self.time = (self.time + dt.as_secs_f64()) % self.period.get();

        // Convert the time to a fraction in the range [0, 1)
        let r = self.time * self.frequency.get();

        // Filter the frame
        for led in frame.iter_mut() {
            if r > *self.duty.get() {
                led.scale(0.0);
            };
        }

        FilterState::Ok
    }

    fn reset(&mut self) {
        self.time = 0.0;
    }
}
