//! # RGB

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
#[derive(Debug, Default, Copy, Clone)]
pub struct RGB(u8, u8, u8);

impl RGB {
    /// Creates a new `RGB` value with default color black.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new `RGB` value from the given `u32` color code, interepreted as in the specified order.
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

    /// Creates a new `RGB` value from the given tuple, interepreted as in the specified order.
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

    /// Creates a new `RGB` value with a random color.
    #[inline]
    pub fn random() -> Self {
        Self(rand::random(), rand::random(), rand::random())
    }

    /// Creates a new `RGB` value from HSV values.
    #[inline]
    pub fn from_hsv(mut h: f32, s: f32, v: f32) -> Self {
        h = h % 360.0;

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
    /// # Example
    ///
    /// ```
    /// let (h, s, v) = RGB::random().into_hsv();
    /// ```
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
