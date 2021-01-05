//! # Terminal Draw

use colored::Colorize;
use ranos_animation::AnimationState;
use std::collections::VecDeque;

use ranos_ds::collections::frame::Frame;
use ranos_core::{Info, Timer};

use super::*;

/// Presents some info about `TermDraw` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TermDrawInfo();

impl Info for TermDrawInfo {
    fn new() -> Box<dyn Info>
    where
        Self: Sized,
    {
        Box::new(TermDrawInfo::default())
    }

    fn name(&self) -> String {
        "TermDraw".to_owned()
    }

    fn details(&self) -> String {
        "Emulates an LED display by writing whitespace with specified colored backgrounds to a terminal that supports full RGB colors.".to_owned()
    }
}

/// Emulates an LED display by writing whitespace with specified colored
/// backgrounds to a terminal that supports full RGB colors.
///
/// LEDs are displayed in a rectangular grid with 1 LED's worth of space between
/// each column and row.build_helper
#[derive(Debug)]
pub struct TermDraw {
    max_width: usize,

    queue: VecDeque<Box<dyn Animation>>,
    timer: Timer,
    frame: Frame,

    stats: DrawStats,
}

impl TermDraw {
    /// Returns a builder for this struct.
    ///
    /// # Example
    ///
    /// ```
    /// # use base::draw::{Draw, DrawBuilder, TermDraw, TermDrawBuilder};
    /// let drawer = TermDraw::builder().build();
    /// ```
    pub fn builder() -> Box<TermDrawBuilder> {
        TermDrawBuilder::new()
    }

    /// Creates a new `TermDraw` object.
    ///
    /// # Parameters
    ///
    /// * `max_width` - The maximum number of LEDs to draw per line in the
    /// terminal. E.g. if there are 256 LEDs to draw and a `max_width` of 16,
    /// then a 16x16 grid will be displayed.
    pub fn new(max_width: usize, timer: Timer, brightness: f32, size: usize) -> Self {
        Self {
            max_width,

            queue: VecDeque::new(),
            timer,
            frame: Frame::new(brightness, size),

            stats: DrawStats::new(),
        }
    }

    fn write_frame(&mut self) {
        // Create output string with enough capacity to minimize reallocations of memory for growing the string's capacity
        let mut output =
            String::with_capacity(self.frame.len() * 4 + (self.frame.len() / self.max_width) * 2);
        output.push_str("\x1B[1;1H"); // ANSI "move cursor to upper-left corner" code

        // Loop through the enumerated RGB values
        for (i, led) in self.frame.iter().enumerate() {
            // Check if max width has been reached on the current row
            if i % self.max_width == 0 {
                output = format!("{}\n\n", output);
            }

            // Scale the color and print it to the output
            let led = led.scale(self.frame.brightness());
            output = format!(
                "{}{}  ",
                output,
                "  ".on_truecolor(led.red(), led.green(), led.blue())
            );
        }

        println!("{}", output);
    }
}

impl Draw for TermDraw {
    fn push_queue(&mut self, a: Box<dyn Animation>) {
        self.queue.push_back(a);
    }

    fn queue_len(&self) -> usize {
        self.queue.len()
    }

    fn run(&mut self) -> Vec<Box<dyn Animation>> {
        self.timer.reset();
        self.stats.reset();

        let mut out = Vec::new();

        while let Some(mut ani) = self.queue.pop_front() {
            loop {
                match ani.render_frame(&mut self.frame, self.timer.ping()) {
                    AnimationState::Continue | AnimationState::ErrRetry => (),
                    AnimationState::Last | AnimationState::ErrFatal => break,
                }
                self.write_frame();

                self.stats.inc_frames();
            }

            self.stats.set_num(self.frame.len());
            self.stats.end();

            out.push(ani);
        }

        out
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}

/// Builder for [`TermDraw`][0].
///
/// Allows for optional setting of the `max_width` and `timer` parameters of [`TermDraw::new`][1]. If a parameter is not
/// supplied, a default value will be inserted in its place. This default parameter will be the same as the corresponding
/// default parameter seen in [`TermDraw::default`][2].
///
/// [0]: struct.TermDraw.html
/// [1]: struct.TermDraw.html#method.new
/// [2]: struct.TermDraw.html#method.default
#[derive(Default, Copy, Clone)]
pub struct TermDrawBuilder {
    max_width: Option<usize>,
}

impl TermDrawBuilder {
    /// Creates a new builder
    pub fn new() -> Box<Self> {
        Box::new(Default::default())
    }

    /// Sets the maximum number of LEDs to draw per line.
    ///
    /// If this parameter is not set, the default value of `8` will be used instead.
    pub fn max_width(mut self: Box<Self>, width: usize) -> Box<Self> {
        self.max_width = Some(width);

        self
    }
}

impl DrawBuilder for TermDrawBuilder {
    fn build(self: Box<Self>, timer: Timer, brightness: f32, size: usize) -> Box<dyn Draw> {
        Box::new(TermDraw::new(
            self.max_width.unwrap_or(8),
            timer,
            brightness,
            size,
        ))
    }
}
