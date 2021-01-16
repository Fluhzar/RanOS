//! Contains definition for the classic RGB color data structure.

use std::io;

use serde::{Deserialize, Serialize};

/// Enum defining all possible combinations of color order.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum RGBOrder {
    /// RGB-order color
    RGB,
    /// RBG-order color
    RBG,
    /// GRB-order color
    GRB,
    /// GBR-order color
    GBR,
    /// BRG-order color
    BRG,
    /// BGR-order color
    BGR,
}

/// Simple RGB struct that holds the color as a single `u32` value.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct RGB(u8, u8, u8);

impl RGB {
    /// Creates a new [`RGB`] value with default color black.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Attempts to read an [`RGB`] value from the `reader` in the given `order`, returning the resulting [`RGB`] value.
    ///
    /// # Errors
    ///
    /// This function returns an error if `reader` encounters an error during reading.
    pub fn read<R: io::Read>(reader: &mut R, order: RGBOrder) -> io::Result<RGB> {
        use std::mem::size_of;
        let mut r_buf = [0_u8; size_of::<u8>()];
        let mut g_buf = [0_u8; size_of::<u8>()];
        let mut b_buf = [0_u8; size_of::<u8>()];

        match order {
            RGBOrder::RGB => {
                reader.read_exact(&mut r_buf)?;
                reader.read_exact(&mut g_buf)?;
                reader.read_exact(&mut b_buf)?;
            }
            RGBOrder::RBG => {
                reader.read_exact(&mut r_buf)?;
                reader.read_exact(&mut b_buf)?;
                reader.read_exact(&mut g_buf)?;
            }
            RGBOrder::GRB => {
                reader.read_exact(&mut g_buf)?;
                reader.read_exact(&mut r_buf)?;
                reader.read_exact(&mut b_buf)?;
            }
            RGBOrder::GBR => {
                reader.read_exact(&mut g_buf)?;
                reader.read_exact(&mut b_buf)?;
                reader.read_exact(&mut r_buf)?;
            }
            RGBOrder::BRG => {
                reader.read_exact(&mut b_buf)?;
                reader.read_exact(&mut r_buf)?;
                reader.read_exact(&mut g_buf)?;
            }
            RGBOrder::BGR => {
                reader.read_exact(&mut b_buf)?;
                reader.read_exact(&mut g_buf)?;
                reader.read_exact(&mut r_buf)?;
            }
        }

        Ok(Self(
            u8::from_ne_bytes(r_buf),
            u8::from_ne_bytes(g_buf),
            u8::from_ne_bytes(b_buf),
        ))
    }

    /// Attempts to read a `n`umber of `RGB`s from the `reader` in the given `order`, returning the resulting [`Vec`].
    ///
    /// # Errors
    ///
    /// This function returns an error if the `reader` encounters an error during reading.
    pub fn read_n<R: io::Read>(reader: &mut R, n: usize, order: RGBOrder) -> io::Result<Vec<RGB>> {
        let mut out = Vec::with_capacity(n);

        for _ in 0..n {
            out.push(RGB::read(reader, order)?);
        }

        Ok(out)
    }

    /// Attempts to write `self` to the `writer` in the given `order`, returning the number of bytes written.
    ///
    /// # Errors
    ///
    /// This function returns an error if the `writer` encounters an error during writing.
    pub fn write<W: io::Write>(self, writer: &mut W, order: RGBOrder) -> io::Result<usize> {
        let r_buf = self.0.to_ne_bytes();
        let g_buf = self.1.to_ne_bytes();
        let b_buf = self.2.to_ne_bytes();

        match order {
            RGBOrder::RGB => {
                writer.write_all(&r_buf)?;
                writer.write_all(&g_buf)?;
                writer.write_all(&b_buf)?;
            }
            RGBOrder::RBG => {
                writer.write_all(&r_buf)?;
                writer.write_all(&b_buf)?;
                writer.write_all(&g_buf)?;
            }
            RGBOrder::GRB => {
                writer.write_all(&g_buf)?;
                writer.write_all(&r_buf)?;
                writer.write_all(&b_buf)?;
            }
            RGBOrder::GBR => {
                writer.write_all(&g_buf)?;
                writer.write_all(&b_buf)?;
                writer.write_all(&r_buf)?;
            }
            RGBOrder::BRG => {
                writer.write_all(&b_buf)?;
                writer.write_all(&r_buf)?;
                writer.write_all(&g_buf)?;
            }
            RGBOrder::BGR => {
                writer.write_all(&b_buf)?;
                writer.write_all(&g_buf)?;
                writer.write_all(&r_buf)?;
            }
        }

        Ok(r_buf.len() + g_buf.len() + b_buf.len())
    }

