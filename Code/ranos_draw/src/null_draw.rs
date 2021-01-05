//! # Null Draw

use std::collections::VecDeque;

use ranos_animation::AnimationState;
use ranos_core::{Info, Timer};
use ranos_ds::collections::Frame;

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
    frame: Frame,

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
    pub fn builder() -> Box<NullDrawBuilder> {
        NullDrawBuilder::new()
    }

    /// Creates a new `NullDraw` object.
    pub fn new(timer: Timer, brightness: f32, size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            timer,
            frame: Frame::new(brightness, size),

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

        let mut out = Vec::new();

        while let Some(mut ani) = self.queue.pop_front() {
            loop {
                match ani.render_frame(&mut self.frame, self.timer.ping()) {
                    AnimationState::Continue | AnimationState::ErrRetry => (),
                    AnimationState::Last | AnimationState::ErrFatal => break,
                }

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
pub struct NullDrawBuilder;

impl NullDrawBuilder {
    /// Creates a new builder.
    pub fn new() -> Box<Self> {
        Box::new(Default::default())
    }
}

impl DrawBuilder for NullDrawBuilder {
    fn build(self: Box<Self>, timer: Timer, brightness: f32, size: usize) -> Box<dyn Draw> {
        Box::new(NullDraw::new(timer, brightness, size))
    }
}
