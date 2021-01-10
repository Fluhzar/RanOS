//! # Terminal Draw

use std::collections::{HashMap, VecDeque};

use colored::Colorize;
use serde::{Serialize, Deserialize};

use ranos_core::Timer;
use ranos_display::DisplayState;

use super::*;

/// Builder for [`TermDraw`](TermDraw).
#[derive(Serialize, Deserialize)]
#[serde(rename = "TermDraw")]
pub struct TermDrawBuilder {
    max_width: usize,
    timer: Timer,
    displays: VecDeque<DisplayBuilder>,
}

impl TermDrawBuilder {
    /// Sets the maximum number of LEDs to draw per line.
    ///
    /// If this parameter is not set, the default value of `8` will be used instead.
    pub fn max_width(mut self: Box<Self>, width: usize) -> Box<Self> {
        self.max_width = width;

        self
    }

    /// Sets the timer.
    pub fn timer(mut self: Box<Self>, timer: Timer) -> Box<Self> {
        self.timer = timer;

        self
    }

    /// Add a builder for a display that will be built at the same time as this builder.
    ///
    /// Be sure to add animations to the display builder before adding it to the drawer as it will be inaccessible afterwards.
    ///
    /// Note: Multiple [`DisplayBuilder`](ranos_display::DisplayBuilder)s can be added.
    pub fn display(mut self: Box<Self>, display: DisplayBuilder) -> Box<Self> {
        self.displays.push_back(display);

        self
    }

    /// Constructs a [`TermDraw`](TermDraw) object.
    pub fn build(self: Box<Self>) -> TermDraw {
        TermDraw::from_builder(self)
    }
}

#[typetag::serde]
impl DrawBuilder for TermDrawBuilder {
    fn timer(self: Box<Self>, timer: Timer) -> Box<dyn DrawBuilder> {
        self.timer(timer)
    }

    fn display(self: Box<Self>, display: DisplayBuilder) -> Box<dyn DrawBuilder> {
        self.display(display)
    }

    fn build(self: Box<Self>) -> Box<dyn Draw> {
        Box::new(TermDraw::from_builder(self))
    }
}

#[cfg(test)]
mod builder_test {
    use std::collections::VecDeque;
    use ranos_core::Timer;

    use crate::TermDrawBuilder;

    #[test]
    fn test_serialize() {
        let builder = TermDrawBuilder {
            max_width: 8,
            timer: Timer::new(None),
            displays: VecDeque::new(),
        };

        let data = serde_json::ser::to_string(&builder).unwrap();

        let expected = r#"{"max_width":8,"timer":{"target_dt":null},"displays":[]}"#.to_owned();
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"{"max_width":8,"timer":{"target_dt":null},"displays":[]}"#;

        let data: TermDrawBuilder = serde_json::de::from_str(input).unwrap();

        assert_eq!(data.max_width, 8);
        assert_eq!(data.timer, Timer::new(None));
        assert_eq!(data.displays.len(), 0);
    }
}

/// Emulates an LED display by writing whitespace with specified colored
/// backgrounds to a terminal that supports full RGB colors.
///
/// LEDs are displayed in a rectangular grid with 1 LED's worth of space between
/// each column and row.
///
/// To create a `TermDraw` object, use the [associated builder](TermDrawBuilder)
/// which can be accessed by calling [`TermDraw::builder()`](TermDraw::builder).
#[derive(Debug)]
pub struct TermDraw {
    max_width: usize,

    displays: HashMap<usize, (Display, bool)>,
    display_ids: Vec<usize>,

    timer: Timer,

    stats: DrawStats,
}

impl TermDraw {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<TermDrawBuilder> {
        Box::new(
            TermDrawBuilder {
                max_width: 8,
                timer: Timer::new(None),
                displays: VecDeque::new(),
            }
        )
    }

    fn from_builder(mut builder: Box<TermDrawBuilder>) -> Self {
        Self::new(builder.max_width, builder.timer, builder.displays.drain(0..))
    }

    fn new<I>(max_width: usize, timer: Timer, display_iter: I) -> Self
    where
        I: Iterator<Item = DisplayBuilder>,
    {
        let mut ids = Vec::new();
        let displays = display_iter
            .map(
                |b| {
                    let disp = b.build();
                    ids.push(disp.id());
                    (disp.id(), (disp, false))
                }
            )
            .collect();
        let display_ids = ids;

        Self {
            max_width,

            displays,
            display_ids,

            timer,

            stats: DrawStats::new(),
        }
    }

    fn write_frame(&mut self, display_id: usize) {
        let frame = self.displays.get(&display_id).unwrap().0.frame();

        // Create output string with enough capacity to minimize reallocations of memory for growing the string's capacity
        let mut output =
            String::with_capacity(frame.len() * 4 + (frame.len() / self.max_width) * 2);
        output.push_str("\x1B[1;1H"); // ANSI "move cursor to upper-left corner" code

        // Loop through the enumerated RGB values
        for (i, led) in frame.iter().enumerate() {
            // Check if max width has been reached on the current row
            if i % self.max_width == 0 {
                output = format!("{}\n\n", output);
            }

            // Scale the color and print it to the output
            let led = led.scale(frame.brightness());
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
    fn run(&mut self) {
        self.timer.reset();
        self.stats.reset();

        let mut num_finished = 0;

        while num_finished < self.displays.len() {
            let dt = self.timer.ping();
            let mut total_leds = 0;

            for i in 0..self.displays.len() {
                let display_id = {
                    let (d, has_finished) = self.displays.get_mut(&self.display_ids[i]).unwrap();

                    total_leds += d.frame_len();

                    if !*has_finished {
                        match d.render_frame(dt) {
                            DisplayState::Continue => (),
                            DisplayState::Last => {
                                *has_finished = true;
                                num_finished += 1;
                            },
                            DisplayState::Err => return,
                        }
                    }

                    d.id()
                };

                self.write_frame(display_id);
                self.stats.inc_frames();
            }

            self.stats.set_num(total_leds);
            self.stats.end();
        }
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}
