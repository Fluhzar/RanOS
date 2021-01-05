//! # Cycle

use std::time::Duration;

use ranos_ds::{const_val::ConstVal, rgb::{RGB, RGBOrder}};
use ranos_core::info::Info;

use super::*;

pub use super::breath::ColorOrder as ColorOrder;

/// Presents some info about `Cycle` for pretty printing.
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
    /// Creates new `Cycle` object.
    ///
    /// # Parameters
    ///
    /// * `runtime` - The length of time this animation will run.
    /// * `cycle_period` - The duration a single color is drawn for.
    /// * `brightness` - The brightness value to use. Should be in range [0, 1].
    /// * `size` - The number of LEDs this animation will animate for.
    /// * `order` - A given order that the animation cycles through.
    pub fn new(
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

impl Default for Cycle {
    fn default() -> Self {
        Self::new(
            Duration::from_secs_f64(60.0/165.0*3.0*15.0),
            Duration::from_secs_f64(60.0/165.0),
            // ColorOrder::RandomBright,
            ColorOrder::Ordered(vec![
                RGB::from_code(0xFF0000, RGBOrder::RGB),
                RGB::from_code(0x00FF00, RGBOrder::RGB),
                RGB::from_code(0x0000FF, RGBOrder::RGB),
            ]),
        )
    }
}
