//! # Draw
//! 
//! This module contains the types that will "draw" to the LEDs.
//! 
//! There are two drawers defined in this module, one being the actual drawer
//! that will draw the colors to physical LEDs connected to a Raspberry Pi, and
//! the second is an emulated LED setup that draws to "LEDs" on the terminal
//! with a configurable number of "LEDs" per row.

use crate::util::rgb::RGB;

use std::time::Instant;
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

    /// Returns the statistics tracking object.
    fn stats(&self) -> DrawStats;
}

/// Type for tracking statistics about the drawing.
#[derive(Debug, Copy, Clone)]
pub struct DrawStats {
    start: Instant,
    end: Instant,
    frames: usize,
}

impl DrawStats {
    /// Creates a new statistics object. Start time is set to the the instant
    /// that this object is created.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            end: Instant::now(),
            frames: 0,
        }
    }

    /// Increments the number of frames.
    #[inline]
    pub fn inc_frames(&mut self) {
        self.frames += 1;
    }

    /// Sets the end time.
    #[inline]
    pub fn end(&mut self) {
        self.end = Instant::now();
    }
}

/// # Pretty printing.
/// 
/// Displays the duration that statistics were tracked for, the number of frames
/// tracked, and the average frames per second across the entire run-time.
impl fmt::Display for DrawStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Drawing statistics: \nDuration: {}s \tFrame count: {} \nAvg frame rate: {} fps", self.end.duration_since(self.start).as_secs_f64(), self.frames, self.frames as f64 / (self.end - self.start).as_secs_f64())
    }
}

/// Adds two statistic objects together, accounting for the time between the end
/// of one statistic and the start of the next.
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

/// Like ops::Add, but assigns to self.
impl ops::AddAssign<DrawStats> for DrawStats {
    fn add_assign(&mut self, rhs: Self) {
        self.start += rhs.start - self.end;
        self.end = rhs.end;
        self.frames += rhs.frames;
    }
}
