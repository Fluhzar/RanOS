//! # Sample Format
//!
//! Module containing different output formats like stereo, 2.1, 5.1, 7.1, etc.
//!
//! All functions that deal with converting raw bytes to numeric types assume
//! the bytes are in little-endian format.
//!
//! As there is no i24 built-in type, i32 is used in it's place where
//! applicable. In most cases where a 24-bit sample is stored in a 32-bit data
//! type, the upper byte is ignored or explicitly set to 0.

extern crate wav;

pub mod gain;
pub mod mono;
pub mod stereo;

pub use gain::*;
pub use mono::*;
pub use stereo::*;

use std::convert::TryFrom;
use std::ops::*;

/// Type to perform infrequent mathematical operations with when accuracy is needed.
pub type MathT = f64;
/// Type to calculate samples with.
pub type SampleT = f32;
/// Shorthand for a vector containing sample data.
pub type SampleTrackT = Vec<SampleT>;

/// Trait implementing the ability to perform math operations with a polyphonic
/// sample format and a monophonic sample.
///
/// # Dependencies:
///
/// * Default - A good default value for audio samples is 0.
/// * Most mathematical operators are required to be implemented to be able to
/// perform common operations on sample values.
/// * [`Mul`]/[`MulAssign`] is defined for both [`MathT`] as well as [`SampleT`]
/// for the convenience of common audio operations.
/// * [`From`]/[`Into`] implemented for [`SampleT`] - These functions should be
/// simple calls to [`from_sample`] and [`into_sample`] respectively.
/// * [`TryFrom`]/[`Into`] implemented for [`Vec<_>`] - These functions should
/// convert the sample values to the given standard integer types. As [`Vec`]s
/// are generic types, it cannot be assumed that any attempted conversions of
/// [`Vec`]s to a given sample format will succeed. Therefore those conversions
/// use [`TryFrom`] to indicate when there is an issue, which can be
/// communicated with the given [`String`] used for the error type. An example
/// of such an error could be (for the [`Stereo`] type):
///
/// ```rust
/// # use bae_rs::Stereo;
/// # use std::convert::TryFrom;
///
/// let v: Vec<i16> = vec![];
///
/// assert_eq!(Err("ERROR: Given vector was length 0. This function requires length 2.".to_owned()), Stereo::try_from(v));
/// ```
///
/// [`Mul`]: https://doc.rust-lang.org/std/ops/trait.Mul.html
/// [`MulAssign`]: https://doc.rust-lang.org/std/ops/trait.MulAssign.html
/// [`MathT`]: ../type.MathT.html
/// [`SampleT`]: ../type.SampleT.html
/// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`from_sample`]: #tymethod.from_sample
/// [`into_sample`]: #tymethod.into_sample
/// [`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
/// [`Vec<_>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`String`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Stereo`]: stereo/struct.Stereo.html
pub trait SampleFormat:
    Default
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Mul<SampleT, Output = Self>
    + MulAssign<SampleT>
    + Mul<MathT, Output = Self>
    + MulAssign<MathT>
    + From<SampleT>
    + Into<SampleT>
    + TryFrom<Vec<u8>, Error = String>
    + Into<Vec<u8>>
    + TryFrom<Vec<i16>, Error = String>
    + Into<Vec<i16>>
    + TryFrom<Vec<i32>, Error = String>
    + Into<Vec<i32>>
{
    /// Creates an object from a single monophonic sample.
    fn from_sample(x: SampleT) -> Self;

    /// Converts the given polyphonic sample to a monophonic sample.
    fn into_sample(self) -> SampleT;

    /// Returns the number of [`SampleT`] values held within a given
    /// [`SampleFormat`]. A common use for this would be for ensuring [`Vec`]s
    /// given to [`try_from`] have the correct size.
    ///
    /// [`SampleT`]: ../type.SampleT.html
    /// [`SampleFormat`]: trait.SampleFormat.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [`try_from`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html#tymethod.try_from
    fn num_samples() -> usize;
}

