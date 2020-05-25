// use crate gain_test;
use gain::*;

/// Size of the processing chunk
const CHUNK_SIZE: usize = 1 << 10;

fn calc_gain(t: SampleTrackT) -> SampleTrackT {
    t.chunks_exact(CHUNK_SIZE)
        .flat_map(|x| {
            let mut largest = 0.0;
            for s in x {
                if s.abs() > largest {
                    largest = s.abs();
                }
            }
            vec![largest; CHUNK_SIZE]
        })
        .collect()
}

fn main() -> Result<(), std::io::Error> {
    for f in std::env::args() {
        match f.find(".wav") {
            Some(_) => {
                let (w, t) = {
                    let mut file = std::fs::File::open(f)?;
                    let (w, mut t) = read_wav(&mut file)?;
                    (w, t.pop().unwrap())
                };

                let t = calc_gain(t);

                let mut file = std::fs::File::create("gain.wav")?;
                WaveWriteOptions::new()
                    .bps(16)
                    .unwrap()
                    .r(w.sampling_rate as MathT)
                    .clip(false)
                    .write(vec![t], &mut file)?;
            }
            None => (),
        };
    }

    Ok(())
}
