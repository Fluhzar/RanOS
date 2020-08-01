//! # Frame

use super::rgb::RGB;
use std::slice::{IterMut, Iter};

/// A struct representing a single frame of color values and a brightness.
#[derive(Debug, Default, Clone)]
pub struct Frame {
    brightness: f32,
    leds: Vec<RGB>,
}

impl Frame {
    /// Creates a new `Frame` from a given brightness and size.
    pub fn new(brightness: f32, size: usize) -> Self {
        Self {
            brightness,
            leds: vec![ Default::default() ; size ],
        }
    }

    /// Returns the brightness in range [0, 1]
    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    /// Returns the brightness in the format used by the APA102C LEDs, which is
    /// an integer format in the range [0, 31].
    pub fn brightness_apa102c(&self) -> u8 {
        (self.brightness.min(1.0).max(0.0) * 0x1F as f32) as u8
    }

    /// Returns the length of the internal buffer.
    pub fn len(&self) -> usize {
        self.leds.len()
    }

    /// Returns the internal buffer as a immutable slice.
    pub fn as_slice(&self) -> &[RGB] {
        &self.leds
    }

    /// Returns the internal buffer as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [RGB] {
        &mut self.leds
    }

    /// Returns an immutable iterator of the internal buffer.
    pub fn iter(&self) -> Iter<RGB> {
        self.leds.iter()
    }

    /// Returns an mutable iterator of the internal buffer.
    pub fn iter_mut(&mut self) -> IterMut<RGB> {
        self.leds.iter_mut()
    }
}
