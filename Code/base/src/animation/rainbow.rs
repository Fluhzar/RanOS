//! # Rainbow

use std::time::Duration;

use crate::ds::{collections::frame::Frame, const_val::ConstVal, rgb::RGB};
use crate::util::info::Info;

use super::*;

/// Presents some info about `Rainbow` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RainbowInfo();

impl Info for RainbowInfo {
    fn new() -> Box<dyn Info>
    where
        Self: Sized,
    {
        Box::new(RainbowInfo::default())
    }

    fn name(&self) -> String {
        "Rainbow".to_owned()
    }

    fn details(&self) -> String {
        "Classic RGB rainbow puke that we all know and love but instead of displaying on a fancy RGB keyboard it's just these stupid LEDs puking out everything.".to_owned()
    }
}

/// Struct for animating the classic RGB rainbow puke that we all know and love
/// but instead of displaying on a fancy RGB keyboard it's just these stupid
/// LEDs puking out everything.
#[derive(Debug)]
pub struct Rainbow {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,
    frame: Frame,

    hue: f32,
    sat: ConstVal<f32>,
    val: ConstVal<f32>,
    dh: ConstVal<f32>,

    arc: ConstVal<f32>,
    step: ConstVal<usize>,
}

impl Rainbow {
    /// Creates a new `Rainbow` object.
    ///
    /// # Parameters
    ///
    /// * `runtime` - The length of time this animation will run.
    /// * `rainbow_length` - The length of time it takes for the rainbow to fully cycle through all the LEDs.
    /// * `brightness` - The brightness value to use. Should be in range [0, 1].
    /// * `size` - The number of LEDs this animation will animate for.
    /// * `sat` - The saturation to be used for all colors generated by HSV values.
    /// * `val` - The value to be used for all colors generated by HSV values.
    /// * `arc` - The amount of the arc between hue 0 and 360 (mapped to the range [0, 1] for this parameter) that is displayed across all the LEDs. NOTE: The arc can be larger than one, and can be as large as you like. For example an arc value of 2 would mean that there are 2 full rainbows visible across the LEDs.
    /// * `step` - The number of LEDs in a row that keep the same color before moving on to the next color. E.g if a step of 1 yields an LED array of \[1, 2, 3, 4\], then a step of 2 yields an array of \[1, 1, 2, 2\].
    pub fn new(
        runtime: Duration,
        rainbow_length: Duration,
        brightness: f32,
        size: usize,
        sat: f32,
        val: f32,
        arc: f32,
        step: usize,
    ) -> Self {
        Self {
            runtime: ConstVal::new(runtime),
            time_remaining: runtime,
            frame: Frame::new(None, brightness, size),

            hue: 0.0,
            sat: ConstVal::new(sat),
            val: ConstVal::new(val),
            dh: ConstVal::new(360.0 / rainbow_length.as_secs_f32()),

            arc: ConstVal::new(arc),
            step: ConstVal::new(step),
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

        self.hue += self.dh.get() * dt.as_secs_f32();

        if self.hue >= 360.0 {
            self.hue -= 360.0;
        }

        let len = self.frame.len() as f32;
        for (i, led) in self.frame.iter_mut().enumerate() {
            let step = i as f32 / *self.step.get() as f32;
            let step = step.floor();
            let step = step * (*self.step.get() as f32);
            let step = step / len;
            let step = step * 360.0 * self.arc.get();
            *led = RGB::from_hsv(self.hue + step, *self.sat.get(), *self.val.get())
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
        self.hue = 0.0;
    }
}

impl Default for Rainbow {
    fn default() -> Self {
        Self::new(
            Duration::from_secs(16),
            Duration::from_secs(2),
            0.25,
            16,
            1.0,
            1.0,
            1.0,
            1,
        )
    }
}
