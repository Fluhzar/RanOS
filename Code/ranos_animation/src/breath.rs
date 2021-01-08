//! # Breath

use std::time::Duration;

use serde::{Serialize, Deserialize};

use ranos_ds::{const_val::ConstVal, rgb::RGB};

use super::*;

/// Builder for the [`Breath`](Breath) animation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Breath")]
pub struct BreathBuilder {
    runtime: Duration,
    breath_duration: Duration,
    order: ColorOrder,
}

impl BreathBuilder {
    /// Sets the length of time the animation should run for.
    pub fn runtime(mut self, runtime: Duration) -> Self {
        self.runtime = runtime;

        self
    }

    /// Sets the duration a single color is drawn for, from black up to full color back down to black.
    pub fn breath_duration(mut self, breath_duration: Duration) -> Self {
        self.breath_duration = breath_duration;

        self
    }

    /// Sets a given order that the animation cycles through.
    pub fn order(mut self, order: ColorOrder) -> Self {
        self.order = order;

        self
    }

    /// Constructs a [`Breath`](Breath) object.
    pub fn build(self) -> Breath {
        Breath::from_builder(self)
    }
}

impl AnimationBuilder for BreathBuilder {
    fn build(self) -> Box<dyn Animation> {
        Box::new(self.build())
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
    pub fn builder() -> BreathBuilder {
        BreathBuilder {
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
        }
    }

    fn from_builder(builder: BreathBuilder) -> Self {
        Self::new(builder.runtime, builder.breath_duration, builder.order)
    }

    fn new(
        runtime: Duration,
        breath_duration: Duration,
        order: ColorOrder,
    ) -> Self {
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

    fn reset(&mut self) {
        self.time_remaining = *self.runtime.get();
        self.ind = 0;
        self.current_color = match &self.order {
            ColorOrder::Ordered(v) => v[0],
            ColorOrder::Random => RGB::random(),
            ColorOrder::RandomBright => RGB::random_bright(),
        };
        self.vel = *self.vel0.get();
    }
}