/// Trait implementing the ability to pan a monophonic sample into a polyphonic
/// sample. This is generic for the polyphonic type and the type that defines
/// how it is panned. To see an implementation, see
/// [`Stereo::to_sample_format`].
///
/// [`Stereo::to_sample_format`]: stereo/struct.Stereo.html#method.to_sample_format
pub trait Panner<G>: SampleFormat {
    /// Converts the monophonic sample into a polyphonic sample.
    fn to_sample_format(s: SampleT, g: G) -> Self;
}

/// Converts a u8 8-bit sample to a `SampleT`.
pub fn sample_from_u8(v: u8) -> SampleT {
    (v as SampleT - 128.0) / 128.0
}
/// Converts a raw byte to a `SampleT`.
pub fn sample_from_u8_bytes(v: [u8; 1]) -> SampleT {
    (v[0] as SampleT - 128.0) / 128.0
}

/// Converts a `SampleT` to an `u8`.
pub fn sample_to_u8(s: SampleT) -> u8 {
    (s * 128.0 + 128.0).round() as u8
}
/// Converts a `SampleT` to a raw little-endian byte.
pub fn sample_to_u8_bytes(s: SampleT) -> [u8; 1] {
    [sample_to_u8(s)]
}

/// Converts an i16 16-bit sample to a `SampleT`.
pub fn sample_from_i16(v: i16) -> SampleT {
    v as SampleT / ((1 << 15) as SampleT - 1.0)
}
/// Converts raw bytes to a `SampleT`.
pub fn sample_from_i16_bytes(v: [u8; 2]) -> SampleT {
    (i16::from_le_bytes(v) as SampleT) / ((1 << 15) as SampleT - 1.0)
}

/// Converts a `SampleT` to an `i16`.
pub fn sample_to_i16(s: SampleT) -> i16 {
    (s * ((1 << 15) as SampleT - 1.0)).round() as i16
}
/// Converts a `SampleT` to raw little-endian bytes.
pub fn sample_to_i16_bytes(s: SampleT) -> [u8; 2] {
    sample_to_i16(s).to_le_bytes()
}

/// Converts an i32 24-bit sample to a `SampleT`.
pub fn sample_from_i24(v: i32) -> SampleT {
    v as SampleT / ((1 << 23) as SampleT - 1.0)
}
/// Converts raw bytes to a `SampleT`.
pub fn sample_from_i24_bytes(v: [u8; 3]) -> SampleT {
    (i32::from_le_bytes([v[0], v[1], v[2], 0]) as SampleT) / ((1 << 23) as SampleT - 1.0)
}

/// Converts a `SampleT` to an `i24`.
pub fn sample_to_i24(s: SampleT) -> i32 {
    (s * ((1 << 23) as SampleT - 1.0)).round() as i32
}
/// Converts a `SampleT` to raw little-endian bytes.
pub fn sample_to_i24_bytes(s: SampleT) -> [u8; 3] {
    let i = sample_to_i24(s).to_le_bytes();

    [i[0], i[1], i[2]]
}

/// Linear interpolation (y-y1 = m * (x-x1)) of a given value.
#[inline]
pub fn lerp<T>(x: T, x1: T, x2: T, y1: T, y2: T) -> T
where
    T: Copy + Sized + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    ((y2 - y1) / (x2 - x1)) * (x - x1) + y1
}

#[inline]
fn clamp<T>(x: T, mut x1: T, mut x2: T) -> T
where
    T: Copy + Sized + PartialOrd,
{
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }

    if x > x2 {
        x2
    } else if x < x1 {
        x1
    } else {
        x
    }
}

