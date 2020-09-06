//! # Terminal Draw

use super::*;

use crate::util::timer::Timer;

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::ds::collections::frame::Frame;
use colored::Colorize;

lazy_static! {
    static ref SIGINT: Arc<AtomicBool> = {
        println!("{}", "\x1B[2J"); // ANSI clear screen code

        let arc = Arc::new(AtomicBool::new(false));

        {
            let arc = arc.clone();
            ctrlc::set_handler(move || arc.store(true, Ordering::Relaxed)).unwrap(); // Can't error check in static initialization.
        }

        arc
    };
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

    should_exit: Arc<AtomicBool>,
    stats: DrawStats,
}

impl TermDraw {
    /// Creates a new `TermDraw` object.
    ///
    /// # Parameters
    ///
    /// * `max_width` - The maximum number of LEDs to draw per line in the
    /// terminal. E.g. if there are 256 LEDs to draw and a `max_width` of 16,
    /// then a 16x16 grid will be displayed.
    pub fn new(max_width: usize) -> Self {
        Self {
            max_width,

            queue: VecDeque::new(),
            timer: Timer::new(None),

            should_exit: SIGINT.clone(),
            stats: DrawStats::new(),
        }
    }

    fn write_frame(&mut self, frame: &Frame) {
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
    fn push_queue(&mut self, a: Box<dyn Animation>) {
        self.queue.push_back(a);
    }

    fn queue_len(&self) -> usize {
        self.queue.len()
    }

    fn run(&mut self) -> Result {
        self.timer.reset();
        self.stats.reset();

        let zero_duration = Duration::new(0, 0);

        while let Some(mut ani) = self.queue.pop_front() {
            while ani.time_remaining() > zero_duration {
                ani.update(self.timer.ping());
                self.write_frame(ani.frame());

                self.stats.inc_frames();
            }

            self.stats.set_num(ani.frame().len());
            self.stats.end();

            if self.should_exit.load(Ordering::Relaxed) == true {
                for _ in 0..((ani.frame().len() / self.max_width + 1) * 2) {
                    println!("{}", "\x1B[2T");
                }
                println!("{}", "\x1B[1;1H");

                return Err("\nCaught SIGINT, stopping".to_owned());
            }
        }

        Ok(())
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}
