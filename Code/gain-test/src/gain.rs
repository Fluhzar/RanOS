use super::*;

/// Takes the input vector and process it in chunks of size `chunk_size`. The
/// resulting size of the returned vector is equivalent to `t.len()/chunk_size`.
pub fn calc(t: SampleTrackT, chunk_size: usize) -> SampleTrackT {
    t.chunks_exact(chunk_size)
        .map(|x| (x.iter().fold(0.0, |s, x| s + x.powi(2)) / x.len() as SampleT).powf(0.5))
        .collect()
}
