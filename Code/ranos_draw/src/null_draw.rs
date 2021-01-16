//! A drawer with no output.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use ranos_core::Timer;
use ranos_display::DisplayState;

use super::*;

/// Builder for [`NullDraw`].
#[derive(Default, Serialize, Deserialize)]
#[serde(rename = "NullDraw")]
pub struct NullDrawBuilder {
    // Fields public to crate for testing purposes, see `DrawBuilder` tests.
    pub(crate) timer: Timer,
    pub(crate) displays: VecDeque<DisplayBuilder>,
}

impl NullDrawBuilder {
    /// Sets the timer.
    pub fn timer(mut self: Box<Self>, timer: Timer) -> Box<Self> {
        self.timer = timer;

        self
    }

    /// Add a builder for a display that will be built at the same time as this builder.
    ///
    /// Be sure to add animations to the display builder before adding it to the drawer as it will be inaccessible afterwards.
    ///
    /// Note: Multiple [`DisplayBuilder`]s can be added.
    pub fn display(mut self: Box<Self>, display: DisplayBuilder) -> Box<Self> {
        self.displays.push_back(display);

        self
    }

    /// Constructs a [`NullDraw`] object.
    pub fn build(self: Box<Self>) -> NullDraw {
        NullDraw::from_builder(self)
    }
}

#[typetag::serde]
impl DrawBuilder for NullDrawBuilder {
    fn timer(self: Box<Self>, timer: Timer) -> Box<dyn DrawBuilder> {
        self.timer(timer)
    }

    fn display(self: Box<Self>, display: DisplayBuilder) -> Box<dyn DrawBuilder> {
        self.display(display)
    }

    fn build(self: Box<Self>) -> Box<dyn Draw> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use crate::{NullDraw, NullDrawBuilder};
    use ranos_core::Timer;

    #[test]
    fn test_serialize() {
        let builder = NullDraw::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        // eprintln!("{}", data);
        let expected = r#"(timer:(target_dt:None),displays:[])"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(timer:(target_dt:None),displays:[])"#;

        let data: NullDrawBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.timer, Timer::new(None));
        assert_eq!(data.displays.len(), 0);
    }
}

/// Drawer that doesn't have any form of output.
#[derive(Debug)]
pub struct NullDraw {
    displays: Vec<(Display, bool)>,
    timer: Timer,
}

impl NullDraw {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<NullDrawBuilder> {
        Box::new(NullDrawBuilder {
            timer: Timer::new(None),
            displays: VecDeque::new(),
        })
    }

    fn from_builder(mut builder: Box<NullDrawBuilder>) -> Self {
        Self::new(builder.timer, builder.displays.drain(0..))
    }

    fn new<I>(timer: Timer, display_iter: I) -> Self
    where
        I: Iterator<Item = DisplayBuilder>,
    {
        Self {
            displays: display_iter.map(|b| (b.build(), false)).collect(),
            timer,
        }
    }
}

impl Draw for NullDraw {
    fn run(&mut self) {
        self.timer.reset();

        let mut num_finished = 0;

        while num_finished < self.displays.len() {
            let dt = self.timer.ping();

            for i in 0..self.displays.len() {
                let (d, has_finished) = self.displays.get_mut(i).unwrap();

                if !*has_finished {
                    match d.render_frame(dt) {
                        DisplayState::Continue => (),
                        DisplayState::Last => {
                            *has_finished = true;
                            num_finished += 1;
                        }
                        DisplayState::Err => return,
                    }
                }

                if SIGINT.load(Ordering::Relaxed) == true {
                    return;
                }
            }
        }
    }

    fn stats(&self) -> &TimerStats {
        self.timer.stats()
    }
}
