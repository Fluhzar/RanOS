//! # RGB

/// Simple RGB struct that holds the color as a single `u32` value.
#[derive(Debug, Default, Copy, Clone)]
pub struct RGB {
    code: u32, // 0x00RRGGBB
}

impl RGB {
    /// Creates a new `RGB` value with default color black.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new `RGB` value from the given `u32` color code.
    #[inline]
    pub fn from_code(x: u32) -> Self {
        RGB {
            code: x & 0x00_FF_FF_FF
        }
    }

    /// Creates a new `RGB` value with a random color.
    #[inline]
    pub fn random() -> Self {
        Self::from_code(rand::random())
    }

    /// Creates a new `RGB` value from HSV values.
    #[inline]
    pub fn from_hsv(mut h: f32, s: f32, v: f32) -> Self {
        h = h % 360.0;

        let c = v * s;
        let x = c * (1.0 - (((h/60.0) % 2.0) - 1.0).abs());
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

        let (r, g, b) = (((r+m) * 255.0) as u32, ((g+m) * 255.0) as u32, ((b+m) * 255.0) as u32);

        Self::from_code(r << 16 | g << 8 | b)
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
            60.0 * (((g-b)/delta) % 6.0)
        } else if cmax == g {
            60.0 * (((b-r)/delta) + 2.0)
        } else {
            60.0 * (((r-g)/delta) + 4.0)
        };

        let s = if cmax == 0.0 {
            0.0
        } else {
            delta / cmax
        };

        let v = cmax;

        (h, s, v)
    }

    /// Sets the red value.
    #[inline]
    pub fn set_red(&mut self, red: u8) {
        self.code = (red as u32) << 16 | (self.code & 0x00_00_FF_FF);
    }

    /// Sets the green value.
    #[inline]
    pub fn set_green(&mut self, green: u8) {
        self.code = (green as u32) << 8 | (self.code & 0x00_FF_00_FF);
    }

    /// Sets the blue value.
    #[inline]
    pub fn set_blue(&mut self, blue: u8) {
        self.code = (blue as u32) << 0 | (self.code & 0x00_FF_FF_00);
    }

    /// Scales the color by the given scalar value.
    #[inline]
    pub fn scale(&self, scalar: f32) -> Self{
        let scalar = scalar.min(1.0).max(0.0);

        Self {
            code: ((self.red() as f32 * scalar) as u32 ) << 16 | ((self.green() as f32 * scalar) as u32 ) << 8 | ((self.blue() as f32 * scalar) as u32 ) << 0
        }
    }

    /// Returns the red color value.
    #[inline]
    pub fn red(&self) -> u8 {
        ((self.code & 0x00_FF_00_00) >> 16) as u8
    }

    /// Returns the green color value.
    #[inline]
    pub fn green(&self) -> u8 {
        ((self.code & 0x00_00_FF_00) >> 8) as u8
    }

    /// Returns the blue color value.
    #[inline]
    pub fn blue(&self) -> u8 {
        ((self.code & 0x00_00_00_FF) >> 0) as u8
    }

    /// Returns the value in RGB format.
    #[inline]
    pub fn rgb(&self) -> u32 {
        self.code
    }

    /// Returns the value in RBG format.
    #[inline]
    pub fn rbg(&self) -> u32 {
        ((self.red() as u32) << 16) | ((self.blue() as u32) << 8) | ((self.green() as u32) << 0)
    }

    /// Returns the value in GRB format.
    #[inline]
    pub fn grb(&self) -> u32 {
        ((self.green() as u32) << 16) | ((self.red() as u32) << 8) | ((self.blue() as u32) << 0)
    }

    /// Returns the value in GBR format.
    #[inline]
    pub fn gbr(&self) -> u32 {
        ((self.green() as u32) << 16) | ((self.blue() as u32) << 8) | ((self.red() as u32) << 0)
    }

    /// Returns the value in BRG format.
    #[inline]
    pub fn brg(&self) -> u32 {
        ((self.blue() as u32) << 16) | ((self.red() as u32) << 8) | ((self.green() as u32) << 0)
    }

    /// Returns the value in BGR format.
    #[inline]
    pub fn bgr(&self) -> u32 {
        ((self.blue() as u32) << 16) | ((self.green() as u32) << 8) | ((self.red() as u32) << 0)
    }
}
