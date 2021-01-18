//! This module is designed with the APA102C LEDs in mind. There are
//! additionally aliases for the SK9822 LEDs, which have a compatible protocol
//! to the APA102C's.
//!
//! For more details see the [`APA102CPiDraw`] documentation.

#![cfg(target_os = "linux")]

use std::collections::{HashMap, VecDeque};

use rppal::gpio;
use serde::{Deserialize, Serialize};

use ranos_core::Timer;
use ranos_display::DisplayState;
use ranos_ds::rgb::*;

use super::*;

/// The default data pin to use when one isn't supplied.
pub const DEFAULT_DAT_PIN: u8 = 6;
/// The default clock pin to use when one isn't supplied.
pub const DEFAULT_CLK_PIN: u8 = 5;

#[inline]
fn bit_to_level(byte: u8, bit: u8) -> gpio::Level {
    if byte >> bit & 1 != 0 {
        gpio::Level::High
    } else {
        gpio::Level::Low
    }
}

/// Local rename of the GPIO pin type.
pub type Pin = gpio::OutputPin;

/// Type alias of [`APA102CPiDrawBuilder`] for the compatible SK9822 LEDs
pub type SK9822PiDrawBuilder = APA102CPiDrawBuilder;

/// Type alias for the SK9822 LED, which is a clone of the APA102C and
/// compatible with this module's implementation of the APA102C's data transmission protocol.
pub type SK9822PiDraw = APA102CPiDraw;

/// Builder for [`APA102CPiDraw`].
#[derive(Serialize, Deserialize)]
#[serde(rename = "APA102CPiDraw")]
pub struct APA102CPiDrawBuilder {
    data_pin: u8,
    clock_pin: u8,

    brightness: u8, // should be in the range [0, 31].

    timer: Timer,
    displays: VecDeque<DisplayBuilder>,
}

impl APA102CPiDrawBuilder {
    /// Sets the data pin.
    pub fn data(mut self: Box<Self>, pin: u8) -> Box<Self> {
        self.data_pin = pin;

        self
    }

    /// Sets the clock pin.
    pub fn clock(mut self: Box<Self>, pin: u8) -> Box<Self> {
        self.clock_pin = pin;

        self
    }

    /// Sets the hardware brightness value. Should be in the range \[0, 31\].
    pub fn brightness(mut self: Box<Self>, brightness: u8) -> Box<Self> {
        self.brightness = brightness.min(31);

        self
    }

    /// Sets the timer.
    pub fn timer(mut self: Box<Self>, timer: Timer) -> Box<Self> {
        self.timer = timer;

        self
    }

    /// Add a builder for a display that will be built at the same time as this builder.
    ///
    /// Be sure to add generators to the display builder before adding it to the drawer as it will be inaccessible afterwards.
    ///
    /// Note: Multiple [`DisplayBuilder`]s can be added.
    pub fn display(mut self: Box<Self>, display: DisplayBuilder) -> Box<Self> {
        self.displays.push_back(display);

        self
    }

    /// Constructs a [`APA102CPiDraw`] object.
    pub fn build(self: Box<Self>) -> APA102CPiDraw {
        APA102CPiDraw::from_builder(self)
    }
}

#[typetag::serde]
impl DrawBuilder for APA102CPiDrawBuilder {
    fn timer(self: Box<Self>, timer: Timer) -> Box<dyn DrawBuilder> {
        self.timer(timer)
    }

    fn display(self: Box<Self>, display: DisplayBuilder) -> Box<dyn DrawBuilder> {
        self.display(display)
    }

    fn build(self: Box<Self>) -> Box<dyn Draw> {
        Box::new(self.build())
    }
}

#[cfg(test)]
mod builder_test {
    use super::{APA102CPiDraw, APA102CPiDrawBuilder, DEFAULT_CLK_PIN, DEFAULT_DAT_PIN};
    use ranos_core::Timer;

