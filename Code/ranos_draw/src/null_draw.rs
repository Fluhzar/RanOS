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

impl DrawBuilder for NullDrawBuilder {
    fn build(mut self, timer: Timer) -> Box<dyn Draw> {
        self.timer = timer;
        Box::new(NullDraw::from_builder(self))
    }
}

/// Drawer that doesn't have any form of output, and only holds a [`Frame`][0]
/// to satisfy the requirements of [`Draw`][1].
///
/// [0]: ../../util/frame/struct.Frame.html
/// [1]: ../trait.Draw.html
#[derive(Debug)]
pub struct NullDraw {
    displays: VecDeque<(Display, bool)>,
    timer: Timer,

    stats: DrawStats,
}

impl NullDraw {
    /// Returns a builder for this struct.
    ///
    /// # Example
    ///
    /// ```
    /// # use base::draw::{Draw, DrawBuilder, NullDraw, NullDrawBuilder};
    /// let drawer = NullDraw::builder().build();
    /// ```
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