/// Clamped linear interpolation (y-y1 = m * (x-x1)) of a given value. The input
/// `x` is clamped to the range [`x1`,`x2`]. If `x1` is greater than `x2`, they
/// are swapped.
#[inline]
pub fn clerp<T>(x: T, x1: T, x2: T, y1: T, y2: T) -> T
where
    T: Copy
        + Sized
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    lerp(clamp(x, x1, x2), x1, x2, y1, y2)
}

/// Converts a given sample count to seconds.
pub fn samples_to_seconds(s: usize, r: MathT) -> std::time::Duration {
    std::time::Duration::from_secs_f64(s as f64 * r as f64)
}

/// Converts the given duration to samples, rounded to the nearest sample.
pub fn seconds_to_samples(s: std::time::Duration, r: MathT) -> usize {
    (s.as_secs_f64() * r as f64).round() as usize
}

/// Converts from a linear gain value to a decibel (dBFS) value.
pub fn linear_to_db(g: MathT) -> MathT {
    20.0 * g.log10()
}

/// Converts from a decibel (dBFS) to a linear gain value
pub fn db_to_linear(db: MathT) -> MathT {
    10.0_f64.powf(db / 20.0)
}

/// Normalizes the given audio track to have a peak value at the given dBFS
/// value.
pub fn normalize(db: MathT, t: &mut SampleTrackT) {
    let y = t.clone();
    let mut dc = 0.0;

    for s in &y {
        dc += s;
    }

    dc /= y.len() as SampleT;

    let mut max = 0.0;

    for s in y {
        if (s - dc).abs() > max {
            max = (s - dc).abs();
        }
    }

    let factor = db_to_linear(db) as SampleT / max;

    for s in t {
        *s = (*s - dc) * factor;
    }
}

/// Same as `normalize`, just on the range of [0,1]
pub fn positive_normalize(t: &mut SampleTrackT) {
    let dc = t.clone().iter().sum::<SampleT>() / (t.len() as SampleT);

    let mut max = 0.0;
    t.iter()
        .map(|&s| (s-dc).abs())
        .for_each(|s| if s > max { max = s; });

    t.iter_mut()
        .map(|x| {*x = (*x-dc) / (2.0*max) + 0.5; *x})
        .for_each(|_| ());
}

/// Takes the given path and reads the track data from the WAV file at the given
/// location.
///
/// # Parameters/Returns
///
/// * `s` - The source to read from.
/// * Returned value is a [`std::io::Result`] with the `Ok` data being a tuple
/// of a [`wav::Header`] and a vector of [`TrackT`]s.
///
/// # Errors
///
/// This function fails if:
/// * Anything that [`wav::read`] specifies.
///
/// [`std::io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
/// [`wav::Header`]: https://docs.rs/wav/0.4.0/wav/struct.Header.html
/// [`TrackT`]: ../../type.TrackT.html
/// [`wav::read`]: https://docs.rs/wav/0.4.0/wav/fn.read.html
pub fn read_wav(s: &mut dyn std::io::Read) -> std::io::Result<(wav::Header, Vec<SampleTrackT>)> {
    let (h, bd) = wav::read(s)?;

    let mut tracks = Vec::new();
    for _ in 0..h.channel_count {
        tracks.push(SampleTrackT::new());
    }

    match bd {
        wav::BitDepth::Eight(d) => {
            for i in 0..d.len() {
                tracks[i % h.channel_count as usize].push(sample_from_u8(d[i]));
            }
        }
        wav::BitDepth::Sixteen(d) => {
            for i in 0..d.len() {
                tracks[i % h.channel_count as usize].push(sample_from_i16(d[i]));
            }
        }
        wav::BitDepth::TwentyFour(d) => {
            for i in 0..d.len() {
                tracks[i % h.channel_count as usize].push(sample_from_i24(d[i]));
            }
        }

        _ => (),
    }

    Ok((h, tracks))
}

