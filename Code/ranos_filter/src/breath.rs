//! A simple breathing filter, filtering a frame starting at 0 brightness, then fading to full brightness, and back to 0.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use ranos_ds::const_val::ConstVal;

use super::*;

/// Builder for the [`Breath`] generator.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Breath")]
pub struct BreathBuilder {
    breath_duration: Duration,
}

impl BreathBuilder {
    /// Sets the duration a single color is drawn for, from black up to full color back down to black.
    pub fn breath_duration(mut self: Box<Self>, breath_duration: Duration) -> Box<Self> {
        self.breath_duration = breath_duration;

        self
    }

    /// Constructs a [`Breath`] object.
    pub fn build(self: Box<Self>) -> Breath {
        Breath::from_builder(self)
    }
}

#[typetag::serde]
impl FilterBuilder for BreathBuilder {
    fn build(self: Box<Self>) -> Box<dyn Filter> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use super::{Breath, BreathBuilder};
    use std::time::Duration;

    #[test]
    fn test_serialize() {
        let builder = Breath::builder();
        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(breath_duration:(secs:3,nanos:0))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(breath_duration:(secs:3,nanos:0))"#;
        let data: BreathBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.breath_duration, Duration::from_secs(3));
    }
}

/// Struct for a filtered breathing display, fading a supplied frame along a
/// parabolic curve from black to the full frame and back down to black.
#[derive(Debug)]
pub struct Breath {
    acc: ConstVal<f32>,
    vel: f32,
    vel0: ConstVal<f32>,
    pos: f32,
}

impl Breath {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<BreathBuilder> {
        Box::new(BreathBuilder {
            breath_duration: Duration::from_secs(16),
        })
    }

    fn from_builder(builder: Box<BreathBuilder>) -> Self {
        Self::new(builder.breath_duration)
    }

    fn new(breath_duration: Duration) -> Self {
        Self {
            acc: ConstVal::new(-8.0 / breath_duration.as_secs_f32().powi(2)),
            vel: 4.0 / breath_duration.as_secs_f32(),
            vel0: ConstVal::new(4.0 / breath_duration.as_secs_f32()),
            pos: 0.0,
        }
    }
}

impl Filter for Breath {
    fn filter_frame(&mut self, frame: &mut Frame, dt: Duration) -> FilterState {
        self.vel += self.acc.get() * dt.as_secs_f32();
        self.pos += self.vel * dt.as_secs_f32();

        if self.pos <= 0.0 && self.vel < 0.0 {
            self.pos = 0.0;
            self.vel = *self.vel0.get();
        }

        for led in frame.iter_mut() {
            led.scale(self.pos);
        }

        FilterState::Ok
    }

    fn reset(&mut self) {
        self.vel = *self.vel0.get();
    }
}
