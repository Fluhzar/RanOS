//! # Draw
//!
//! This module contains the types that will "draw" to the LEDs.
//!
//! There are two drawers defined in this module, one being the actual drawer
//! that will draw the colors to physical LEDs connected to a Raspberry Pi, and
//! the second is an emulated LED setup that draws to "LEDs" on the terminal
//! with a configurable number of "LEDs" per row.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

extern crate ranos_display;
extern crate ranos_core;

pub use null_draw::{NullDraw, NullDrawBuilder};
pub use term_draw::{TermDraw, TermDrawBuilder};

#[cfg(target_os = "linux")]
pub use pi_draw::{
    APA102CPiDraw, APA102CPiDrawBuilder,
    SK9822PiDraw, SK9822PiDrawBuilder,
};

use std::time::Instant;
use std::{fmt, ops};

use ranos_display::{Display, DisplayBuilder};
use ranos_core::Timer;

pub mod null_draw;
pub mod term_draw;

#[cfg(target_os = "linux")]
pub mod pi_draw;

/// Trait defining the ability to draw a frame of colors to LEDs.
pub trait Draw {
    /// Draws the internal frame to its destination.
    fn run(&mut self);

    /// Returns the statistics tracking object.
    fn stats(&self) -> DrawStats;
}

/// Defines the behavior of a builder of a type that implements [`Draw`][crate::Draw].
///
/// Note: As the trait's functions return `Box<dyn DrawBuilder>` rather than `Box<Self>`, be sure to set any parameters for the
/// specific `Draw`-implementing type you're using before calling these functions, as the original type will be inaccessible
/// after calling one of the functions from this trait.
#[typetag::serde(tag = "type")]
pub trait DrawBuilder {
    /// Sets the timer parameter from a pre-built object.
    fn timer(self: Box<Self>, timer: Timer) -> Box<dyn DrawBuilder>;

    /// Add a builder for a display that will be built at the same time as this builder.
    ///
    /// Be sure to add animations to the display builder before adding it to the drawer as it will be inaccessible afterwards.
    ///
    /// Note: Multiple [`DisplayBuilder`](ranos_display::DisplayBuilder)s can be added.
    fn display(self: Box<Self>, display: DisplayBuilder) -> Box<dyn DrawBuilder>;

    /// Builds [`Draw`][crate::Draw] object, returning it boxed up.
    fn build(self: Box<Self>) -> Box<dyn Draw>;
}

#[cfg(test)]
mod builder_test {
    use std::collections::VecDeque;
    use ranos_core::Timer;
    use crate::{DrawBuilder, NullDraw, NullDrawBuilder};

    #[test]
    fn test_serialize() {
        let builder: Box<dyn DrawBuilder> = NullDraw::builder();

        let data = serde_json::ser::to_string(&builder).unwrap();

        assert_eq!(data, r#"{"type":"NullDrawBuilder","timer":{"target_dt":null},"displays":[]}"#);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"{"type":"NullDrawBuilder","timer":{"target_dt":null},"displays":[]}"#;

        let data: NullDrawBuilder = serde_json::de::from_str(input).unwrap();

        assert_eq!(data.timer, Timer::new(None));
        assert_eq!(data.displays.len(), 0);
    }
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
    /// This method may be called multiple times during the life of the object,
    /// as it simply saves the [`Instant`] when this method was called. Calling
    /// it again therefore only updates the saved [`Instant`] to the current value.
    ///
    /// [`Instant`]: std::time::Instant
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
