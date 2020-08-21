//! # Breath

use super::*;

use crate::ds::rgb::RGB;

use std::time::Duration;

/// Color order used by `Breath`, can be a predetermined order or a random order.
#[derive(Debug, Clone)]
pub enum ColorOrder {
    /// Order determined by random colors generated when needed.
    Random,
    /// Order determined by the associated data which is looped through sequentially.
    Ordered(Vec<RGB>),
}

/// Struct for an animated breathing display that will
#[derive(Debug, Clone)]
pub struct Breath {
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
    /// * `duration` - The length of time this animation will run.
    /// * `breath_duration` - The duration a single color is drawn for, from black up to full color back down to black.
    /// * `brightness` - The brightness value to use. Should be in range [0, 1].
    /// * `size` - The number of LEDs this animation will animate for.
    /// * `order` - A given order that the animation cycles through.
    pub fn new(
        duration: Duration,
        breath_duration: Duration,
        brightness: f32,
        size: usize,
        order: ColorOrder,
    ) -> Self {
        Self {
            time_remaining: duration,
            frame: Frame::new(brightness, size),

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
        return &self.frame;
    }

    fn time_remaining(&self) -> Duration {
        self.time_remaining
    }
}
