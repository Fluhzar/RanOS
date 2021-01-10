//! # Cycle

use std::time::Duration;

use serde::{Serialize, Deserialize};

use ranos_ds::{const_val::ConstVal, rgb::{RGB, RGBOrder}};

use super::*;

/// Builder for the [`Cycle`](Cycle) animation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Cycle")]
pub struct CycleBuilder {
    runtime: Duration,
    cycle_period: Duration,
    order: ColorOrder,
}

impl CycleBuilder {
    /// Sets the length of time the animation should run for.
    pub fn runtime(mut self: Box<Self>, runtime: Duration) -> Box<Self> {
        self.runtime = runtime;

        self
    }

    /// Sets the duration a single color is drawn for.
    pub fn cycle_period(mut self: Box<Self>, cycle_period: Duration) -> Box<Self> {
        self.cycle_period = cycle_period;

        self
    }

    /// Sets a given order that the animation cycles through.
    pub fn order(mut self: Box<Self>, order: ColorOrder) -> Box<Self> {
        self.order = order;

        self
    }

    /// Constructs a [`Cycle`](Cycle) object.
    pub fn build(self: Box<Self>) -> Cycle {
        Cycle::from_builder(self)
    }
}

impl AnimationBuilder for CycleBuilder {
    fn build(self: Box<Self>) -> Box<dyn Animation> {
        Box::new(self.build())
    }
}

/// Struct for a simple cycling between colors by either walking a provided list
/// of colors or generating random colors. Each color is shown for the set amount
/// of time before proceeding to the next color.
#[derive(Debug)]
pub struct Cycle {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,

    order: ColorOrder,
    ind: usize,
    current_color: RGB,

    cycle_period: ConstVal<Duration>,
    cycle_time_remaining: Duration,
}

impl Cycle {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<CycleBuilder> {
        Box::new(
            CycleBuilder {
                runtime: Duration::from_secs_f64(60.0/165.0*3.0*15.0),
                cycle_period: Duration::from_secs_f64(60.0/165.0),
                order: ColorOrder::Ordered(vec![
                    RGB::from_code(0xFF0000, RGBOrder::RGB),
                    RGB::from_code(0x00FF00, RGBOrder::RGB),
                    RGB::from_code(0x0000FF, RGBOrder::RGB),
                ]),
            }
        )
    }

    fn from_builder(builder: Box<CycleBuilder>) -> Self {
        Self::new(builder.runtime, builder.cycle_period, builder.order)
    }

    fn new(
        runtime: Duration,
        cycle_period: Duration,
        order: ColorOrder,
    ) -> Self {
        Self {
            runtime: runtime.into(),
            time_remaining: runtime,

            order: order.clone(),
            ind: 0,
            current_color: match order {
                ColorOrder::Ordered(v) => v[0],
                ColorOrder::Random => RGB::random(),
                ColorOrder::RandomBright => RGB::random_bright(),
            },

            cycle_period: cycle_period.into(),
            cycle_time_remaining: cycle_period,
        }
    }
}

impl Animation for Cycle {
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> AnimationState {
        self.cycle_time_remaining = if let Some(d) = self.cycle_time_remaining.checked_sub(dt) {
            d
        } else {
            if let ColorOrder::Ordered(v) = &self.order {
                self.ind += 1;
                self.ind %= v.len();
                self.current_color = v[self.ind];
            } else {
                self.current_color = RGB::random()
            }

            // Only update the frame when there's a new color
            for led in frame.iter_mut() {
                *led = self.current_color;
            }

            self.cycle_period.get().clone() + self.cycle_time_remaining - dt
        };

        let mut res = AnimationState::Continue;

        self.time_remaining = if let Some(d) = self.time_remaining.checked_sub(dt) {
            d
        } else {
            res = AnimationState::Last;

            Duration::new(0,0)
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
        self.cycle_time_remaining = *self.cycle_period.get();
    }
}
