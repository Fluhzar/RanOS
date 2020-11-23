//! # Cycle

use std::time::Duration;

use ranos_ds::{const_val::ConstVal, rgb::{RGB, RGBOrder}};
use ranos_core::info::Info;

use super::*;

pub use super::breath::ColorOrder as ColorOrder;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CycleInfo();

impl Info for CycleInfo {
    fn new() -> Box<dyn Info>
    where
        Self: Sized,
    {
        Box::new(CycleInfo::default())
    }

    fn name(&self) -> String {
        "Cycle".to_owned()
    }

    fn details(&self) -> String {
        "Animates a static color for a given amount of time before cutting to the next color in a given order".to_owned()
    }
}

#[derive(Debug)]
pub struct Cycle {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,
    frame: Frame,

    order: ColorOrder,
    ind: usize,
    current_color: RGB,

    cycle_period: ConstVal<Duration>,
    cycle_time_remaining: Duration,
}

impl Cycle {
    pub fn new(
        runtime: Duration,
        cycle_period: Duration,
        brightness: f32,
        size: usize,
        order: ColorOrder,
    ) -> Self {
        Self {
            runtime: runtime.into(),
            time_remaining: runtime,
            frame: Frame::new(None, brightness, size),

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
    fn update(&mut self, dt: Duration) {
        self.time_remaining = if let Some(d) = self.time_remaining.checked_sub(dt) {
            d
        } else {
            Duration::new(0,0)
        };

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
            for led in self.frame.iter_mut() {
                *led = self.current_color;
            }

            self.cycle_period.get().clone() + self.cycle_time_remaining - dt
        }
    }

    fn set_brightness(&mut self, b: f32) {
        self.frame.set_brightness(b);
    }

    fn frame(&self) -> &Frame {
        &self.frame
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

impl Default for Cycle {
    fn default() -> Self {
        Self::new(
            Duration::from_secs(18),
            Duration::from_secs_f64(60.0/165.0),
            0.25,
            16,
            // ColorOrder::RandomBright,
            ColorOrder::Ordered(vec![
                RGB::from_code(0xFF0000, RGBOrder::RGB),
                RGB::from_code(0x00FF00, RGBOrder::RGB),
                RGB::from_code(0x0000FF, RGBOrder::RGB),
            ]),
        )
    }
}
