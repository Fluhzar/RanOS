//! # Null Draw

use std::collections::VecDeque;

use serde::{Serialize, Deserialize};

use ranos_core::Timer;
use ranos_display::DisplayState;

use super::*;

/// Builder for [`NullDraw`](NullDraw).
#[derive(Default, Copy, Clone, Serialize, Deserialize)]
#[serde(rename = "NullDraw")]
pub struct NullDrawBuilder {
    #[serde(skip)]
    timer: Timer,
}

impl NullDrawBuilder {
    /// Sets the timer.
    pub fn timer(mut self, timer: Timer) -> Self {
        self.timer = timer;

        self
    }

    /// Constructs a [`NullDraw`](NullDraw) object.
    pub fn build(self) -> NullDraw {
        NullDraw::from_builder(self)
    }
}

impl DrawBuilder for NullDrawBuilder {
    fn build(self, timer: Timer) -> Box<dyn Draw> {
        Box::new(self.timer(timer).build())
    }
}

/// Drawer that doesn't have any form of output.
#[derive(Debug)]
pub struct NullDraw {
    displays: VecDeque<(Display, bool)>,
    timer: Timer,

    stats: DrawStats,
}

impl NullDraw {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> NullDrawBuilder {
        NullDrawBuilder {
            timer: Timer::new(None),
        }
    }

    fn from_builder(builder: NullDrawBuilder) -> Self {
        Self::new(builder.timer)
    }

    fn new(timer: Timer) -> Self {
        Self {
            displays: VecDeque::new(),
            timer,

            stats: DrawStats::new(),
        }
    }
}

impl Draw for NullDraw {
    fn add_display(&mut self, d: Display) {
        self.displays.push_back((d, false));
    }

    // TODO: Prime candidate for refactoring.
    fn run(&mut self) {
        self.timer.reset();
        self.stats.reset();

        let mut num_finished = 0;

        while num_finished < self.displays.len() {
            let dt = self.timer.ping();
            let mut total_leds = 0;

            for i in 0..self.displays.len() {
                let (d, has_finished) = self.displays.get_mut(i).unwrap();

                if !*has_finished {
                    match d.render_frame(dt) {
                        DisplayState::Continue => (),
                        DisplayState::Last => { *has_finished = true; num_finished += 1; },
                        DisplayState::Err => return,
                    }

                    self.stats.inc_frames();
                }

                total_leds += d.frame_len();
            }

            self.stats.set_num(total_leds);
            self.stats.end();
        }
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}
