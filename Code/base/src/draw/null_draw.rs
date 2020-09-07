//! # Null Draw

use std::collections::VecDeque;
use std::time::Duration;

use crate::util::{Info, Timer};

use super::*;

/// Presents some info about `TermDraw` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NullDrawInfo();

impl Info for NullDrawInfo {
    fn new() -> Box<dyn Info>
    where
        Self: Sized,
    {
        Box::new(NullDrawInfo::default())
    }

    fn name(&self) -> String {
        "NullDraw".to_owned()
    }

    fn details(&self) -> String {
        "Drawer that doesn't have any form of output.".to_owned()
    }
}

/// Drawer that doesn't have any form of output, and only holds a [`Frame`][0]
/// to satisfy the requirements of [`Draw`][1].
///
/// [0]: ../../util/frame/struct.Frame.html
/// [1]: ../trait.Draw.html
#[derive(Debug)]
pub struct NullDraw {
    queue: VecDeque<Box<dyn Animation>>,
    timer: Timer,

    stats: DrawStats,
}

impl NullDraw {
    /// Creates a new `NullDraw` object.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            timer: Timer::new(None),

            stats: DrawStats::new(),
        }
    }
}

impl Draw for NullDraw {
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

        let mut out = Vec::new();

        while let Some(mut ani) = self.queue.pop_front() {
            while ani.time_remaining() > zero_duration {
                ani.update(self.timer.ping());

                self.stats.inc_frames();
            }

            self.stats.set_num(ani.frame().len());
            self.stats.end();
            out.push(ani);
        }

        Ok(out)
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}

impl Default for NullDraw {
    fn default() -> Self {
        NullDraw::new()
    }
}
