//! Enables playback functionality using [`rodio`].

use std::{iter::Iterator, time::Duration};

use rodio::{self, OutputStream, OutputStreamHandle, PlayError, StreamError};

/// Struct for playing audio to the default output device of the running system.
///
/// Note: Any ongoing playback from this object will terminate when this object is dropped.
pub struct Output {
    _stream: OutputStream, // Need to keep value around to ensure it isn't prematurely dropped and ending active playback.
    handle: OutputStreamHandle,
}

impl Output {
    /// Creates a new output object.
    ///
    /// Returns an error if a default output device could not be opened.
    pub fn new() -> Result<Self, StreamError> {
        let (_stream, handle) = OutputStream::try_default()?;

        Ok(Self { _stream, handle })
    }

    /// Plays the supplied source to the default output device.
    pub fn play(&self, source: Source) -> Result<(), PlayError> {
        self.handle.play_raw(source)
    }
}

/// A source of audio samples to be played using the [`Output`] struct.
pub struct Source {
    data: Vec<f32>,
    sample_rate: u32,
    iter_pos: usize,
}

impl Source {
    /// Creates a new source object from the given samples and sample rate.
    pub fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        Self {
            data,
            sample_rate,
            iter_pos: 0,
        }
    }
}

impl rodio::Source for Source {
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
            self.data.len() as f32 / (self.sample_rate as f32),
        ))
    }
}

impl Iterator for Source {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;
    use std::fs::File;
    extern crate wav;

    #[test]
    fn test_playback() {
        eprintln!("Reading file");
        let (sample_rate, data) = read_wav(File::open("../assets/testSong1.wav").unwrap()).unwrap();

        let duration = Duration::from_secs_f32(data.len() as f32 / (sample_rate as f32));

        eprintln!("Playing file");
        let out = Output::new().unwrap();
        out.play(Source::new(data, sample_rate as u32)).unwrap();

        eprintln!("Sleeping for {} seconds", duration.as_secs_f32());
        std::thread::sleep(duration);
        eprintln!("Done");
    }
}
