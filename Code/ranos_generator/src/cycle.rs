//! Generator that cycles between colors at a set interval.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use ranos_ds::{
    const_val::ConstVal,
    rgb::{RGBOrder, RGB},
};

use super::*;

/// Builder for the [`Cycle`] generator.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Cycle")]
pub struct CycleBuilder {
    cycle_period: Duration,
    order: ColorOrder,
}

impl CycleBuilder {
    /// Sets the duration a single color is drawn for.
    pub fn cycle_period(mut self: Box<Self>, cycle_period: Duration) -> Box<Self> {
        self.cycle_period = cycle_period;

        self
    }

    /// Sets a given order that the generator cycles through.
    pub fn order(mut self: Box<Self>, order: ColorOrder) -> Box<Self> {
        self.order = order;

        self
    }

    /// Constructs a [`Cycle`] object.
    pub fn build(self: Box<Self>) -> Cycle {
        Cycle::from_builder(self)
    }
}

#[typetag::serde]
impl GeneratorBuilder for CycleBuilder {
    fn build(self: Box<Self>) -> Box<dyn Generator> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use super::CycleBuilder;
    use crate::{ColorOrder, Cycle};
    use ranos_ds::rgb::{RGBOrder, RGB};
    use std::time::Duration;

    #[test]
    fn test_serialize() {
        let builder = Cycle::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        let expected = r#"(cycle_period:(secs:0,nanos:363636363),order:Ordered([(255,0,0),(0,255,0),(0,0,255)]))"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(cycle_period:(secs:0,nanos:363636363),order:Ordered([(255,0,0),(0,255,0),(0,0,255)]))"#;

        let data: CycleBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.cycle_period, Duration::from_secs_f64(60.0 / 165.0));
        assert_eq!(
            data.order,
            ColorOrder::Ordered(vec![
                RGB::from_code(0xFF0000, RGBOrder::RGB),
                RGB::from_code(0x00FF00, RGBOrder::RGB),
                RGB::from_code(0x0000FF, RGBOrder::RGB),
            ]),
        );
    }
}

/// Struct for a simple cycling between colors by either walking a provided list
/// of colors or generating random colors. Each color is shown for the set
/// amount of time before proceeding to the next color.
#[derive(Debug)]
pub struct Cycle {
    order: ColorOrder,
    ind: usize,
    current_color: RGB,

    cycle_period: ConstVal<Duration>,
    cycle_time_remaining: Duration,
}

impl Cycle {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<CycleBuilder> {
        Box::new(CycleBuilder {
            cycle_period: Duration::from_secs_f64(60.0 / 165.0),
            order: ColorOrder::Ordered(vec![
                RGB::from_code(0xFF0000, RGBOrder::RGB),
                RGB::from_code(0x00FF00, RGBOrder::RGB),
                RGB::from_code(0x0000FF, RGBOrder::RGB),
            ]),
        })
    }

    fn from_builder(builder: Box<CycleBuilder>) -> Self {
        Self::new(builder.cycle_period, builder.order)
    }

    fn new(cycle_period: Duration, order: ColorOrder) -> Self {
        Self {
            order: order.clone(),
            ind: 0,
            current_color: match order {
                ColorOrder::Ordered(v) => v[0],
                ColorOrder::Random => RGB::random(),
                ColorOrder::RandomBright => RGB::random_bright(),
            },

            cycle_period: cycle_period.into(),
            cycle_time_remaining: cycle_period,
        }
    }
}

impl Generator for Cycle {
    fn render_frame(&mut self, frame: &mut Frame, dt: Duration) -> GeneratorState {
        self.cycle_time_remaining = if let Some(d) = self.cycle_time_remaining.checked_sub(dt) {
            d
        } else {
            if let ColorOrder::Ordered(v) = &self.order {
                self.ind += 1;
                self.ind %= v.len();
                self.current_color = v[self.ind];
            } else {
                self.current_color = RGB::random()
            }

            // Only update the frame when there's a new color
            for led in frame.iter_mut() {
                *led = self.current_color;
            }

            self.cycle_period.get().clone() + self.cycle_time_remaining - dt
        };

        GeneratorState::Ok
    }

    fn reset(mut self: Box<Self>) -> Box<dyn Generator> {
        self.ind = 0;
        self.current_color = match &self.order {
            ColorOrder::Ordered(v) => v[0],
            ColorOrder::Random => RGB::random(),
            ColorOrder::RandomBright => RGB::random_bright(),
        };
        self.cycle_time_remaining = *self.cycle_period.get();

        self
    }
}
