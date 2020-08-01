//! # Terminal Draw

use super::*;

use crate::util::{frame::Frame, rgb::RGB};
use colored::Colorize;

/// Emulates an LED display by writing whitespace with specified colored
/// backgrounds to a terminal that supports full RGB colors.
/// 
/// LEDs are displayed in a rectangular grid with 1 LED's worth of space between
/// each column and row.build_helper
#[derive(Debug, Clone)]
pub struct TermDraw {
    max_width: usize,

    frame: Frame,
}

impl TermDraw {
    /// Creates a new `TermDraw` object.
    /// 
    /// # Parameters
    /// 
    /// * `max_width` - The maximum number of LEDs to draw per line in the
    ///                 terminal. E.g. if there are 256 LEDs to draw and a
    ///                 `max_width` of 16, then a 16x16 grid will be displayed.
    /// * `brightness` - Value in the range of \[0, 1\]. Note: the actual value
    ///                  sent to LEDs is an integer value in the range of
    ///                  \[0, 32).
    /// * `size` - The number of LEDs the drawer will draw to.
    pub fn new(max_width: usize, brightness: f32, size: usize) -> Self {
        Self {
            max_width,

            frame: Frame::new(brightness, size),
        }
    }
}

impl Draw for TermDraw {
    fn write_frame(&mut self) {
        let mut output = String::with_capacity(self.frame.len() * 3);
        output.push_str("\x1B[2J"); // ASCII clear screen code

        for (i, led) in self.frame.iter().enumerate() {
            // Check if max width has been reached on the current row
            if i % self.max_width == 0 {
                output = format!("{}\n\n", output);
            }

            let led = led.scale(self.frame.brightness());
            output = format!("{}{}  ", output, "  ".on_truecolor(led.red(), led.green(), led.blue()));
        }

        println!("{}", output);
    }

    fn as_slice(&self) -> &[RGB] {
        self.frame.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [RGB] {
        self.frame.as_mut_slice()
    }
}
