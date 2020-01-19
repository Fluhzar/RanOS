use riff;

type SampleT = f32;
type MathT = f64;

#[derive(Debug,Default,Copy,Clone)]
/// Struct representing a stereophonic audio sample
pub struct StereoData{
	left:SampleT,
	right:SampleT,
}

impl StereoData {
	/// Returns a new StereoData object created from individual left and right
	/// audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `l` - the left audio sample.
	/// * `r` - the right audio sapmle.
	pub fn from_stereo(l:SampleT, r:SampleT) -> StereoData {
		StereoData{
			left:l,
			right:r
		}
	}

	/// Creates a new StereoData object from a single monophonic sample. This
	/// function reduces the power of the given sample by half to reflect human
	/// hearing.
	/// 
	/// # Parameters
	/// 
	/// * `x` - the input sample.
	pub fn from_mono(x:SampleT) -> StereoData {
		StereoData{
			left: SampleT::sqrt(0.5)*x,
			right: SampleT::sqrt(0.5)*x
		}
	}

	/// Converts the given stereophonic sample to a monophonic sample by summing
	/// the left and right samples and dividing by half power to get the full
	/// power monophonic sample.
	pub fn as_mono(&self) -> SampleT {
		(self.left + self.right)/SampleT::sqrt(0.5)
	}

	/// Returns the left audio sample.
	pub fn left(&self) -> SampleT {
		self.left
	}

	/// Returns the right audio sample.
	pub fn right(&self) -> SampleT {
		self.right
	}
}

impl std::ops::Mul<SampleT> for StereoData {
	/// Output type of the multiplication
	type Output = StereoData;

	/// Multiplies a sample by a value. E.g. scaling the sample by a gain amount.
	fn mul(self, rhs: SampleT) -> Self::Output {
		StereoData {
			left: self.left * rhs,
			right: self.right * rhs,
		}
	}
}

impl std::ops::Mul<MathT> for StereoData {
	/// Output type of the multiplication
	type Output = StereoData;

	/// Multiplies a sample by a value. E.g. scaling the sample by a gain amount.
	fn mul(self, rhs: MathT) -> Self::Output {
		StereoData {
			left:(self.left as MathT * rhs) as SampleT,
			right:(self.right as MathT * rhs) as SampleT,
		}
	}
}

impl Into<Vec<u8>> for StereoData {
	/// Converts the StereoData into a vector of bytes.
	fn into(self) -> Vec<u8> {
		let mut v = Vec::new();

			// Converts the left sample from SampleT (f32) to i16, then to bytes
		let n = ((self.left * 0x8000 as SampleT) as i16).to_le_bytes();
		v.push(n[0]);
		v.push(n[1]);

			// Converts the right sample from SampleT (f32) to i16, then to bytes
		let n = ((self.right * 0x8000 as SampleT) as i16).to_le_bytes();
		v.push(n[0]);
		v.push(n[1]);

		v
	}
}

impl From<[u8;2]> for StereoData {
	/// Converts the array of 2 bytes into a StereoData object.
	/// It is assumed that the bytes are 8-bit unsigned audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The raw bytes to convert from.
	fn from(v:[u8;2]) -> Self {
		StereoData {
			left: sample_from_u8([v[0]]),
			right: sample_from_u8([v[1]])
		}
	}
}

impl From<[u8;4]> for StereoData {
	/// Converts the array of 4 bytes into a StereoData object.
	/// It is assumed that the bytes are 16-bit signed audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The raw bytes to convert from.
	fn from(v:[u8;4]) -> Self {
		StereoData {
			left: sample_from_i16([v[0],v[1]]),
			right: sample_from_i16([v[2],v[3]])
		}
	}
}

impl From<[u8;6]> for StereoData {
	/// Converts the array of 6 bytes into a StereoData object.
	/// It is assumed that the bytes are 24-bit signed audio samples.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The raw bytes to convert from.
	fn from(v:[u8;6]) -> Self {
		StereoData {
			left:  sample_from_i24([v[0],v[1],v[2]]),
			right: sample_from_i24([v[3],v[4],v[5]])
		}
	}
}

