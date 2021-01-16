//! # Display
//!
//! Provides a level of abstraction between objects that draw and animations that get drawn.
//!
//! May become more generic in the future to facilitate different uses.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

use std::{collections::VecDeque, iter::Iterator, time::Duration};

use serde::{Deserialize, Serialize};

use ranos_animation::{Animation, AnimationBuilder, AnimationState};
use ranos_ds::collections::Frame;

/// Enum denoting different end-states that a [`Display`] object may return.
pub enum DisplayState {
    /// Denotes that the operation was successful and the object can operate for more iterations
    Continue,
    /// Denotes that the operation was successful and the object has nothing more to operate on.
    Last,
    /// Denotes that the operation failed and cannot be recovered from.
    Err,
}

/// Trait for building [`Display`]s.
#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayBuilder {
    brightness: f32,
    size: usize,
    looping: bool,
    animation_builders: Vec<Box<dyn AnimationBuilder>>,
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

    /// Sets whether the display will loop the animations or not.
    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;

        self
    }

    /// Add a builder for an animation that will be built at the same time as this builder.
    ///
    /// Note: Multiple [`AnimationBuilder`]s can be added.
    pub fn animation(mut self, animation: Box<dyn AnimationBuilder>) -> Self {
        self.animation_builders.push(animation);

        self
    }

    /// Similar to [`DisplayBuilder::animation`], but takes an iterator over
    /// animation builders, extending the internal list with the iterator's contents.
    pub fn animation_iter<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = Box<dyn AnimationBuilder>>,
    {
        self.animation_builders.extend(iter);

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

        let expected = r#"(brightness:1,size:64,looping:false,animation_builders:[])"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserializer() {
        let input = r#"(brightness:1,size:64,looping:false,animation_builders:[])"#;

        let data: DisplayBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.brightness, 1.0);
        assert_eq!(data.size, 64);
        assert_eq!(data.animation_builders.len(), 0);
    }
}

/// Provides a level of abstraction between objects that draw and animations that get drawn.
///
/// May become more generic in the future to facilitate different uses.
#[derive(Debug)]
pub struct Display {
    id: usize,
    frame: Frame,
    looping: bool,
    animations: VecDeque<Box<dyn Animation>>,
}

impl Display {
    /// Returns a builder for this type.
    pub fn builder() -> DisplayBuilder {
        DisplayBuilder {
            brightness: 1.0,
            size: 64,
            looping: false,
            animation_builders: Vec::new(),
        }
    }

    fn from_builder(mut builder: DisplayBuilder) -> Self {
        Self::with_iter(
            builder.brightness,
            builder.size,
            builder.looping,
            builder.animation_builders.drain(0..),
        )
    }

    fn with_iter<I>(brightness: f32, size: usize, looping: bool, iter: I) -> Self
    where
        I: Iterator<Item = Box<dyn AnimationBuilder>>,
    {
        Display {
            id: ranos_core::id::generate(),
            frame: Frame::new(brightness, size),
            looping,
            animations: iter.map(|ab| ab.build()).collect(),
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

    /// Renders a frame from the current animation.
    pub fn render_frame(&mut self, dt: Duration) -> DisplayState {
        if let Some(mut anim) = self.animations.pop_front() {
            match anim.render_frame(&mut self.frame, dt) {
                AnimationState::Continue => {
                    self.animations.push_front(anim);
                    DisplayState::Continue
                }
                AnimationState::Last => {
                    if self.looping {
                        self.animations.push_back(anim.reset());
                        DisplayState::Continue
                    } else {
                        if self.animations.len() > 0 {
                            DisplayState::Continue
                        } else {
                            DisplayState::Last
                        }
                    }
                }
                AnimationState::ErrRetry => self.render_frame(dt),
                AnimationState::ErrFatal => DisplayState::Err,
            }
        } else {
            DisplayState::Err
        }
    }
}
