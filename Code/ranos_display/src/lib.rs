//! # Display
//!
//! Provides a level of abstraction between objects that draw and generators that get drawn.
//!
//! May become more generic in the future to facilitate different uses.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

use std::{
    collections::{HashMap, VecDeque},
    iter::Iterator,
    time::Duration,
};

use ranos_filter::{Filter, FilterBuilder};
use serde::{Deserialize, Serialize};

use ranos_ds::{collections::Frame, const_val::ConstVal};
use ranos_generator::{Generator, GeneratorBuilder, GeneratorState};

/// Sets the type of runtime a generator has within the display. Can be a configured time, or an event trigger.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Runtime {
    /// The duration of time the generator should run for.
    Time(Duration),
    ///
    Trigger,
}

/// Enum denoting different end-states that a [`Display`] object may return.
pub enum DisplayState {
    /// Denotes that the operation was successful.
    Ok,
    /// Denotes that the operation was successful and there are no more operations to perform.
    Done,
    /// Denotes that an error occurred and cannot be recovered from.
    Err,
}

/// Trait for building [`Display`]s.
#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayBuilder {
    brightness: f32,
    size: usize,
    looping: bool,
    generator_builders: Vec<Box<dyn GeneratorBuilder>>,
    generator_runtimes: Vec<Runtime>,
    filter_builders: Vec<Box<dyn FilterBuilder>>,
}

impl DisplayBuilder {
    /// Sets the brightness, value will be clamped to the range \[0, 1\].
    pub fn brightness(mut self, brightness: f32) -> Self {
        let brightness = brightness.min(1.0).max(0.0);
        self.brightness = brightness;

        self
    }

    /// Sets the size, the number of LEDs to display.
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;

        self
    }

    /// Alternative to [`Self::size`].
    ///
    /// Calculates the size as `width * height`.
    pub fn dimensions(self, width: usize, height: usize) -> Self {
        self.size(width * height)
    }

    /// Sets whether the display will loop the generators or not.
    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;

        self
    }

    /// Add a builder for an generator that will be built at the same time as this builder.
    ///
    /// The `runtime` parameter specifies how long the generator should run for
    ///
    /// Note: Multiple [`GeneratorBuilder`]s can be added.
    pub fn generator(mut self, builder: Box<dyn GeneratorBuilder>, runtime: Runtime) -> Self {
        self.generator_builders.push(builder);
        self.generator_runtimes.push(runtime);

        self
    }

    /// Similar to [`Self::generator`], but takes an iterator over generator
    /// builders, extending the internal list with the iterator's contents.
    pub fn generator_iter<I, R>(mut self, builder_iter: I, runtime_iter: R) -> Self
    where
        I: Iterator<Item = Box<dyn GeneratorBuilder>>,
        R: Iterator<Item = Runtime>,
    {
        self.generator_builders.extend(builder_iter);
        self.generator_runtimes.extend(runtime_iter);

        self
    }

    /// Add a builder for a filter that will be built at the same time as this builder.
    ///
    /// Note: Multiple [`FilterBuilder`]s can be added.
    pub fn filter(mut self, builder: Box<dyn FilterBuilder>) -> Self {
        self.filter_builders.push(builder);

        self
    }

    /// Similar to [`Self::filter`], but takes an iterator over filter builders,
    /// extending the internal list with the iterator's contents.
    pub fn filter_iter<I>(mut self, builder_iter: I) -> Self
    where
        I: Iterator<Item = Box<dyn FilterBuilder>>,
    {
        self.filter_builders.extend(builder_iter);

        self
    }

    /// Builds a [`Display`].
    pub fn build(self) -> Display {
        Display::from_builder(self)
    }
}

#[cfg(test)]
mod builder_test {
    use crate::{Display, DisplayBuilder};

