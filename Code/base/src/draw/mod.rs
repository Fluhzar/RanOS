//! # Draw
//! 
//! This module contains the types that will "draw" to the LEDs.
//! 
//! There are two drawers defined in this module, one being the actual drawer
//! that will draw the colors to physical LEDs connected to a Raspberry Pi, and
//! the second is an emulated LED setup that draws to "LEDs" on the terminal
//! with a configurable number of "LEDs" per row.

use crate::util::rgb::RGB;

use std::time::{Duration, Instant};
use std::{ops, fmt};

#[cfg(feature = "pi_draw")]
pub mod pi_draw;

#[cfg(feature = "term_draw")]
pub mod term_draw;

pub mod null_draw;

/// Trait defining the ability to draw a frame of colors to LEDs.
pub trait Draw {
    /// Draws the internal frame to its destination.
    fn write_frame(&mut self) -> Result<(), String>;

    /// Returns the internal frame as a immutable slice.
    fn as_slice(&self) -> &[RGB];

    /// Returns the internal frame as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [RGB];

    fn stats(&self) -> DrawStats;
}

#[derive(Debug, Copy, Clone)]
pub struct DrawStats {
    start: Instant,
    end: Instant,
    frames: usize,
}

impl DrawStats {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            end: Instant::now(),
            frames: 0,
        }
    }

    #[inline]
    pub fn inc_frames(&mut self) {
        self.frames += 1;
    }

    #[inline]
    pub fn end(&mut self) {
        self.end = Instant::now();
    }
}

impl fmt::Display for DrawStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Drawing statistics: \nDuration: {}s \tFrame count: {} \nAvg frame rate: {} fps", self.end.duration_since(self.start).as_secs_f64(), self.frames, self.frames as f64 / (self.end - self.start).as_secs_f64())
    }
}

impl ops::Add<DrawStats> for DrawStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            start: self.start + (rhs.start - self.end),
            end: rhs.end,
            frames: self.frames + rhs.frames,
        }
    }
}

impl ops::AddAssign<DrawStats> for DrawStats {
    fn add_assign(&mut self, rhs: Self) {
        self.start += rhs.start - self.end;
        self.end = rhs.end;
        self.frames += rhs.frames;
    }
}