    #[test]
    fn test_serialize() {
        let builder = APA102CPiDraw::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        // eprintln!("{}", data);
        let expected =
            r#"(data_pin:6,clock_pin:5,brightness:1,timer:(target_dt:None),displays:[])"#;
        assert_eq!(data, expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"(data_pin:6,clock_pin:5,brightness:1,timer:(target_dt:None),displays:[])"#;
        let data: APA102CPiDrawBuilder = ron::de::from_str(input).unwrap();

        assert_eq!(data.data_pin, DEFAULT_DAT_PIN);
        assert_eq!(data.clock_pin, DEFAULT_CLK_PIN);
        assert_eq!(data.timer, Timer::new(None));
        assert_eq!(data.displays.len(), 0);
    }
}

/// Struct that draws [APA102C][0] LEDs through the Raspberry Pi's GPIO pins.
///
/// To create a [`APA102CPiDraw`] object, use the associated [builder](APA102CPiDrawBuilder) which can be accessed by calling
/// [`APA102CPiDraw::builder()`].
///
/// ## Compatibility
///
/// This implementation is also compatible with the SK9822 LEDs which is more or less a clone of the APA102C LED, though there
/// are some notable differences, seen [here][1], that are accounted for in this implementation.
///
/// For APA102C LEDs, it generally isn't recommended to have the brightness set to anything other than full as the PWM that
/// handles the brightness runs at 440Hz, which can cause flicker issues on lower brightness settings. The SK9822 clone gets
/// around this issue by current-limiting according to the brightness value instead of adjusting PWM settings.
///
/// NOTE TO FUTURE SELF: As both the start and end frame both are writing 0s to the data line, why not try combining the start
/// and end frame into 1 call, and writing n/2 + 32 0s to the data line in one shot.
///
/// [0]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
/// [1]: https://cpldcpu.wordpress.com/2016/12/13/sk9822-a-clone-of-the-apa102/
///
/// ## Further Info
///
/// Most of the private functions include documentation relevant to their operation. You are free to take a look at it in its
/// context, but it will also be provided here for clarity and concise-ness.
///
/// ### Start Frame
///
/// The start frame representing the start of a message to the LEDs as defined by the [datasheet][2].
///
/// [2]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
///
/// ### End Frame
///
/// The end frame representing the end of a message to the LEDs as defined by the [datasheet][3] with modifications as revealed
/// in [this blog post][4], and a subsequent [follow-up post][5] discussing the APA102C clone, the SK9822.
///
/// [3]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
/// [4]: https://cpldcpu.wordpress.com/2014/11/30/understanding-the-apa102-superled/
/// [5]: https://cpldcpu.wordpress.com/2016/12/13/sk9822-a-clone-of-the-apa102/#[derive(Debug)]
pub struct APA102CPiDraw {
    data: Pin,
    clock: Pin,

    brightness: u8,

    displays: HashMap<usize, (Display, bool)>,
    display_ids: Vec<usize>,

    timer: Timer,

    num: usize,
}

impl APA102CPiDraw {
    /// Constructs a builder object with safe default values.
    pub fn builder() -> Box<APA102CPiDrawBuilder> {
        Box::new(APA102CPiDrawBuilder {
            data_pin: DEFAULT_DAT_PIN,
            clock_pin: DEFAULT_CLK_PIN,
            brightness: 1,
            timer: Timer::new(None),
            displays: VecDeque::new(),
        })
    }

    fn from_builder(mut builder: Box<APA102CPiDrawBuilder>) -> Self {
        let gpio = gpio::Gpio::new().unwrap();

        Self::new(
            gpio.get(builder.data_pin).unwrap().into_output(),
            gpio.get(builder.clock_pin).unwrap().into_output(),
            builder.brightness,
            builder.timer,
            builder.displays.drain(0..),
        )
    }

    fn new<I>(data: Pin, clock: Pin, brightness: u8, timer: Timer, display_iter: I) -> Self
    where
        I: Iterator<Item = DisplayBuilder>,
    {
        let mut num = 0;
        let mut ids = Vec::new();
        let displays = display_iter
            .map(|b| {
                let disp = b.build();
                num += disp.frame_len();
                ids.push(disp.id());
                (disp.id(), (disp, false))
            })
            .collect();
        let display_ids = ids;

        Self {
            data: data,
            clock: clock,

            brightness,

            displays,
            display_ids,

            timer,

            num,
        }
    }

    /// The start frame representing the start of a message to the LEDs as defined by the [datasheet][0].
    ///
    /// [0]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
    #[inline]
    fn start_frame(&mut self) {
        self.set_pins_low();

        self.write_byte(0x00);
        self.write_byte(0x00);
        self.write_byte(0x00);
        self.write_byte(0x00);
    }

