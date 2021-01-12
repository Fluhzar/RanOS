//! # Rainbow

use std::time::Duration;

use serde::{Deserialize, Serialize};

use ranos_ds::{collections::frame::Frame, const_val::ConstVal, rgb::RGB};

use super::*;

/// Builder for the [`Rainbow`](Rainbow) animation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Rainbow")]
pub struct RainbowBuilder {
    runtime: Duration,
    rainbow_length: Duration,
    sat: f32,
    val: f32,
    arc: f32,
    step: usize,
}

impl RainbowBuilder {
    /// Sets the length of time the animation should run for.
    pub fn runtime(mut self: Box<Self>, runtime: Duration) -> Box<Self> {
        self.runtime = runtime;

        self
    }

    /// Sets the length of time it takes for the rainbow to fully cycle through all the LEDs.
    pub fn rainbow_length(mut self: Box<Self>, rainbow_length: Duration) -> Box<Self> {
        self.rainbow_length = rainbow_length;

        self
    }

    /// Sets the saturation to be used for all colors generated by HSV values.
    pub fn saturation(mut self: Box<Self>, sat: f32) -> Box<Self> {
        let sat = sat.min(1.0).max(0.0);
        self.sat = sat;

        self
    }

    /// Sets the value to be used for all colors generated by HSV values.
    pub fn value(mut self: Box<Self>, val: f32) -> Box<Self> {
        let val = val.min(1.0).max(0.0);
        self.val = val;

        self
    }

    /// Sets the amount of the arc between hue 0 and 360 (mapped to the range
    /// \[0, 1\] for this parameter) that is displayed across all the LEDs.
    ///
    /// NOTE: The arc can be larger than 1, and can be as large as you like. For
    /// example an arc value of 2 would mean that there are 2 full rainbows
    /// visible across the LEDs.
    pub fn arc(mut self: Box<Self>, arc: f32) -> Box<Self> {
        self.arc = arc;

        self
    }

    /// Sets the number of LEDs in a row that keep the same color before moving
    /// on to the next color. E.g if a step of 1 yields an LED array of \[1, 2,
    /// 3, 4\], then a step of 2 yields an array of \[1, 1, 2, 2\].
    pub fn step(mut self: Box<Self>, step: usize) -> Box<Self> {
        self.step = step;

        self
    }

    /// Constructs a [`Rainbow`](Rainbow) object.
    pub fn build(self: Box<Self>) -> Rainbow {
        Rainbow::from_builder(self)
    }
}

#[typetag::serde]
impl AnimationBuilder for RainbowBuilder {
    fn build(self: Box<Self>) -> Box<dyn Animation> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use super::RainbowBuilder;
    use crate::Rainbow;
    use std::time::Duration;

    #[test]
    fn test_serialize() {
        let builder = Rainbow::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(runtime:(secs:16,nanos:0),rainbow_length:(secs:2,nanos:0),sat:1,val:1,arc:1,step:1)"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(runtime:(secs:16,nanos:0),rainbow_length:(secs:2,nanos:0),sat:1,val:1,arc:1,step:1)"#;

        let data: RainbowBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.runtime, Duration::from_secs(16));
        assert_eq!(data.rainbow_length, Duration::from_secs(2));
        assert_eq!(data.sat, 1.0);
        assert_eq!(data.val, 1.0);
        assert_eq!(data.arc, 1.0);
        assert_eq!(data.step, 1);
    }
}

/// Struct for animating the classic RGB rainbow puke that we all know and love
/// but instead of displaying on a fancy RGB keyboard it's just these stupid
/// LEDs puking out everything.
#[derive(Debug)]
pub struct Rainbow {
    runtime: ConstVal<Duration>,
    time_remaining: Duration,

    hue: f32,
    sat: ConstVal<f32>,
    val: ConstVal<f32>,
    dh: ConstVal<f32>,

    arc: ConstVal<f32>,
    step: ConstVal<usize>,
}

impl Rainbow {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<RainbowBuilder> {
        Box::new(RainbowBuilder {
            runtime: Duration::from_secs(16),
            rainbow_length: Duration::from_secs(2),
            sat: 1.0,
            val: 1.0,
            arc: 1.0,
            step: 1,
        })
    }

    fn from_builder(builder: Box<RainbowBuilder>) -> Self {
        Self::new(
            builder.runtime,
            builder.rainbow_length,
            builder.sat,
            builder.val,
            builder.arc,
            builder.step,
        )
    }

    fn new(
        runtime: Duration,
        rainbow_length: Duration,
        sat: f32,
        val: f32,
        arc: f32,
        step: usize,
    ) -> Self {
        Self {
            runtime: ConstVal::new(runtime),
            time_remaining: runtime,

            hue: 0.0,
            sat: ConstVal::new(sat),
            val: ConstVal::new(val),
            dh: ConstVal::new(360.0 / rainbow_length.as_secs_f32()),

            arc: ConstVal::new(arc),
            step: ConstVal::new(step),
        }
    }
}

impl Animation for Rainbow {
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> AnimationState {
        self.hue += self.dh.get() * dt.as_secs_f32();

        if self.hue >= 360.0 {
            self.hue -= 360.0;
        }

        let len = frame.len() as f32;
        for (i, led) in frame.iter_mut().enumerate() {
            let step = i as f32 / *self.step.get() as f32;
            let step = step.floor();
            let step = step * (*self.step.get() as f32);
            let step = step / len;
            let step = step * 360.0 * self.arc.get();
            *led = RGB::from_hsv(self.hue + step, *self.sat.get(), *self.val.get());
        }

        let mut res = AnimationState::Continue;

        self.time_remaining = if let Some(d) = self.time_remaining.checked_sub(dt) {
            d
        } else {
            res = AnimationState::Last;

            Duration::new(0, 0)
        };

        res
    }

    fn time_remaining(&self) -> Duration {
        self.time_remaining
    }

    fn reset(mut self: Box<Self>) -> Box<dyn Animation> {
        self.time_remaining = *self.runtime.get();
        self.hue = 0.0;

        self
    }
}
