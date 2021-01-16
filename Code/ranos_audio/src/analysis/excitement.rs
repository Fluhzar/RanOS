//! Algorithm adapted from one I made for the Wild Holidays charity event.

use std::sync::Arc;

use ranos_core::curve::Curve;
use rustfft::{num_complex::Complex, Fft, FftPlanner};

use crate::SIZE;

/// A type that processes audio samples and extracts info from its spectrum
pub struct Excitement {
    scalar: f32,
    curve: Curve,
    decay: f32,
    bin_range: (f32, f32),
    num_bins: usize,
    bins: Vec<f32>,
    fft: Arc<dyn Fft<f32>>,
    spectrum: [Complex<f32>; SIZE],
    scratch: [Complex<f32>; SIZE],
}

impl Excitement {
    /// Creates a new Excitement object with the given parameters.
    ///
    /// # Parameters
    ///
    /// * `scalar` - The scalar value to amplify the excitement values by
    /// * `power` - The curve power to attenuate the excitement values by. For
    /// more information see the [`Curve`] documentation. Should be in the range of \[-1, 1\].
    /// * `decay` - The rate at which the excitement value decays after a new peak value. Should be in the range of \[0, 1\).
    /// * `bin_range` - The range that the binned values will take from the
    /// spectrum data of the audio samples. The first value of the tuple is
    /// interpreted as the minimum and the second value as the maximum. Should be in the range of \[0, 1\].
    /// * `num_bins` - The number of bins that spectrum data will fit in.
    pub fn new(
        scalar: f32,
        power: f32,
        decay: f32,
        bin_range: (f32, f32),
        num_bins: usize,
    ) -> Self {
        let decay = decay.min(1.0).min(0.0);
        let bin_range = (bin_range.0.max(0.0).min(1.0), bin_range.1.min(1.0).max(0.0));

        Self {
            scalar,
            curve: Curve::new(power),
            decay,
            bin_range,
            num_bins,
            bins: vec![0.0; num_bins],
            fft: FftPlanner::new().plan_fft_forward(SIZE),
            spectrum: [Complex::new(0.0, 0.0); SIZE],
            scratch: [Complex::new(0.0, 0.0); SIZE],
        }
    }

    /// Returns a slice of the bins containing the excitement values for immutable access.
    pub fn bins(&self) -> &[f32] {
        &self.bins
    }

    /// Updates the excitement value with the given [`SIZE`] samples.
    pub fn update(&mut self, samples: &[f32]) {
        if samples.len() != SIZE {
            return;
        }

        self.fft(samples);

        for b in &mut self.bins {
            *b = 0.0;
        }

        let begin = (self.bin_range.0 * (SIZE as f32) / 2.0).floor() as usize;
        let end = (self.bin_range.1 * (SIZE as f32) / 2.0).ceil() as usize;
        let range_size = end - begin;
        let bin_size = range_size / self.num_bins;

        for i in begin..end {
            let bin_idx = (i - begin) / bin_size;
            let bin = self.bins.get_mut(bin_idx).unwrap();
            let spectrum_norm = self.spectrum[i].norm() / (SIZE as f32 / 2.0);
            *bin += spectrum_norm / (bin_size as f32);
        }

        for (i, b) in self.bins.iter_mut().enumerate() {
            let spec_begin = i * bin_size + begin;
            let spec_end = (i + 1) * bin_size + begin;

            let mut bin_mean = 0.0;
            for j in spec_begin..spec_end {
                bin_mean += self.spectrum[j].norm();
            }
            bin_mean /= (spec_end - spec_begin) as f32;

            let bin = self.scalar * (bin_mean / (SIZE as f32 / 2.0)) / (bin_size as f32);
            let curved_bin = self.curve.at(bin);

            if curved_bin > *b {
                *b = curved_bin;
            } else {
                *b *= self.decay;
            }
        }

        for b in &mut self.bins {
            *b = self.curve.at(*b);
        }
    }

    fn fft(&mut self, samples: &[f32]) {
        self.spectrum
            .iter_mut()
            .zip(samples.iter())
            .for_each(|(o, &i)| *o = Complex::new(i, 0.0));

        self.fft
            .process_with_scratch(&mut self.spectrum, &mut self.scratch);
    }
}
