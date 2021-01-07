//! # Null Draw

use std::collections::VecDeque;

use ranos_display::DisplayState;
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
    pub fn builder() -> Box<NullDrawBuilder> {
        NullDrawBuilder::new()
    }

    /// Creates a new `NullDraw` object.
    pub fn new(timer: Timer) -> Self {
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

        let mut numFinished = 0;

        while numFinished < self.displays.len() {
            let dt = self.timer.ping();
            let mut totalLEDs = 0;

            for i in 0..self.displays.len() {
                let (d, has_finished) = self.displays.get_mut(i).unwrap();

                if !*has_finished {
                    match d.render_frame(dt) {
                        DisplayState::Continue => (),
                        DisplayState::Last => { *has_finished = true; numFinished += 1; },
                        DisplayState::Err => return,
                    }

                    self.stats.inc_frames();
                }

                totalLEDs += d.frame_len();
            }

            self.stats.set_num(totalLEDs);
            self.stats.end();
        }
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
    fn build(self: Box<Self>, timer: Timer) -> Box<dyn Draw> {
        Box::new(NullDraw::new(timer))
    }
}
