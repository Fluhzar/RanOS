use std::io;

#[derive(Debug, Clone)]
pub struct VecReader {
    buf: Vec<u8>,
    pos: usize,
}

impl VecReader {
    pub fn new<T, F>(inp: &[T], f: F) -> Self
    where
        F: FnMut(&T) -> &[u8],
    {
        Self {
            buf: inp.iter().flat_map(f).map(|&b| b).collect(),
            pos: 0,
        }
    }

    pub fn from_bytes(inp: &[u8]) -> Self {
        Self {
            buf: Vec::from(inp),
            pos: 0,
        }
    }
}

impl io::Read for VecReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut counter = 0;
        for (out, inp) in buf.iter_mut().zip(self.buf.iter().skip(self.pos)) {
            *out = *inp;
            counter += 1;
        }

        self.pos += counter;

        Ok(counter)
    }
}
