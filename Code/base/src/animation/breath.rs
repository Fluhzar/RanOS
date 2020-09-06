//! # Breath

use super::*;

use crate::ds::rgb::RGB;
use crate::util::info::Info;

use std::time::Duration;

/// Presents some info about `Breath` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BreathInfo();

impl Info for BreathInfo {
    fn new() -> Box<dyn Info>
    where
        Self: Sized
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
    runtime: Duration,
    time_remaining: Duration,
    frame: Frame,

    order: ColorOrder,
    ind: usize,
    current_color: RGB,

    brightness: f32,

    acc: f32,
    vel: f32,
    vel0: f32,
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
            runtime,
            time_remaining: runtime,
            frame: Frame::new(None, brightness, size),

            order: order.clone(),
            ind: 0,
            current_color: match order {
                ColorOrder::Ordered(v) => v[0],
                ColorOrder::Random => RGB::random(),
            },

            brightness: 0.0,

            acc: -8.0 / breath_duration.as_secs_f32().powi(2),
            vel: 4.0 / breath_duration.as_secs_f32(),
            vel0: 4.0 / breath_duration.as_secs_f32(),
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

        self.vel += self.acc * dt.as_secs_f32();
        self.brightness += self.vel * dt.as_secs_f32();

        if self.brightness <= 0.0 && self.vel < 0.0 {
            self.brightness = 0.0;
            self.vel = self.vel0;

            if let ColorOrder::Ordered(v) = &self.order {
                self.ind += 1;
                self.ind %= v.len();
                self.current_color = v[self.ind];
            } else {
                self.current_color = RGB::random();
            }
        }

        for led in self.frame.iter_mut() {
            *led = self.current_color.scale(self.brightness);
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
            runtime: self.runtime,
            time_remaining: self.runtime,
            frame: self.frame.clone(),

            order,
            ind: 0,
            current_color: color,

            brightness: self.brightness,

            acc: self.acc,
            vel: self.vel0,
            vel0: self.vel0,
        }
    }
}
