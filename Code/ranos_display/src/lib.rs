use std::time::Duration;

use ranos_animation::{Animation, AnimationState};
use ranos_ds::collections::Frame;

pub enum DisplayState {
    Continue,
    Last,
    Err,
}

#[derive(Debug)]
pub struct Display {
    id: usize,
    frame: Frame,
    animations: Vec<Box<dyn Animation>>,
}

impl Display {
    pub fn new(brightness: f32, size: usize, animations: Vec<Box<dyn Animation>>) -> Self {
        Display {
            id: ranos_core::id::generate(),
            frame: Frame::new(brightness, size),
            animations,
        }
    }

    pub fn from_iter<I>(brightness: f32, size: usize, iter: I) -> Self
    where
        I: std::iter::Iterator<Item = Box<dyn Animation>>,
    {
        Display {
            id: ranos_core::id::generate(),
            frame: Frame::new(brightness, size),
            animations: iter.collect(),
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
