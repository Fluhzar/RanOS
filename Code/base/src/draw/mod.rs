//! # Draw
//! 
//! This module contains the types that will "draw" to the LEDs.
//! 
//! There are two drawers defined in this module, one being the actual drawer
//! that will draw the colors to physical LEDs connected to a Raspberry Pi, and
//! the second is an emulated LED setup that draws to "LEDs" on the terminal
//! with a configurable number of "LEDs" per row.

use crate::animation::Animation;

use std::time::Instant;
use std::{ops, fmt};

#[cfg(feature = "pi_draw")]
pub mod pi_draw;

#[cfg(feature = "term_draw")]
pub mod term_draw;

pub mod null_draw;

/// Result type used for [`Draw::run`][0], indicating the success of the
/// function. Usually `Err` is returned when `SIGINT` is handled, shutting the
/// system down.
/// 
/// [0]: ./trait.Draw.html#method.Run
pub type Result = std::result::Result<(), String>;

/// Trait defining the ability to draw a frame of colors to LEDs.
pub trait Draw {
    /// Adds an [`Animation`][0] to the queue.
    /// 
    /// [0]: ../animation/trait.Animation.html
    fn push_queue(&mut self, a: Box<dyn Animation>);

    /// Returns the number of [`Animation`][0]s in the queue.
    /// 
    /// [0]: ../animation/trait.Animation.html
    fn queue_len(&self) -> usize;

    /// Draws the internal frame to its destination.
    fn run(&mut self) -> Result;

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