    #[test]
    fn test_serializer() {
        let builder = Display::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected =
            r#"(brightness:1,size:64,looping:false,generator_builders:[],generator_runtimes:[])"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserializer() {
        let input =
            r#"(brightness:1,size:64,looping:false,generator_builders:[],generator_runtimes:[])"#;

        let data: DisplayBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.brightness, 1.0);
        assert_eq!(data.size, 64);
        assert_eq!(data.generator_builders.len(), 0);
        assert_eq!(data.generator_runtimes.len(), 0);
    }
}

/// Provides a level of abstraction between objects that draw and objects that generate the pixel data.
///
/// May become more generic in the future to facilitate different uses.
#[derive(Debug)]
pub struct Display {
    id: usize,

    frame: Frame,
    looping: bool,

    generators: VecDeque<(Box<dyn Generator>, Runtime)>,
    filters: Vec<Box<dyn Filter>>,

    original_runtimes: ConstVal<HashMap<usize, Runtime>>,
}

impl Display {
    /// Returns a builder for this type.
    pub fn builder() -> DisplayBuilder {
        DisplayBuilder {
            brightness: 1.0,
            size: 64,
            looping: false,
            generator_builders: Vec::new(),
            generator_runtimes: Vec::new(),
            filter_builders: Vec::new(),
        }
    }

    fn from_builder(mut builder: DisplayBuilder) -> Self {
        Self::new(
            builder.brightness,
            builder.size,
            builder.looping,
            builder
                .generator_builders
                .drain(0..)
                .zip(builder.generator_runtimes.drain(0..))
                .map(|(ab, rt)| (ab.build(), rt)),
            builder.filter_builders.drain(0..).map(|fb| fb.build()),
        )
    }

    fn new<G, F>(
        brightness: f32,
        size: usize,
        looping: bool,
        generator_iter: G,
        filter_iter: F,
    ) -> Self
    where
        G: Iterator<Item = (Box<dyn Generator>, Runtime)>,
        F: Iterator<Item = Box<dyn Filter>>,
    {
        let generators: VecDeque<_> = generator_iter.collect();
        let runtimes = generators.iter().map(|(g, rt)| (g.id(), *rt)).collect();

        Display {
            id: ranos_core::id::generate(),

            frame: Frame::new(brightness, size),
            looping,

            generators,
            filters: filter_iter.collect(),

            original_runtimes: ConstVal::new(runtimes),
        }
    }

    /// Returns the id of this display.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns a reference to the internal frame.
    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    /// Returns the length of the internal frame, representing the number of LEDs to draw to.
    pub fn frame_len(&self) -> usize {
        self.frame.len()
    }

    /// Triggers the display to advance to the next generator.
    pub fn trigger_next_generator(&mut self) {
        self.generators.pop_front();
    }

    /// Renders a frame from the current generator.
    pub fn render_frame(&mut self, dt: Duration) -> DisplayState {
        if let Some((mut anim, rt)) = self.generators.pop_front() {
            match anim.render_frame(&mut self.frame, dt) {
                GeneratorState::Ok => {
                    match rt {
                        Runtime::Time(t) => {
                            if let Some(t) = t.checked_sub(dt) {
                                self.generators.push_front((anim, Runtime::Time(t)));
                            } else {
                                if self.looping {
                                    self.generators.push_back((anim, rt));
                                }
                                // // Render the next frame with the remaining `dt` of the current frame.
                                // self.render_frame(dt.checked_sub(t).unwrap());
                            }
                        }
                        Runtime::Trigger => {
                            self.trigger_next_generator();
                        }
                    };

                    DisplayState::Ok
                }
                GeneratorState::ErrRetry => self.render_frame(dt),
                GeneratorState::ErrSkip => {
                    self.generators.push_front((anim, rt));

                    DisplayState::Ok
                }
                GeneratorState::ErrFatal => DisplayState::Err,
            }
        } else {
            DisplayState::Done
        }
    }

    /// Resets the display to its pre-run state, operating as if it were never run before.
    pub fn reset(&mut self) {
        for (g, rt) in self.generators.iter_mut() {
            g.reset();
            *rt = *self.original_runtimes.get().get(&g.id()).unwrap();
        }
    }
}