    /// Attempts to write a slice of `RGB` values to the `writer` in the given `order`, returning the number of bytes written.
    ///
    /// # Errors
    ///
    /// This function returns an error if the `writer` encounters an error during writing.
    pub fn write_slice<W: io::Write>(
        vec_self: &[Self],
        writer: &mut W,
        order: RGBOrder,
    ) -> io::Result<usize> {
        let mut count = 0;

        for rgb in vec_self {
            count += rgb.write(writer, order)?;
        }

        Ok(count)
    }

    /// Creates a new [`RGB`] value from the given `u32` color code, interpreted
    /// as in the specified order (e.g. `RGBOrder::RGB => code: 0xRR_GG_BB`).
    #[inline]
    pub fn from_code(x: u32, o: RGBOrder) -> Self {
        match o {
            RGBOrder::RGB => Self(
                ((x & 0x00_FF_00_00) >> 16) as u8,
                ((x & 0x00_00_FF_00) >> 8) as u8,
                ((x & 0x00_00_00_FF) >> 0) as u8,
            ),
            RGBOrder::RBG => Self(
                ((x & 0x00_FF_00_00) >> 16) as u8,
                ((x & 0x00_00_00_FF) >> 0) as u8,
                ((x & 0x00_00_FF_00) >> 8) as u8,
            ),
            RGBOrder::GRB => Self(
                ((x & 0x00_00_FF_00) >> 8) as u8,
                ((x & 0x00_FF_00_00) >> 16) as u8,
                ((x & 0x00_00_00_FF) >> 0) as u8,
            ),
            RGBOrder::GBR => Self(
                ((x & 0x00_00_00_FF) >> 0) as u8,
                ((x & 0x00_FF_00_00) >> 16) as u8,
                ((x & 0x00_00_FF_00) >> 8) as u8,
            ),
            RGBOrder::BRG => Self(
                ((x & 0x00_00_FF_00) >> 8) as u8,
                ((x & 0x00_00_00_FF) >> 0) as u8,
                ((x & 0x00_FF_00_00) >> 16) as u8,
            ),
            RGBOrder::BGR => Self(
                ((x & 0x00_00_00_FF) >> 0) as u8,
                ((x & 0x00_00_FF_00) >> 8) as u8,
                ((x & 0x00_FF_00_00) >> 16) as u8,
            ),
        }
    }

    /// Creates a new [`RGB`] value from the given tuple, interpreted as in the
    /// specified order (e.g. `RGBOrder::RGB => code: 0xRR_GG_BB`).
    #[inline]
    pub fn from_tuple(x: (u8, u8, u8), o: RGBOrder) -> Self {
        match o {
            RGBOrder::RGB => Self(x.0, x.1, x.2),
            RGBOrder::RBG => Self(x.0, x.2, x.1),
            RGBOrder::GRB => Self(x.1, x.0, x.2),
            RGBOrder::GBR => Self(x.2, x.0, x.1),
            RGBOrder::BRG => Self(x.1, x.2, x.0),
            RGBOrder::BGR => Self(x.2, x.1, x.0),
        }
    }

    /// Creates a new [`RGB`] value with a random color.
    #[inline]
    pub fn random() -> Self {
        Self(rand::random(), rand::random(), rand::random())
    }

    /// Creates a new [`RGB`] value with a random color that is bright and vibrant.
    ///
    /// In HSV terms, this means the saturation and value are set close to their maximums.
    pub fn random_bright() -> Self {
        Self::from_hsv(
            rand::random::<f32>() % 360.0,
            ((rand::random::<u16>() as u32) << 16) as f32 / 0xFFFFFFF as f32,
            ((rand::random::<u16>() as u32) << 16) as f32 / 0xFFFFFFF as f32,
        )
    }