/// Structure representing the options available to configure the format of the
/// wave file resulting from a call to [`WaveWriteOptions::write`][0], letting
/// you control the bits per sample, sampling rate, and whether or not the track
/// given to [`WaveWriteOptions::write`][0] will be clipped.
///
/// This struct uses a builder pattern, allowing you to chain the methods that
/// set the internal parameters, and then write the values at the end.
///
/// [0]: #method.write
#[derive(Default)]
pub struct WaveWriteOptions {
    bps: u16,
    r: MathT,
    clip: bool,
}

impl WaveWriteOptions {
    /// Creates new waveWriteOptions object.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the bits per sample value.
    ///
    /// Succeeds if bps is one of either 8, 16, or 24, fails otherwise.
    pub fn bps<'a>(&'a mut self, bps: u16) -> Result<&'a mut WaveWriteOptions, ()> {
        if bps == 8 || bps == 16 || bps == 24 {
            self.bps = bps;
            Ok(self)
        } else {
            Err(())
        }
    }

    /// Sets the sampling rate.
    pub fn r<'a>(&'a mut self, r: MathT) -> &'a mut WaveWriteOptions {
        self.r = r;
        self
    }

    /// Sets whether or not values outside the range of \[-1,1\] will be clipped or not.
    pub fn clip<'a>(&'a mut self, clip: bool) -> &'a mut WaveWriteOptions {
        self.clip = clip;
        self
    }

    /// Takes the given options and tracks and writes the formatted WAV data to
    /// the passed `io::Write` object.
    ///
    /// # Parameters
    ///
    /// * `tracks` - A vector of tracks to write. Each track is considered a channel.
    /// * `d` - The `io::Write` object to write to.
    ///
    /// # Errors
    ///
    /// This function will return an error under the following conditions:
    /// * Anything that [`wav::write`] specifies.
    /// * The channels don't have equal lengths.
    /// * The given vector of channels contains no data.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::fs::File;
    /// # use bae_rs::{*, generators::*, utils::*};
    ///
    /// let mut t = SampleTrackT::new();
    /// let mut opt = WaveWriteOptions::new();
    ///
    /// /* snip */
    ///
    /// opt.write(vec![t], &mut File::create("some_file.wav").unwrap());
    /// ```
    ///
    /// [`wav::write`]: https://docs.rs/wav/0.4.0/wav/fn.write.html
    pub fn write(
        &self,
        mut tracks: Vec<SampleTrackT>,
        d: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        use std::io::{Error, ErrorKind};

        if tracks.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "No channels given, aborting."));
        }

        let len = tracks[0].len();

        for t in &mut tracks {
            if t.len() != len {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Channels have mismatching lengths, aborting.",
                ));
            }
            if self.clip {
                for s in t {
                    if *s > 1.0 {
                        *s = 1.0;
                    } else if *s < -1.0 {
                        *s = -1.0;
                    }
                }
            }
        }

        match self.bps {
            8 => {
                let mut v = Vec::new();

                for i in 0..len {
                    for t in &tracks {
                        v.push(sample_to_u8(t[i]));
                    }
                }

                wav::write(
                    wav::Header::new(1, tracks.len() as u16, self.r as u32, self.bps),
                    wav::BitDepth::Eight(v),
                    d,
                )?;
            }
            16 => {
                let mut v = Vec::new();

                for i in 0..len {
                    for t in &tracks {
                        v.push(sample_to_i16(t[i]));
                    }
                }

                wav::write(
                    wav::Header::new(1, tracks.len() as u16, self.r as u32, self.bps),
                    wav::BitDepth::Sixteen(v),
                    d,
                )?;
            }
            24 => {
                let mut v = Vec::new();

                for i in 0..len {
                    for t in &tracks {
                        v.push(sample_to_i24(t[i]));
                    }
                }

                wav::write(
                    wav::Header::new(1, tracks.len() as u16, self.r as u32, self.bps),
                    wav::BitDepth::TwentyFour(v),
                    d,
                )?;
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unsupported bit depth, aborting.",
                ))
            }
        }

        Ok(())
    }
}
