//! # Strobe

use super::*;

use crate::ds::rgb::RGB;

use std::time::Duration;

/// Struct for animating a flickering light similar to strobe lights one might
/// see at concerts or otherwise.
/// 
/// Allows for PWM-like control with the ability to define the period and
/// "duty cycle" of the strobe.
/// 
/// The `period` is simply the amount of time before the strobe pattern repeats,
/// and the `duty cycle` being a value in the range of [0, 1) representing the
/// percentage of time that the LEDs are on within the `period`.
#[derive(Debug, Clone)]
pub struct Strobe {
    time_remaining: Duration,
    frame: Frame,

    period: f64,
    duty: f64,

    color: RGB,

    time: f64,
}

impl Strobe {
    /// Creates a new `Strobe` animation.
    /// 
    /// # Parameters
    /// 
    /// - `duration` - The length of time this animation will run for.
    /// - `brightness` - The brightness value to use. Should be in range [0, 1].
    /// - `size` - The number of LEDs this animation will animate for.
    /// - `period` - The period of time before the strobe animation repeats,
    /// typically a number less than 1.
    /// - `duty` - The percentage of time in the range of [0, 1) representing
    /// the percentage of time the LEDs are on within the `period`.
    pub fn new(duration: Duration, brightness: f32, size: usize, period: Duration, duty: f64, color: RGB) -> Self {
        let duty = duty.min(1.0).max(0.0);

        Self {
            time_remaining: duration,
            frame: Frame::new(None, brightness, size),

            period: period.as_secs_f64(),
            duty,

            color,

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
        self.time = (self.time + dt.as_secs_f64()) % self.period;

        // Convert the time to a fraction in the range [0, 1)
        let r = self.time / self.period;

        // Set the current color, based on how long it's been in the current cycle
        let color = if r < self.duty {
            self.color
        } else {
            RGB::new()
        };

        // Copy the colors to the frame
        for led in self.frame.iter_mut() {
            *led = color;
        }
    }

    fn frame(&self) -> &Frame {
        &self.frame
    }

    fn time_remaining(&self) -> Duration {
        self.time_remaining
    }
}