    /// The end frame representing the end of a message to the LEDs as defined by the [datasheet][0] with modifications as
    /// revealed in [this blog post][1], and a subsequent [follow-up post][2] discussing the APA102C clone, the SK9822.
    ///
    /// [0]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
    /// [1]: https://cpldcpu.wordpress.com/2014/11/30/understanding-the-apa102-superled/
    /// [2]: https://cpldcpu.wordpress.com/2016/12/13/sk9822-a-clone-of-the-apa102/
    #[inline]
    fn end_frame(&mut self, len: usize) {
        for _ in 0..(len >> 4) {
            self.write_byte(0x00);
        }
    }

    /// Writes a single byte of data to the `data` pin sequentially one bit at a time starting with the MSB.
    #[inline]
    fn write_byte(&mut self, byte: u8) {
        self.data.write(bit_to_level(byte, 7));
        self.clock.toggle();
        self.clock.toggle();

        self.data.write(bit_to_level(byte, 6));
        self.clock.toggle();
        self.clock.toggle();

        self.data.write(bit_to_level(byte, 5));
        self.clock.toggle();
        self.clock.toggle();

        self.data.write(bit_to_level(byte, 4));
        self.clock.toggle();
        self.clock.toggle();

        self.data.write(bit_to_level(byte, 3));
        self.clock.toggle();
        self.clock.toggle();

        self.data.write(bit_to_level(byte, 2));
        self.clock.toggle();
        self.clock.toggle();

        self.data.write(bit_to_level(byte, 1));
        self.clock.toggle();
        self.clock.toggle();

        self.data.write(bit_to_level(byte, 0));
        self.clock.toggle();
        self.clock.toggle();
    }

    /// Simple function used to ensure the pins are set to low before sending a message to the LEDs.
    #[inline]
    fn set_pins_low(&mut self) {
        self.data.set_low();
        self.clock.set_low();
    }

    /// Sets all LEDs up to `len` to black with 0 brightness, effectively
    /// turning the LEDs off. Used in system shutdown code, as well as `SIGINT`
    /// handling.
    fn stop(&mut self, len: usize) {
        self.start_frame();

        for _ in 0..len {
            self.write_byte(0xE0);
            self.write_byte(0);
            self.write_byte(0);
            self.write_byte(0);
        }

        self.end_frame(len);
    }

    /// Writes a frame to the LEDs. Uses color order `BGR` as defined in the datasheet.
    fn write_frame(&mut self, display_id: usize) {
        let (brightness_mask, len) = (
            0xE0 | self.brightness,
            self.displays.get(&display_id).unwrap().0.frame().len(),
        );

        self.start_frame();

        for i in 0..len {
            self.write_byte(brightness_mask);
            let color = {
                let frame = self.displays.get(&display_id).unwrap().0.frame();
                frame[i].scale(frame.brightness()).as_tuple(RGBOrder::BGR)
            };
            self.write_byte(color.0);
            self.write_byte(color.1);
            self.write_byte(color.2);
        }

        self.end_frame(len);
    }
}

impl Draw for APA102CPiDraw {
    fn run(&mut self) {
        // Reset timer and stats to track just this run
        self.timer.reset();

        let mut num_finished = 0;

        while num_finished < self.displays.len() {
            let dt = self.timer.ping();

            for i in 0..self.displays.len() {
                let display_id = {
                    let (d, has_finished) = self.displays.get_mut(&self.display_ids[i]).unwrap();

                    if !*has_finished {
                        match d.render_frame(dt) {
                            DisplayState::Continue => (),
                            DisplayState::Last => {
                                *has_finished = true;
                                num_finished += 1;
                            }
                            DisplayState::Err => return,
                        }
                    }

                    d.id()
                };

                self.write_frame(display_id);

                if SIGINT.load(Ordering::Relaxed) == true {
                    return;
                }
            }
        }
    }

    fn stats(&self) -> &TimerStats {
        self.timer.stats()
    }
}

impl Drop for APA102CPiDraw {
    /// For our eye's sake, this custom `Drop` implementation ensures that when
    /// the LED controller is stopped, the LEDs will be set to off so they don't blind anyone.
    fn drop(&mut self) {
        self.stop(self.num);
    }
}