/// Converts a raw bytes to a Sample
/// It is assumed the bytes are 8-bit unsigned audio samples.
/// 
/// # Parameters
/// 
/// * `v` - The raw bytes to convert from.
pub fn sample_from_u8(v:[u8;1]) -> SampleT {
	(v[0] as SampleT - 128.0) / 128.0
}

/// Converts raw bytes to a Sample
/// It is assumed that the bytes are 16-bit signed audio samples.
/// 
/// # Parameters
/// 
/// * `v` - The raw bytes to convert from.
pub fn sample_from_i16(v:[u8;2]) -> SampleT {
	(i16::from_le_bytes(v) as SampleT) / (0x8000 as SampleT)
}

/// Converts raw bytes to a Sample
/// It is assumed that the bytes are 24-bit signed audio samples.
/// 
/// # Parameters
/// 
/// * `v` - The raw bytes to convert from.
pub fn sample_from_i24(v:[u8;3]) -> SampleT {
	(i32::from_le_bytes([v[0],v[1],v[2],0]) as SampleT) / (0x800000 as SampleT)
}

/// Converts a decibel value to a linear gain value.
/// This assumes that 0dB is unity gain, and ~-6bB is 0.5 gain
pub fn db_linear(db:MathT) -> MathT {
	10.0_f64.powf(db/20.0)
}

/// Converts a linear gain value to a decibel value.
/// This assumes that 0dB is unity gain, and ~-6bB is 0.5 gain
pub fn linear_db(g:MathT) -> MathT {
	20.0 * g.log10()
}

type TrackT = Vec<StereoData>;

#[derive(Debug,Default,Copy,Clone)]
pub struct WAVHeader {
	pub audio_format:u16,
	pub channel_count:u16,
	pub sampling_rate:u32,
	pub bytes_per_second:u32,
	pub bytes_per_sample:u16,
	pub bits_per_sample:u16,
}

impl WAVHeader {
	/// Creates a new WAVHeader object.
	/// 
	/// # Parameters
	/// 
	/// * `af` - Audio format. Generally 1.
	/// * `cc` - Channel count, the number of channels each sample has. Generally 2.
	/// * `r` - Sampling rate.
	/// * `bps` - Number of bits in each (sub-channel) sample. Generally 8, 16, or 32.
	/// 
	/// # Example
	/// 
	/// ```
	/// let h = ocae::tools::WAVHeader::new(1, 2, ocae::SAMPLE_RATE as u32, 16);
	/// ```
	pub fn new(af:u16, cc:u16, r:u32, bps:u16) -> WAVHeader {
		WAVHeader {
			audio_format: af,
			channel_count: cc,
			sampling_rate: r,
			bytes_per_second: (((bps >> 3) * cc) as u32) * r,
			bytes_per_sample: ((bps >> 3) * cc) as u16,
			bits_per_sample: bps
		}
	}
}

impl Into<[u8;16]> for WAVHeader {
	/// Converts the WAVHeader object into a vector of its bytes
	/// 
	/// # Example
	/// 
	/// ```
	/// use ocae::{SAMPLE_RATE,tools::WAVHeader};
	/// let h:[u8;16] = WAVHeader::new(1, 2, SAMPLE_RATE as u32, 16).into();
	/// ```
	fn into(self) -> [u8;16] {
		let mut v:[u8;16] = [0;16];

		let b = self.audio_format.to_le_bytes();
		v[0] = b[0];
		v[1] = b[1];
		let b = self.channel_count.to_le_bytes();
		v[2] = b[0];
		v[3] = b[1];
		let b = self.sampling_rate.to_le_bytes();
		v[4] = b[0];
		v[5] = b[1];
		v[6] = b[2];
		v[7] = b[3];
		let b = self.bytes_per_second.to_le_bytes();
		v[8] = b[0];
		v[9] = b[1];
		v[10] = b[2];
		v[11] = b[3];
		let b = self.bytes_per_sample.to_le_bytes();
		v[12] = b[0];
		v[13] = b[1];
		let b = self.bits_per_sample.to_le_bytes();
		v[14] = b[0];
		v[15] = b[1];

		v
	}
}

