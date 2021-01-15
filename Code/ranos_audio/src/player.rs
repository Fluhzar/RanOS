//! # Player

use std::{
    io::{self, Read, Seek},
    time::Duration,
};

use crate::SIZE;

/// Enables playback and immutable access of preloaded audio samples.
pub struct Player {
    data: Vec<f32>,
    sample_rate: f32,
    ind: f32,
    #[cfg(feature = "audio_out")]
    iter_pos: usize,
}

impl Player {
    /// Creates a new player object by reading a WAV file from the given reader.
    ///
    /// # Errors
    ///
    /// This function will return an error if the reader encountered any errors while reading or if the WAV file is malformed.
    pub fn new<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        let (header, samples) = wav::read(reader)?;

        Ok(Self {
            data: match samples {
                wav::BitDepth::Eight(s) => s.iter().map(u8_to_sample).collect(),
                wav::BitDepth::Sixteen(s) => s.iter().map(i16_to_sample).collect(),
                wav::BitDepth::TwentyFour(s) => s.iter().map(i24_to_sample).collect(),
                wav::BitDepth::Empty => vec![0.0; SIZE],
            },
            sample_rate: header.sampling_rate as f32,
            ind: 0.0,
            #[cfg(feature = "audio_out")]
            iter_pos: 0,
        })
    }

    /// Updates the internal state of the player with the passage of time, ensuring [`Player::most_recent_data`] is accurate.
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

#[cfg(feature = "audio_out")]
impl rodio::Source for Player {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.data.len() as f32 / self.sample_rate,
        ))
    }
}

#[cfg(feature = "audio_out")]
impl std::iter::Iterator for Player {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_pos < self.data.len() {
            let sam = self.data[self.iter_pos];
            self.iter_pos += 1;

            Some(sam)
        } else {
            None
        }
    }
}

fn u8_to_sample(x: &u8) -> f32 {
    let x = *x as i16 - 128;
    x as f32 / 128.0
}

fn i16_to_sample(x: &i16) -> f32 {
    *x as f32 / ((1 << 15) as f32)
}

fn i24_to_sample(x: &i32) -> f32 {
    *x as f32 / ((1 << 23) as f32)
}
