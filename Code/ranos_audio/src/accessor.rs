//! Type allowing immutable access to preloaded mono audio samples.

use std::time::Duration;

use crate::SIZE;

/// Enables immutable access of preloaded mono audio samples.
pub struct Accessor {
    data: Vec<f32>,
    sample_rate: f32,
    ind: f32,
}

impl Accessor {
    /// Creates a new accessor object from the given data and sampling rate.
    pub fn new<R>(mut data: Vec<f32>, sample_rate: usize) -> Self {
        data.extend(vec![0.0; SIZE].iter()); // Add `SIZE` silence to the end of the data so that the last calls to `Self::most_recent_data` will always contain silence.

        Self {
            data,
            sample_rate: sample_rate as f32,
            ind: 0.0,
        }
    }

    /// Updates the internal state of the accessor with the passage of time, ensuring [`Self::most_recent_data`] is accurate.
    pub fn update(&mut self, dt: Duration) {
        self.ind += dt.as_secs_f32() * self.sample_rate;
        if self.ind > self.data.len() as f32 {
            self.ind = self.data.len() as f32;
        }
    }

    /// Accesses the most recent [`SIZE`] samples of audio.
    pub fn most_recent_data(&self) -> &[f32] {
        if (self.ind - SIZE as f32) < 0.0 {
            &self.data[0..SIZE]
        } else {
            &self.data[(self.ind as usize - SIZE)..(self.ind as usize)]
        }
    }
}
