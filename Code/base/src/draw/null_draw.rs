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
    /// Returns a builder for this struct.
    ///
    /// # Example
    ///
    /// ```
    /// # use base::draw::{Draw, DrawBuilder, NullDraw, NullDrawBuilder};
    /// let drawer = NullDraw::builder().build();
    /// ```
    pub fn builder() -> NullDrawBuilder {
        NullDrawBuilder::new()
    }

    /// Creates a new `NullDraw` object.
    pub fn new(timer: Timer) -> Self {
        Self {
            queue: VecDeque::new(),
            timer,

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

    fn run(&mut self) -> Vec<Box<dyn Animation>> {
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

        out
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}

impl Default for NullDraw {
    fn default() -> Self {
        NullDraw::new(Timer::new(None))
    }
}

/// Builder for [`NullDraw`][0].
///
/// Allows for optional setting of the `timer` parameter of [`NullDraw::new`][1]. If the parameter is not supplied, a default
/// value will be inserted in its place. This default parameter will be the same as the default parameter seen in
/// [`NullDraw::default`][2].
///
/// [0]: struct.NullDraw.html
/// [1]: struct.NullDraw.html#method.new
/// [2]: struct.NullDraw.html#method.default
#[derive(Default, Copy, Clone)]
pub struct NullDrawBuilder {
    timer: Option<Timer>,
}

impl NullDrawBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Default::default()
    }
}

impl DrawBuilder for NullDrawBuilder {
    fn timer(mut self, timer: Timer) -> Self {
        self.timer = Some(timer);

        self
    }

    fn build(self) -> Box<dyn Draw> {
        Box::new(NullDraw::new(self.timer.unwrap_or(Timer::new(None))))
    }
}
