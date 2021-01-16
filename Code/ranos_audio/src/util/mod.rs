//! Module providing various utility functions that are useful for working with audio.

use std::io::{self, Read, Seek};

/// Converts an 8-bit `u8` sample to a sample in `f32`.
pub fn u8_to_sample(x: &u8) -> f32 {
    let x = *x as i16 - 128;
    x as f32 / 128.0
}

/// Converts a 16-bit `i16` sample to a sample in `f32`.
pub fn i16_to_sample(x: &i16) -> f32 {
    *x as f32 / (i16::MAX as f32)
}

/// Converts a 24-bit `i32` sample to `f32`.
///
/// Note: This function assumes the data is in the MSB position (e.g. 0xNN_NN_NN_00).
pub fn i24_to_sample(x: &i32) -> f32 {
    (*x >> (32 - 24)) as f32 / ((i32::MAX >> (32 - 24)) as f32)
}

/// Combines interleaved audio channels into a single channel. The data after
/// this call will only occupy `0..data.len()/channels` space, data after this
/// range should be discarded. When combined, the audio data is attenuated by
/// the inverse of the number of channels to ensure that the result doesn't clip.
pub fn combine_channels(data: &mut [f32], channels: usize) {
    for i in 0..(data.len() / channels) {
        let mut sample = 0.0;
        for j in 0..channels {
            sample += data[channels * i + j];
        }
        data[i] = sample / (channels as f32);
    }
}

/// Reads a WAV file from the disk, returning a tuple of the sampling rate and the audio data itself converted to mono.
pub fn read_wav<R>(mut reader: R) -> io::Result<(usize, Vec<f32>)>
where
    R: Read + Seek,
{
    let (header, data) = wav::read(&mut reader)?;

    let mut data: Vec<_> = match data {
        wav::BitDepth::Eight(s) => s.iter().map(u8_to_sample).collect(),
        wav::BitDepth::Sixteen(s) => s.iter().map(i16_to_sample).collect(),
        wav::BitDepth::TwentyFour(s) => s.iter().map(i24_to_sample).collect(),
        wav::BitDepth::Empty => panic!("Empty audio data received."),
    };

    combine_channels(data.as_mut_slice(), header.channel_count as usize);
    data.truncate(data.len() / (header.channel_count as usize));

    Ok((header.sampling_rate as usize, data))
}
