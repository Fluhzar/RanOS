//! # Null Draw

use super::*;
use crate::util::{frame::Frame, rgb::RGB};

/// Drawer that doesn't have any form of output, and only holds a [`Frame`][0] to satisfy the requirements of [`Draw`][1]
/// 
/// [0]: ../../util/frame/struct.Frame.html
/// [1]: ../trait.Draw.html
#[derive(Debug, Default, Clone)]
pub struct NullDraw{
    frame: Frame,
}

impl NullDraw {
    /// Creates a new `NullDraw` object.
    pub fn new(brightness: f32, size: usize) -> Self {
        Self {
            frame: Frame::new(brightness, size),
        }
    }
}

impl Draw for NullDraw {
    fn write_frame(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn as_slice(&self) -> &[RGB] {
        self.frame.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [RGB] {
        self.frame.as_mut_slice()
    }
}
