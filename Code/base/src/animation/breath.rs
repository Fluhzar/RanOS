//! # Breath

use std::time::Duration;

use crate::ds::{const_val::ConstVal, rgb::RGB};
use crate::util::info::Info;

use super::*;

/// Presents some info about `Breath` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BreathInfo();

impl Info for BreathInfo {
    fn new() -> Box<dyn Info>
    where
        Self: Sized,
    {
        Box::new(BreathInfo::default())
    }

    fn name(&self) -> String {
        "Breath".to_owned()
    }

    fn details(&self) -> String {
        "Animates a breathing display that will either walk through a provided list of colors or select random colors, each color fading along a parabolic curve from black to the chosen color and back down to black.".to_owned()
    }
}

/// Color order used by `Breath`, can be a predetermined order or a random order.
#[derive(Debug, Clone)]
pub enum ColorOrder {
    /// Order determined by random colors generated when needed.
    Random,
    /// Order determined by the associated data which is looped through sequentially.
    Ordered(Vec<RGB>),
}

/// Struct for an animated breathing display that will either walk through a
/// provided list of colors or select random colors, each color fading along a
/// parabolic curve from black to the chosen color and back down to black.
#[derive(Debug)]
pub struct Breath {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,
    frame: Frame,

    order: ColorOrder,
    ind: usize,
    current_color: RGB,

    brightness: f32,

    acc: ConstVal<f32>,
    vel: f32,
    vel0: ConstVal<f32>,
}

impl Breath {
    /// Creates new `Breath` object
    ///
    /// # Parameters
    ///
    /// * `runtime` - The length of time this animation will run.
    /// * `breath_duration` - The duration a single color is drawn for, from black up to full color back down to black.
    /// * `brightness` - The brightness value to use. Should be in range [0, 1].
    /// * `size` - The number of LEDs this animation will animate for.
    /// * `order` - A given order that the animation cycles through.
    pub fn new(
        runtime: Duration,
        breath_duration: Duration,
        brightness: f32,
        size: usize,
        order: ColorOrder,
    ) -> Self {
        Self {
            runtime: ConstVal::new(runtime),
            time_remaining: runtime,
            frame: Frame::new(None, brightness, size),

            order: order.clone(),
            ind: 0,
            current_color: match order {
                ColorOrder::Ordered(v) => v[0],
                ColorOrder::Random => RGB::random(),
            },

            brightness: 0.0,

            acc: ConstVal::new(-8.0 / breath_duration.as_secs_f32().powi(2)),
            vel: 4.0 / breath_duration.as_secs_f32(),
            vel0: ConstVal::new(4.0 / breath_duration.as_secs_f32()),
        }
    }
}

impl Animation for Breath {
    fn update(&mut self, dt: Duration) {
        self.time_remaining = if let Some(d) = self.time_remaining.checked_sub(dt) {
            d
        } else {
            Duration::new(0, 0)
        };

        self.vel += self.acc.get() * dt.as_secs_f32();
        self.brightness += self.vel * dt.as_secs_f32();

        if self.brightness <= 0.0 && self.vel < 0.0 {
            self.brightness = 0.0;
            self.vel = *self.vel0.get();

            if let ColorOrder::Ordered(v) = &self.order {
                self.ind += 1;
                self.ind %= v.len();
                self.current_color = v[self.ind];
            } else {
                self.current_color = RGB::random();
            }
        }

        for led in self.frame.iter_mut() {
            *led = self.current_color;
        }
    }

    fn frame(&self) -> &Frame {
        &self.frame
    }

    fn time_remaining(&self) -> Duration {
        self.time_remaining
    }
}

impl Clone for Breath {
    /// Clones and resets `self` so it is as if it were just created with `Breath::new`.
    fn clone(&self) -> Self {
        let order = self.order.clone();
        let color = match &order {
            ColorOrder::Ordered(v) => v[0],
            ColorOrder::Random => RGB::random(),
        };

        Self {
            runtime: self.runtime.clone(),
            time_remaining: *self.runtime.get(),
            frame: self.frame.clone(),

            order,
            ind: 0,
            current_color: color,

            brightness: self.brightness,

            acc: self.acc.clone(),
            vel: *self.vel0.get(),
            vel0: self.vel0.clone(),
        }
    }
}

impl Default for Breath {
    fn default() -> Self {
        Self::new(
            Duration::from_secs(18),
            Duration::from_secs(3),
            0.25,
            16,
            ColorOrder::Ordered(vec![
                RGB::from_hsv(0.0, 1.0, 1.0),
                RGB::from_hsv(30.0, 1.0, 1.0),
                RGB::from_hsv(60.0, 1.0, 1.0),
                RGB::from_hsv(120.0, 1.0, 1.0),
                RGB::from_hsv(210.0, 1.0, 1.0),
                RGB::from_hsv(280.0, 1.0, 1.0),
            ])
        )
    }
}
