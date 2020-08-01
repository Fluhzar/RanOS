//! # Breath

use super::*;

use crate::util::rgb::RGB;

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
pub struct Breath {
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
    /// * `breath_duration` - The duration a single color is drawn for, from
    ///                       black up to full color back down to black.
    /// * `order` - A given order that the animation cycles through.
    pub fn new(breath_duration: Duration, order: ColorOrder) -> Self {
        Self {
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
    fn update(&mut self, dt: Duration, frame: &mut [RGB]) {
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

        for led in frame {
            *led = self.current_color.scale(self.brightness);
        }
    }
}