impl From<[u8;16]> for WAVHeader {
	/// Converts an array of 16 raw bytes into a WAVHeader object. Intended for
	/// use with bytes read in from wave files.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The raw bytes to convert from.
	fn from(v:[u8;16]) -> Self {
		let audio_format     = u16::from_le_bytes([v[0 ],v[1 ]]);
		let channel_count    = u16::from_le_bytes([v[2 ],v[3 ]]);
		let sampling_rate    = u32::from_le_bytes([v[4 ],v[5 ],v[6 ],v[7 ]]);
		let bytes_per_second = u32::from_le_bytes([v[8 ],v[9 ],v[10],v[11]]);
		let bytes_per_sample = u16::from_le_bytes([v[12],v[13]]);
		let bits_per_sample  = u16::from_le_bytes([v[14],v[15]]);

		WAVHeader {
			audio_format,
			channel_count,
			sampling_rate,
			bytes_per_second,
			bytes_per_sample,
			bits_per_sample,
		}
	}
}

impl From<&[u8]> for WAVHeader {
	/// Converts a slice of raw bytes into a WAVHeader object.
	/// 
	/// # Panics
	/// 
	/// This function will panic if the given slice is smaller than 16 bytes.
	/// 
	/// # Parameters
	/// 
	/// * `v` - The slice to convert from.
	fn from(v:&[u8]) -> Self {
		let mut a:[u8;16] = [0;16];
		a.copy_from_slice(&v[0..16]);
		WAVHeader::from(a)
	}
}

fn read_file(s: &str) -> (WAVHeader, TrackT) {
	let mut v = std::vec::Vec::new();
	let mut h:WAVHeader = WAVHeader::default();

		// Open the wave file
	let mut f = std::fs::File::open(s).unwrap();

		// Read all chunks from the file
	let mut chunks = Vec::new();
	loop {
		match riff::read_chunk(&mut f) {
			Result::Ok(c) => chunks.push(c.0),
			Result::Err(_) => break,
		};
	}

		// Parse all the chunks, saving the format and data chunk data
	let mut sam:Vec<u8> = Vec::new();
	for c in chunks {
		let mut id = Vec::new();
		id.extend_from_slice(&c.id.value);

		if String::from_utf8(id.clone()).unwrap() == String::from("fmt ") {
			match c.content {
				riff::ChunkContent::Subchunk(sc) => h=WAVHeader::from(&sc[0..16]),
				_ => ()
			};
		} else if String::from_utf8(id.clone()).unwrap() == String::from("data") {
			match c.content {
				riff::ChunkContent::Subchunk(sc) => sam=sc,
				_ => ()
			};
		}
	}

		// Convert bytes to StereoData. If channel count is anything but 1
		// or 2, it is assumed the first two indices are the left and right
		// channel, respectively
	let mut i = 0;
	while i < sam.len() {
		match h.bits_per_sample {
			8 => match h.channel_count {
				1 => {
					v.push(StereoData::from_mono(sample_from_u8([sam[i]])));
					i += 1;
				},
				c => {
					v.push(StereoData::from([sam[i],sam[i+1]]));
					i += c as usize;
				},
			},
			16 => match h.channel_count {
				1 => {
					v.push(StereoData::from_mono(sample_from_i16([sam[i],sam[i+1]])));
					i += 2;
				},
				c => {
					v.push(StereoData::from([sam[i],sam[i+1],sam[i+2],sam[i+3]]));
					i += (c as usize)*2;
				},
			},
			24 => match h.channel_count {
				1 => {
					v.push(StereoData::from_mono(sample_from_i24([sam[i],sam[i+1],sam[i+2]])));
					i += 3;
				},
				c => {
					v.push(StereoData::from([sam[i],sam[i+1],sam[i+2],sam[i+3],sam[i+4],sam[i+5]]));
					i += (c as usize)*3;
				},
			},
			_ => ()
		}
	}

	(h,v)
}

fn main() {
	for f in std::env::args() {
		match f.find(".wav") {
			Some(_) => {
				let (w,t) = read_file(f.as_str());
			},
			None => (),
		};
	}

	println!("Hello, world!");
}
