use audio;

/// Size of the processing chunk
const CHUNK_SIZE: usize = 1 << 9; // 512

fn main() -> Result<(), std::io::Error> {
    for f in std::env::args() {
        match f.find(".wav") {
            Some(_) => {
                let (w, t) = {
                    let mut file = std::fs::File::open(f)?;
                    let (w, mut t) = audio::read_wav(&mut file)?;
                    (w, t.pop().unwrap())
                };

                let mut t = audio::gain::calc(t, CHUNK_SIZE)
                    .iter()
                    .flat_map(|x| vec![*x; CHUNK_SIZE])
                    .collect();
                audio::positive_normalize(&mut t);

                let mut file = std::fs::File::create("gain.wav")?;
                audio::WaveWriteOptions::new()
                    .bps(16)
                    .unwrap()
                    .r(w.sampling_rate as audio::MathT)
                    .clip(false)
                    .write(vec![t], &mut file)?;
            }
            None => (),
        };
    }

    Ok(())
}
