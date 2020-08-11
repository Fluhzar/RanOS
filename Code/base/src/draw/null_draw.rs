//! # Null Draw

use super::*;
use crate::util::{frame::Frame, rgb::RGB};

/// Drawer that doesn't have any form of output, and only holds a [`Frame`][0] to satisfy the requirements of [`Draw`][1]
/// 
/// [0]: ../../util/frame/struct.Frame.html
/// [1]: ../trait.Draw.html
#[derive(Debug, Clone)]
pub struct NullDraw{
    frame: Frame,

    stats: DrawStats,
}

impl NullDraw {
    /// Creates a new `NullDraw` object.
    pub fn new(brightness: f32, size: usize) -> Self {
        Self {
            frame: Frame::new(brightness, size),

            stats: DrawStats::new(),
        }
    }
}

impl Draw for NullDraw {
    fn write_frame(&mut self) -> Result<(), String> {
        self.stats.inc_frames();
        self.stats.end();
        Ok(())
    }

    fn as_slice(&self) -> &[RGB] {
        self.frame.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [RGB] {
        self.frame.as_mut_slice()
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}
