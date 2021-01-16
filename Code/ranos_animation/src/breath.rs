//! A simple breathing animation, drawing a color starting at 0 brightness, then fading to full brightness, and back to 0.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use ranos_ds::{const_val::ConstVal, rgb::RGB};

use super::*;

/// Builder for the [`Breath`] animation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Breath")]
pub struct BreathBuilder {
    runtime: Duration,
    breath_duration: Duration,
    order: ColorOrder,
}

impl BreathBuilder {
    /// Sets the length of time the animation should run for.
    pub fn runtime(mut self: Box<Self>, runtime: Duration) -> Box<Self> {
        self.runtime = runtime;

        self
    }

    /// Sets the duration a single color is drawn for, from black up to full color back down to black.
    pub fn breath_duration(mut self: Box<Self>, breath_duration: Duration) -> Box<Self> {
        self.breath_duration = breath_duration;

        self
    }

    /// Sets a given order that the animation cycles through.
    pub fn order(mut self: Box<Self>, order: ColorOrder) -> Box<Self> {
        self.order = order;

        self
    }

    /// Constructs a [`Breath`] object.
    pub fn build(self: Box<Self>) -> Breath {
        Breath::from_builder(self)
    }
}

#[typetag::serde]
impl AnimationBuilder for BreathBuilder {
    fn build(self: Box<Self>) -> Box<dyn Animation> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use super::{Breath, BreathBuilder};
    use crate::ColorOrder;
    use ranos_ds::rgb::RGB;
    use std::time::Duration;

    #[test]
    fn test_serialize() {
        let builder = Breath::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(runtime:(secs:18,nanos:0),breath_duration:(secs:3,nanos:0),order:Ordered([(255,0,0),(255,255,0),(0,255,0),(0,255,255),(0,0,255),(255,0,255)]))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(runtime:(secs:18,nanos:0),breath_duration:(secs:3,nanos:0),order:Ordered([(255,0,0),(255,255,0),(0,255,0),(0,255,255),(0,0,255),(255,0,255)]))"#;

        let data: BreathBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.runtime, Duration::from_secs(18));
        assert_eq!(data.breath_duration, Duration::from_secs(3));
        assert_eq!(
            data.order,
            ColorOrder::Ordered(vec![
                RGB::from_hsv(0.0, 1.0, 1.0),
                RGB::from_hsv(60.0, 1.0, 1.0),
                RGB::from_hsv(120.0, 1.0, 1.0),
                RGB::from_hsv(180.0, 1.0, 1.0),
                RGB::from_hsv(240.0, 1.0, 1.0),
                RGB::from_hsv(300.0, 1.0, 1.0),
            ])
        );
    }
}

/// Struct for an animated breathing display that will either walk through a
/// provided list of colors or select random colors, each color fading along a
/// parabolic curve from black to the chosen color and back down to black.
#[derive(Debug)]
pub struct Breath {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,

    order: ColorOrder,
    ind: usize,
    current_color: RGB,

    acc: ConstVal<f32>,
    vel: f32,
    vel0: ConstVal<f32>,
    pos: f32,
}

impl Breath {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<BreathBuilder> {
        Box::new(BreathBuilder {
            runtime: Duration::from_secs(18),
            breath_duration: Duration::from_secs(3),
            order: ColorOrder::Ordered(vec![
                RGB::from_hsv(0.0, 1.0, 1.0),
                RGB::from_hsv(60.0, 1.0, 1.0),
                RGB::from_hsv(120.0, 1.0, 1.0),
                RGB::from_hsv(180.0, 1.0, 1.0),
                RGB::from_hsv(240.0, 1.0, 1.0),
                RGB::from_hsv(300.0, 1.0, 1.0),
            ]),
        })
    }

    fn from_builder(builder: Box<BreathBuilder>) -> Self {
        Self::new(builder.runtime, builder.breath_duration, builder.order)
    }

    fn new(runtime: Duration, breath_duration: Duration, order: ColorOrder) -> Self {
        Self {
            runtime: ConstVal::new(runtime),
            time_remaining: runtime,

            order: order.clone(),
            ind: 0,
            current_color: match order {
                ColorOrder::Ordered(v) => v[0],
                ColorOrder::Random => RGB::random(),
                ColorOrder::RandomBright => RGB::random_bright(),
            },

            acc: ConstVal::new(-8.0 / breath_duration.as_secs_f32().powi(2)),
            vel: 4.0 / breath_duration.as_secs_f32(),
            vel0: ConstVal::new(4.0 / breath_duration.as_secs_f32()),
            pos: 0.0,
        }
    }
}

impl Animation for Breath {
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> AnimationState {
        self.vel += self.acc.get() * dt.as_secs_f32();
        self.pos += self.vel * dt.as_secs_f32();

        if self.pos <= 0.0 && self.vel < 0.0 {
            self.pos = 0.0;
            self.vel = *self.vel0.get();

            if let ColorOrder::Ordered(v) = &self.order {
                self.ind += 1;
                self.ind %= v.len();
                self.current_color = v[self.ind];
            } else {
                self.current_color = RGB::random();
            }
        }

        for led in frame.iter_mut() {
            *led = self.current_color.scale(self.pos);
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

    fn reset(mut self: Box<Self>) -> Box<dyn Animation> {
        self.time_remaining = *self.runtime.get();
        self.ind = 0;
        self.current_color = match &self.order {
            ColorOrder::Ordered(v) => v[0],
            ColorOrder::Random => RGB::random(),
            ColorOrder::RandomBright => RGB::random_bright(),
        };
        self.vel = *self.vel0.get();

        self
    }
}
