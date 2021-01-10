use std::{
    iter::Iterator,
    time::Duration
};

use serde::{Serialize, Deserialize};

use ranos_animation::{Animation, AnimationState, AnimationBuilder};
use ranos_ds::collections::Frame;

pub enum DisplayState {
    Continue,
    Last,
    Err,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayBuilder {
    brightness: f32,
    size: usize,
    animation_builders: Vec<Box<dyn AnimationBuilder>>,
}

impl DisplayBuilder {
    pub fn brightness(mut self, brightness: f32) -> Self {
        self.brightness = brightness;

        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;

        self
    }

    pub fn dimensions(mut self, width: usize, height: usize) -> Self {
        self.size = width*height;

        self
    }

    pub fn add_animation_builder(mut self, animation: Box<dyn AnimationBuilder>) -> Self {
        self.animation_builders.push(animation);

        self
    }

    pub fn add_animation_builders<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = Box<dyn AnimationBuilder>>,
    {
        self.animation_builders.extend(iter);

        self
    }

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

        let data = serde_json::ser::to_string(&builder).unwrap();

        let expected = r#"{"brightness":1.0,"size":64,"animation_builders":[]}"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserializer() {
        let input = r#"{"brightness":1.0,"size":64,"animation_builders":[]}"#;

        let data: DisplayBuilder = serde_json::de::from_str(input).unwrap();

        assert_eq!(data.brightness, 1.0);
        assert_eq!(data.size, 64);
        assert_eq!(data.animation_builders.len(), 0);
    }
}

#[derive(Debug)]
pub struct Display {
    id: usize,
    frame: Frame,
    animations: Vec<Box<dyn Animation>>,
}

impl Display {
    pub fn builder() -> DisplayBuilder {
        DisplayBuilder {
            brightness: 1.0,
            size: 64,
            animation_builders: Vec::new(),
        }
    }

    fn from_builder(mut builder: DisplayBuilder) -> Self {
        Self::with_iter(builder.brightness, builder.size, builder.animation_builders.drain(0..))
    }

    fn with_iter<I>(brightness: f32, size: usize, iter: I) -> Self
    where
        I: Iterator<Item = Box<dyn AnimationBuilder>>,
    {
        Display {
            id: ranos_core::id::generate(),
            frame: Frame::new(brightness, size),
            animations: iter.map(|ab| ab.build()).collect(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    pub fn frame_len(&self) -> usize {
        self.frame.len()
    }

    pub fn render_frame(&mut self, dt: Duration) -> DisplayState {
        if let Some(anim) = self.animations.first_mut() {
            match anim.render_frame(&mut self.frame, dt) {
                AnimationState::Continue => DisplayState::Continue,
                AnimationState::Last => DisplayState::Last,
                AnimationState::ErrRetry => self.render_frame(dt),
                AnimationState::ErrFatal => DisplayState::Err,
            }
        } else {
            DisplayState::Err
        }
    }
}
