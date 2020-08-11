//! # Terminal Draw

use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::Arc;
use super::*;

use crate::util::{frame::Frame, rgb::RGB};
use colored::Colorize;

lazy_static! {
    static ref SIGINT: Arc<AtomicBool> = {
        println!("{}", "\x1B[2J"); // ANSI clear screen code

        let arc = Arc::new(AtomicBool::new(false));

        {
            let arc = arc.clone();
            ctrlc::set_handler(move || arc.store(true, Ordering::Relaxed)).unwrap();
        }

        arc
    };
}

/// Emulates an LED display by writing whitespace with specified colored
/// backgrounds to a terminal that supports full RGB colors.
/// 
/// LEDs are displayed in a rectangular grid with 1 LED's worth of space between
/// each column and row.build_helper
#[derive(Debug, Clone)]
pub struct TermDraw {
    max_width: usize,
    frame: Frame,

    should_exit: Arc<AtomicBool>,
    stats: DrawStats,
}

impl TermDraw {
    /// Creates a new `TermDraw` object.
    /// 
    /// # Parameters
    /// 
    /// * `max_width` - The maximum number of LEDs to draw per line in the terminal. E.g. if there are 256 LEDs to draw and a `max_width` of 16, then a 16x16 grid will be displayed.
    /// * `brightness` - Value in the range of \[0, 1\]. Note: the actual value sent to LEDs is an integer value in the range of \[0, 32).
    /// * `size` - The number of LEDs the drawer will draw to.
    pub fn new(max_width: usize, brightness: f32, size: usize) -> Self {
        Self {
            max_width,
            frame: Frame::new(brightness, size),

            should_exit: SIGINT.clone(),
            stats: DrawStats::new(),
        }
    }
}

impl Draw for TermDraw {
    fn write_frame(&mut self) -> Result<(), String> {
        let mut output = String::with_capacity(self.frame.len() * 3);
        output.push_str("\x1B[1;1H"); // ANSI "move cursor to upper-left corner" code

        for (i, led) in self.frame.iter().enumerate() {
            // Check if max width has been reached on the current row
            if i % self.max_width == 0 {
                output = format!("{}\n\n", output);
            }

            let led = led.scale(self.frame.brightness());
            output = format!("{}{}  ", output, "  ".on_truecolor(led.red(), led.green(), led.blue()));
        }

        println!("{}", output);

        self.stats.inc_frames();
        self.stats.end();

        if self.should_exit.load(Ordering::Relaxed) == true {
            for _ in 0..((self.frame.len()/self.max_width + 1)*2) {
                println!("{}", "\x1B[2T");
            }
            println!("{}", "\x1B[1;1H");

            Err("\nCaught SIGINT, stopping".to_owned())
        } else {
            Ok(())
        }
    }

    fn as_slice(&self) -> &[RGB] {
        self.frame.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [RGB] {
        self.frame.as_mut_slice()
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}
