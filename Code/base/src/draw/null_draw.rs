//! # Null Draw

use super::*;

use crate::util::timer::Timer;

use std::collections::VecDeque;
use std::time::Duration;

/// Drawer that doesn't have any form of output, and only holds a [`Frame`][0]
/// to satisfy the requirements of [`Draw`][1].
/// 
/// [0]: ../../util/frame/struct.Frame.html
/// [1]: ../trait.Draw.html
#[derive(Debug)]
pub struct NullDraw{
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
        let zero_duration = Duration::new(0, 0);

        while let Some(mut ani) = self.queue.pop_front() {
            while ani.time_remaining() > zero_duration {
                ani.update(self.timer.ping());

                self.stats.inc_frames();
            }

            self.stats.end();
        }

        Ok(())
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}
