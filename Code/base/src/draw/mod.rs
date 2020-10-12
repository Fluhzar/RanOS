//! # Draw
//!
//! This module contains the types that will "draw" to the LEDs.
//!
//! There are two drawers defined in this module, one being the actual drawer
//! that will draw the colors to physical LEDs connected to a Raspberry Pi, and
//! the second is an emulated LED setup that draws to "LEDs" on the terminal
//! with a configurable number of "LEDs" per row.

pub use null_draw::{NullDraw, NullDrawBuilder, NullDrawInfo};
pub use term_draw::{TermDraw, TermDrawBuilder, TermDrawInfo};

#[cfg(target_os = "linux")]
pub use pi_draw::{
    APA102CPiDraw, APA102CPiDrawBuilder, APA102CPiDrawInfo,
    SK9822PiDraw, SK9822PiDrawBuilder, SK9822PiDrawInfo,
};

use std::time::Instant;
use std::{fmt, ops};

use crate::animation::Animation;
use crate::util::{Info, Timer};

pub mod null_draw;
pub mod term_draw;

#[cfg(target_os = "linux")]
pub mod pi_draw;

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
    fn run(&mut self) -> Vec<Box<dyn Animation>>;

    /// Returns the statistics tracking object.
    fn stats(&self) -> DrawStats;
}

/// Defines the behavior of a builder of a type that implements [`Draw`][0].
///
/// Optionally allows the setting of a timer for the built object. If the parameter is not supplied, `Timer::new(None)` will
/// likely be used as default though this behavior is implementation defined.
///
/// [0]: trait.Draw.html
pub trait DrawBuilder {
    /// Sets the `timer` parameter of the object.
    /// 
    /// # !!! IMPORTANT NOTE !!!
    /// 
    /// Since rust doesn't let you create objects from traits with functions referencing `Self` in the return parameter, this
    /// function instead returns `Box<dyn DrawBuilder>`. Once this function is called, the return value cannot be called with
    /// any of the implementing type's functions.
    /// 
    /// An example of this issue would be something akin to the following:
    /// 
    /// ```
    /// use base::draw::{DrawBuilder, TermDrawBuilder};
    /// let draw = TermDrawBuilder::default()
    ///     .timer(Timer::new(None))
    ///     .max_width(8) // ERROR: This will fail because the return type is `Box<dyn DrawBuilder>` and not `Box<TermDrawBuilder>`
    ///     .build();
    /// ```
    fn timer(self: Box<Self>, timer: Timer) -> Box<dyn DrawBuilder>;

    /// Consumes the builder and returns a built [`Draw`][0] object.
    ///
    /// [0]: trait.Draw.html
    fn build(self: Box<Self>) -> Box<dyn Draw>;
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
#[cfg(target_os = "linux")]
pub fn draw_info() -> Vec<Box<dyn Info>> {
    vec![
        APA102CPiDrawInfo::new(),
        TermDrawInfo::new(),
        NullDrawInfo::new(),
    ]
}

/// Returns a `Vec` of drawer `Info` objects
#[cfg(not(target_os = "linux"))]
pub fn draw_info() -> Vec<Box<dyn Info>> {
    vec![TermDrawInfo::new(), NullDrawInfo::new()]
}

/// Attempts to parse the given `String` into a `Draw` object, returning `None`
/// on failure.
#[cfg(target_os = "linux")]
pub fn match_draw<T>(s: T) -> Option<Box<dyn Draw>>
where
    T: std::ops::Deref<Target = str>,
{
    let s = s.to_lowercase();

    if s == APA102CPiDrawInfo::new().name().to_lowercase() {
        Some(Box::new(APA102CPiDraw::default()))
    } else if s == TermDrawInfo::new().name().to_lowercase() {
        println!("{}", "\x1B[2J"); // ANSI clear screen code
        Some(Box::new(TermDraw::default()))
    } else if s == NullDrawInfo::new().name().to_lowercase() {
        Some(Box::new(NullDraw::default()))
    } else {
        None
    }
}

/// Attempts to parse the given `String` into a `Draw` object, returning `None`
/// on failure.
#[cfg(not(target_os = "linux"))]
pub fn match_draw<T>(s: T) -> Option<Box<dyn Draw>>
where
    T: std::ops::Deref<Target = str>,
{
    let s = s.to_lowercase();

    if s == TermDrawInfo::new().name().to_lowercase() {
        println!("{}", "\x1B[2J"); // ANSI clear screen code
        Some(Box::new(TermDraw::default()))
    } else if s == NullDrawInfo::new().name().to_lowercase() {
        Some(Box::new(NullDraw::default()))
    } else {
        None
    }
}