    /// Creates a new [`RGB`] value from HSV values.
    ///
    /// Based on the algorithm found on [Wikipedia][0].
    ///
    /// [0]: https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
    #[inline]
    pub fn from_hsv(mut h: f32, s: f32, v: f32) -> Self {
        h %= 360.0;

        let c = v * s;
        let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h >= 0.0 && h < 60.0 {
            (c, x, 0.0)
        } else if h >= 60.0 && h < 120.0 {
            (x, c, 0.0)
        } else if h >= 120.0 && h < 180.0 {
            (0.0, c, x)
        } else if h >= 180.0 && h < 240.0 {
            (0.0, x, c)
        } else if h >= 240.0 && h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    #[inline]
    /// Consumes `self` and returns an HSV tuple converted from itself.
    ///
    /// Based on the algorithm found on [Wikipedia][0].
    ///
    /// # Example
    ///
    /// ```
    /// # use ranos_ds::rgb::RGB;
    /// let (h, s, v) = RGB::random().into_hsv();
    /// ```
    ///
    /// [0]: https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
    pub fn into_hsv(self) -> (f32, f32, f32) {
        let r = self.red() as f32 / 255.0;
        let g = self.green() as f32 / 255.0;
        let b = self.blue() as f32 / 255.0;

        let cmax = r.max(g.max(b));
        let cmin = r.min(g.min(b));

        let delta = cmax - cmin;

        let h = if delta == 0.0 {
            0.0
        } else if cmax == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if cmax == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if cmax == 0.0 { 0.0 } else { delta / cmax };

        let v = cmax;

        (h, s, v)
    }

    /// Sets the red value.
    #[inline]
    pub fn set_red(&mut self, red: u8) {
        self.0 = red;
    }

    /// Sets the green value.
    #[inline]
    pub fn set_green(&mut self, green: u8) {
        self.1 = green;
    }

    /// Sets the blue value.
    #[inline]
    pub fn set_blue(&mut self, blue: u8) {
        self.2 = blue;
    }

    /// Scales the color by the given scalar value.
    #[inline]
    pub fn scale(&self, scalar: f32) -> Self {
        let scalar = scalar.min(1.0).max(0.0);

        Self(
            ((self.0 as f32) * scalar).max(0.0).min(255.0) as u8,
            ((self.1 as f32) * scalar).max(0.0).min(255.0) as u8,
            ((self.2 as f32) * scalar).max(0.0).min(255.0) as u8,
        )
    }

    /// Returns the red color value.
    #[inline]
    pub fn red(&self) -> u8 {
        self.0
    }

    /// Returns the green color value.
    #[inline]
    pub fn green(&self) -> u8 {
        self.1
    }

    /// Returns the blue color value.
    #[inline]
    pub fn blue(&self) -> u8 {
        self.2
    }

    /// Returns the value in the given order.
    #[inline]
    pub fn as_tuple(&self, o: RGBOrder) -> (u8, u8, u8) {
        match o {
            RGBOrder::RGB => (self.red(), self.green(), self.blue()),
            RGBOrder::RBG => (self.red(), self.blue(), self.green()),
            RGBOrder::GRB => (self.green(), self.red(), self.blue()),
            RGBOrder::GBR => (self.green(), self.blue(), self.red()),
            RGBOrder::BRG => (self.blue(), self.red(), self.green()),
            RGBOrder::BGR => (self.blue(), self.green(), self.red()),
        }
    }
}

#[cfg(test)]
mod rgb_test {
    use super::*;

    #[test]
    fn rgb_order() {
        let sample = (0, 1, 2);

        let rgb = RGB::from_tuple(sample, RGBOrder::RGB);
        assert_eq!(rgb.red(), 0);
        assert_eq!(rgb.green(), 1);
        assert_eq!(rgb.blue(), 2);

        let result = rgb.as_tuple(RGBOrder::RGB);
        assert_eq!(sample, result);
    }

    #[test]
    fn rbg_order() {
        let sample = (0, 1, 2);

        let rbg = RGB::from_tuple(sample, RGBOrder::RBG);
        assert_eq!(rbg.red(), 0);
        assert_eq!(rbg.green(), 2);
        assert_eq!(rbg.blue(), 1);

        let result = rbg.as_tuple(RGBOrder::RBG);
        assert_eq!(sample, result);
    }

    #[test]
    fn grb_order() {
        let sample = (0, 1, 2);

        let grb = RGB::from_tuple(sample, RGBOrder::GRB);
        assert_eq!(grb.red(), 1);
        assert_eq!(grb.green(), 0);
        assert_eq!(grb.blue(), 2);

        let result = grb.as_tuple(RGBOrder::GRB);
        assert_eq!(sample, result);
    }

    #[test]
    fn gbr_order() {
        let sample = (0, 1, 2);

        let gbr = RGB::from_tuple(sample, RGBOrder::GBR);
        assert_eq!(gbr.red(), 2);
        assert_eq!(gbr.green(), 0);
        assert_eq!(gbr.blue(), 1);

        let result = gbr.as_tuple(RGBOrder::GBR);
        assert_eq!(sample, result);
    }

    #[test]
    fn brg_order() {
        let sample = (0, 1, 2);

        let brg = RGB::from_tuple(sample, RGBOrder::BRG);
        assert_eq!(brg.red(), 1);
        assert_eq!(brg.green(), 2);
        assert_eq!(brg.blue(), 0);

        let result = brg.as_tuple(RGBOrder::BRG);
        assert_eq!(sample, result);
    }

    #[test]
    fn bgr_order() {
        let sample = (0, 1, 2);

        let bgr = RGB::from_tuple(sample, RGBOrder::BGR);
        assert_eq!(bgr.red(), 2);
        assert_eq!(bgr.green(), 1);
        assert_eq!(bgr.blue(), 0);

        let result = bgr.as_tuple(RGBOrder::BGR);
        assert_eq!(sample, result);
    }
}
