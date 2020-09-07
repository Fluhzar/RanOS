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
use std::{fmt, ops};

pub mod pi_draw;
pub use pi_draw::{APA102CPiDraw, APA102CPiDrawInfo, SK9822PiDraw, SK9822PiDrawInfo};

pub mod term_draw;
pub use term_draw::{TermDraw, TermDrawInfo};

pub mod null_draw;
pub use null_draw::{NullDraw, NullDrawInfo};

use crate::util::Info;

/// Result type used for [`Draw::run`][0], indicating the success of the
/// function. Usually `Err` is returned when `SIGINT` is handled, shutting the
/// system down.
///
/// [0]: ./trait.Draw.html#method.Run
pub type Result = std::result::Result<Vec<Box<dyn Animation>>, String>;

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
    num: usize,
}

impl DrawStats {
    /// Creates a new statistics object. Start time is set to the the instant
    /// that this object is created.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            end: Instant::now(),
            frames: 0,
            num: 0,
        }
    }

    /// Sets the number of LEDs tracked by the owner of this stats tracker.
    pub fn set_num(&mut self, num: usize) {
        self.num = num;
    }

    /// Resets the `DrawStats` to a brand-new state, as if it were just initialized.
    pub fn reset(&mut self) {
        *self = DrawStats::new();
    }

    /// Increments the number of frames.
    #[inline]
    pub fn inc_frames(&mut self) {
        self.frames += 1;
    }

    /// Sets the end time.
    ///
    /// This method may be called multiple times during the life of the object, as it simply saves the [`Instant`][0] when this
    /// method was called. Calling it again therefore only updates the saved [`Instant`][0] to the current value.
    ///
    /// [0]: https://doc.rust-lang.org/std/time/struct.Instant.html
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
        let duration = self.end.duration_since(self.start).as_secs_f64();
        let num_frames = self.frames as f64;
        let num_leds = self.num as f64;
        let fps = num_frames / duration;
        let led_rate = (num_frames * num_leds) / duration;
        write!(
            f,
            "Drawing statistics: \nDuration: {}s \tFrame count: {} \tLED count: {} \nAvg frame rate: {} Hz \nAvg time per LED: {} Hz",
            duration,
            num_frames,
            num_leds,
            fps,
            led_rate
        )
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
            num: if self.num > rhs.num {
                self.num
            } else {
                rhs.num
            },
        }
    }
}

/// Like ops::Add, but assigns to self.
impl ops::AddAssign<DrawStats> for DrawStats {
    fn add_assign(&mut self, rhs: Self) {
        self.start += rhs.start - self.end;
        self.end = rhs.end;
        self.frames += rhs.frames;
        self.num = if self.num > rhs.num {
            self.num
        } else {
            rhs.num
        };
    }
}

/// Returns a `Vec` of drawer `Info` objects
pub fn draw_info() -> Vec<Box<dyn Info>> {
    vec![
        APA102CPiDrawInfo::new(),
        TermDrawInfo::new(),
        NullDrawInfo::new(),
    ]
}

pub fn match_draw<T>(s: T) -> Option<Box<dyn Draw>>
where
    T: std::ops::Deref<Target=str>
{
    let s = s.to_lowercase();

    if s == APA102CPiDrawInfo::new().name().to_lowercase() {
        Some(Box::new(APA102CPiDraw::default()))
    } else if s == TermDrawInfo::new().name().to_lowercase() {
        Some(Box::new(TermDraw::default()))
    } else if s == NullDrawInfo::new().name().to_lowercase() {
        Some(Box::new(NullDraw::default()))
    } else {
        None
    }
}
