use std::io;

pub struct VecWriter {
    buf: Vec<u8>,
}

impl VecWriter {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
        }
    }

    pub fn buf(&self) -> &[u8] {
        self.buf.as_slice()
    }

    pub fn buf_mut(&mut self) -> &mut [u8] {
        self.buf.as_mut_slice()
    }
}

impl io::Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
