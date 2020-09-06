//! # Rainbow

use super::*;

use crate::ds::{collections::frame::Frame, rgb::RGB};
use crate::util::info::Info;

use std::time::Duration;

/// Presents some info about `Rainbow` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RainbowInfo();

impl RainbowInfo {
    /// Creates a new `RainbowInfo` object.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Info for RainbowInfo {
    fn name(&self) -> String {
        "Rainbow".to_owned()
    }

    fn details(&self) -> String {
        "Classic RGB rainbow puke that we all know and love but instead of displaying on a fancy RGB keyboard it's just these stupid LEDs puking out everything".to_owned()
    }
}

/// Struct for animating the classic RGB rainbow puke that we all know and love
/// but instead of displaying on a fancy RGB keyboard it's just these stupid
/// LEDs puking out everything.
#[derive(Debug, Clone)]
pub struct Rainbow {
    time_remaining: Duration,
    frame: Frame,

    hue: f32,
    saturation: f32,
    value: f32,
    dh: f32,

    arc: f32,
    step: usize,
}

impl Rainbow {
    /// Creates a new `Rainbow` object.
    ///
    /// # Parameters
    ///
    /// * `duration` - The length of time this animation will run.
    /// * `rainbow_length` - The length of time it takes for the rainbow to fully cycle through all the LEDs.
    /// * `brightness` - The brightness value to use. Should be in range [0, 1].
    /// * `size` - The number of LEDs this animation will animate for.
    /// * `saturation` - The saturation to be used for all colors generated by HSV values.
    /// * `value` - The value to be used for all colors generated by HSV values.
    /// * `arc` - The amount of the arc between hue 0 and 360 (mapped to the range [0, 1] for this parameter) that is displayed across all the LEDs. NOTE: The arc can be larger than one, and can be as large as you like. For example an arc value of 2 would mean that there are 2 full rainbows visible across the LEDs.
    /// * `step` - The number of LEDs in a row that keep the same color before moving on to the next color. E.g if a step of 1 yields an LED array of \[1, 2, 3, 4\], then a step of 2 yields an array of \[1, 1, 2, 2\].
    pub fn new(
        duration: Duration,
        rainbow_length: Duration,
        brightness: f32,
        size: usize,
        saturation: f32,
        value: f32,
        arc: f32,
        step: usize,
    ) -> Self {
        Self {
            time_remaining: duration,
            frame: Frame::new(None, brightness, size),

            hue: 0.0,
            saturation,
            value,
            dh: 360.0 / rainbow_length.as_secs_f32(),

            arc,
            step,
        }
    }
}

impl Animation for Rainbow {
    fn update(&mut self, dt: Duration) {
        self.time_remaining = if let Some(d) = self.time_remaining.checked_sub(dt) {
            d
        } else {
            Duration::new(0, 0)
        };

        self.hue += self.dh * dt.as_secs_f32();

        if self.hue >= 360.0 {
            self.hue -= 360.0;
        }

        let len = self.frame.len() as f32;
        let brightness = self.frame.brightness();
        for (i, led) in self.frame.iter_mut().enumerate() {
            let step = i as f32 / self.step as f32;
            let step = step.floor();
            let step = step * (self.step as f32);
            let step = step / len;
            let step = step * 360.0 * self.arc;
            *led = RGB::from_hsv(self.hue + step, self.saturation, self.value).scale(brightness);
        }
    }

    fn frame(&self) -> &Frame {
        return &self.frame;
    }

    fn time_remaining(&self) -> Duration {
        self.time_remaining
    }
}
