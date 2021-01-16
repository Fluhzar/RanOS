//! Defines a struct that will take an input value in the range \[-1, 1\] and curve it depending on a configured curve power.

/// Creates a curve of varying properties based on the input power (in a range of \[-1, 1\]).
///
/// The curve's customizability is demonstrated with [this Desmos graph set][0]
/// (the input `power` value here is represented with the `p` value in the graph).
///
/// If the link doesn't work, here is the equation in LaTeX used for the curve,
/// with `p` relating directly to the `power` parameter:
///
/// ```tex
/// y=\frac{\sqrt{p^2+p(4x-2)+1}-p(x-1)-1}{p}
/// ```
///
/// Note the special case when `p` equals 0. At this point the above function is
/// undefined, but a power of 0 is meant to represent no curve applied to the
/// input value. To resolve this issue there is a simple test case in the
/// [`Self::at`] function that ensures the correct behavior.
///
/// [0]: https://www.desmos.com/calculator/uzxxrnudcy
pub struct Curve {
    power: f32,
}

impl Curve {
    /// Creates a new object with the given curve. Value will be clamped to the range \[-1, 1\].
    pub fn new(power: f32) -> Self {
        let power = power.min(1.0).max(-1.0);
        Self { power }
    }

    /// Sets the curve power. Value will be clamped to the range \[-1, 1\].
    pub fn set_power(&mut self, power: f32) {
        let power = power.min(1.0).max(-1.0);

        self.power = power;
    }

    /// Returns the power of the curve.
    pub fn power(&self) -> f32 {
        self.power
    }

    /// Calculates the value of the curve at a given input.
    pub fn at(&self, x: f32) -> f32 {
        let x = x.max(0.0).min(1.0);

        if self.power == 0.0 {
            x
        } else {
            ((self.power.powi(2) + self.power * (4.0 * x - 2.0) + 1.0).sqrt()
                - self.power * (x - 1.0)
                - 1.0)
                / self.power
        }
    }
}
