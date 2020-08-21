//! # Frame

use crate::ds::rgb::{RGBOrder, RGB};
use std::io;
use std::slice::{Iter, IterMut};

/// A struct representing a single frame of color values and a brightness.
#[repr(C)]
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
            leds: vec![Default::default(); size],
        }
    }

    /// Attempts to read a `Frame` from the `reader`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the `reader` encounters an error while reading.
    pub fn read<R: io::Read>(reader: &mut R) -> io::Result<Frame> {
        use std::mem::size_of;

        let mut brightness_buf = [0_u8; size_of::<f32>()];
        reader.read_exact(&mut brightness_buf)?;
        let brightness = f32::from_ne_bytes(brightness_buf);

        let mut len_buf = [0_u8; size_of::<usize>()];
        reader.read_exact(&mut len_buf)?;
        let len = usize::from_ne_bytes(len_buf);

        let leds = RGB::read_n(reader, len, RGBOrder::RGB)?;

        Ok(Self { brightness, leds })
    }

    /// Attempts to write a `Frame` to the `writer`, returning the number of bytes written.
    ///
    /// # Errors
    ///
    /// This function returns an error if the `writer` encounters an error while writing.
    pub fn write<W: io::Write>(self, writer: &mut W) -> io::Result<usize> {
        let mut count = 0;

        let brightness_buf = self.brightness.to_ne_bytes();
        writer.write_all(&brightness_buf)?;
        count += brightness_buf.len();

        let len_buf = self.leds.len().to_ne_bytes();
        writer.write_all(&len_buf)?;
        count += len_buf.len();

        count += RGB::write_slice(&self.leds, writer, RGBOrder::RGB)?;

        Ok(count)
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

    /// Same as [`Frame::brightness_sk9822`][0], just a new name for niceness.
    ///
    /// [0]: ./struct.Frame.html#method.brightness_apa102c
    pub fn brightness_sk9822(&self) -> u8 {
        self.brightness_apa102c()
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
