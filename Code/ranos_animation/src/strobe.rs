//! # Strobe

use std::time::Duration;

use ranos_ds::{
    const_val::ConstVal,
    rgb::{RGBOrder, RGB},
};
use ranos_core::info::Info;

use super::*;

/// Presents some info about `Strobe` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StrobeInfo();

impl Info for StrobeInfo {
    fn new() -> Box<dyn Info>
    where
        Self: Sized,
    {
        Box::new(StrobeInfo::default())
    }

    fn name(&self) -> String {
        "Strobe".to_owned()
    }

    fn details(&self) -> String {
        "Animates a flickering light similar to the strobe lights one might see at concerts or otherwise.".to_owned()
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
    frame: Frame,

    period: ConstVal<f64>,
    duty: ConstVal<f64>,

    color: ConstVal<RGB>,

    time: f64,
}

impl Strobe {
    /// Creates a new `Strobe` animation.
    ///
    /// # Parameters
    ///
    /// - `runtime` - The length of time this animation will run for.
    /// - `brightness` - The brightness value to use. Should be in range [0, 1].
    /// - `size` - The number of LEDs this animation will animate for.
    /// - `period` - The period of time before the strobe animation repeats,
    /// typically a number less than 1.
    /// - `duty` - The percentage of time in the range of [0, 1) representing
    /// the percentage of time the LEDs are on within the `period`.
    pub fn new(
        runtime: Duration,
        brightness: f32,
        size: usize,
        period: Duration,
        duty: f64,
        color: RGB,
    ) -> Self {
        let duty = duty.min(1.0).max(0.0);

        Self {
            runtime: ConstVal::new(runtime),
            time_remaining: runtime,
            frame: Frame::new(brightness, size),

            period: ConstVal::new(period.as_secs_f64()),
            duty: ConstVal::new(duty),

            color: ConstVal::new(color),

            time: 0.0,
        }
    }
}

impl Animation for Strobe {
    fn update(&mut self, dt: Duration) {
        // Calculate remaining time
        self.time_remaining = if let Some(d) = self.time_remaining.checked_sub(dt) {
            d
        } else {
            Duration::new(0, 0)
        };

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
        for led in self.frame.iter_mut() {
            *led = color;
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
        self.time = 0.0;
    }
}

impl Default for Strobe {
    fn default() -> Self {
        Self::new(
            Duration::from_secs(8),
            0.25,
            16,
            Duration::from_secs_f64(1.0 / ((1 << 1) as f64)),
            1.0 / ((1 << 2) as f64),
            RGB::from_code(0xFFFFFF, RGBOrder::RGB),
        )
    }
}
