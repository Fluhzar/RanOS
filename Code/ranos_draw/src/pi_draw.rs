//! # Raspberry Pi Draw
//!
//! This module is designed with the APA102C LEDs in mind. There are additionally aliases for the SK9822 LEDs, which have a compatible protocol to the APA102C's.
//!
//! For more details see the [`APA102CPiDraw`][crate::APA102CPiDraw] documentation.

#![cfg(target_os="linux")]

use ranos_display::DisplayState;
use rppal::gpio;
use std::collections::VecDeque;

use ranos_ds::collections::frame::Frame;
use ranos_ds::rgb::*;
use ranos_core::{Info, Timer};

use super::*;

pub const DEFAULT_DAT_PIN: u8 = 6;
pub const DEFAULT_CLK_PIN: u8 = 5;

/// Presents some info about `APA102CPiDraw` for pretty printing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct APA102CPiDrawInfo();

impl Info for APA102CPiDrawInfo {
    fn new() -> Box<dyn Info> {
        Box::new(APA102CPiDrawInfo::default())
    }

    fn name(&self) -> String {
        "PiDraw".to_owned()
    }

    fn details(&self) -> String {
        "Draws APA102C/SK9822 LEDs through a Raspberry Pi's GPIO pins. This implementation maintains compatibility with both APA102C and SK9822 LEDs.".to_owned()
    }
}

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

/// Type alias for the SK9822 LED, which is a clone of the APA102C and compatible with our implementation of the APA102C's data
/// transmission protocol.
pub type SK9822PiDraw = APA102CPiDraw;

/// Type alias of `APA102CPiDrawBuilder` for the compatible SK9822 LEDs
pub type SK9822PiDrawBuilder = APA102CPiDrawBuilder;

/// Type alias of `APA102CPiDrawInfo` for the compatible SK9822 LEDs
pub type SK9822PiDrawInfo = APA102CPiDrawInfo;

/// Struct that draws [APA102C][0] LEDs through the Raspberry Pi's GPIO pins.
///
/// This implementation is also compatible with the SK9822 LEDs, which are more or less a clone of the APA102C LED, though there are
/// some notable differences seen [here][1] that are accounted for in this implementation.
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
/// ## Start Frame
///
/// The start frame representing the start of a message to the LEDs as
/// defined by the [datasheet][2].
///
/// [2]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
///
/// ## End Frame
///
/// The end frame representing the end of a message to the LEDs as defined
/// by the [datasheet][3] with modifications as revealed in
/// [this blog post][4], and a subsequent [follow-up post][5] discussing the
/// APA102C clone, the SK9822.
///
/// [3]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
/// [4]: https://cpldcpu.wordpress.com/2014/11/30/understanding-the-apa102-superled/
/// [5]: https://cpldcpu.wordpress.com/2016/12/13/sk9822-a-clone-of-the-apa102/#[derive(Debug)]
pub struct APA102CPiDraw {
    data: Pin,
    clock: Pin,

    displays: VecDeque<(Display, bool)>,
    timer: Timer,

    stats: DrawStats,

    num: usize,
}

impl APA102CPiDraw {
    /// Returns a builder for this struct.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(target_os="linux")] {
    /// # use crate::draw::{Draw, DrawBuilder, APA102CPiDraw, APA102CPiDrawBuilder};
    /// let drawer = APA102CPiDraw::builder().build();
    /// # }
    /// ```
    pub fn builder() -> Box<APA102CPiDrawBuilder> {
        APA102CPiDrawBuilder::new()
    }

    /// Creates a new `APA102CPiDraw` object.
    ///
    /// # Parameters
    ///
    /// * `data` - The data pin for the LEDs.
    /// * `clock` - The clock pin for the LEDs.
    pub fn new(data: Pin, clock: Pin, timer: Timer) -> Self {
        Self {
            data: data,
            clock: clock,

            displays: VecDeque::new(),
            timer,

            stats: DrawStats::new(),

            num: 0,
        }
    }

    /// The start frame representing the start of a message to the LEDs as
    /// defined by the [datasheet][0].
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

    /// The end frame representing the end of a message to the LEDs as defined
    /// by the [datasheet][0] with modifications as revealed in
    /// [this blog post][1], and a subsequent [follow-up post][2] discussing the
    /// APA102C clone, the SK9822.
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

    /// Writes a single byte of data to the `data` pin sequentially one bit at a
    /// time starting with the MSB.
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

    /// Simple function used to ensure the pins are set to low before sending a
    /// message to the LEDs.
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

    /// Writes a frame to the LEDs. Uses color order BGR as defined in the
    /// datasheet.
    fn write_frame(&mut self, frame: &Frame) {
        self.start_frame();

        let brightness_mask = 0xE0 | frame.brightness_apa102c();

        for i in 0..frame.len() {
            self.write_byte(brightness_mask);
            let color = frame[i].as_tuple(RGBOrder::BGR);
            self.write_byte(color.0);
            self.write_byte(color.1);
            self.write_byte(color.2);
        }

        self.end_frame(frame.len());
    }
}

impl Draw for APA102CPiDraw {
    fn add_display(&mut self, d: Display) {
        self.displays.push_back((d, false));
        self.num += self.displays.back().unwrap().0.frame_len();
    }

    fn run(&mut self) {
        // Reset timer and stats to track just this run
        self.timer.reset();
        self.stats.reset();

        let mut numFinished = 0;

        while numFinished < self.displays.len() {
            let dt = self.timer.ping();
            let mut totalLEDs = 0;

            for i in 0..self.displays.len() {
                {
                    let (d, has_finished) = self.displays.get_mut(i).unwrap();

                    totalLEDs += d.frame_len();

                    if !*has_finished {
                        match d.render_frame(dt) {
                            DisplayState::Continue => (),
                            DisplayState::Last => {
                                *has_finished = true;
                                numFinished += 1;
                            },
                            DisplayState::Err => return,
                        }
                    }
                }

                self.write_frame(self.displays.get(i).unwrap().0.frame());
                self.stats.inc_frames();
            }

            self.stats.set_num(totalLEDs);
            self.stats.end();
        }
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}

impl Drop for APA102CPiDraw {
    /// For our eye's sake, this custom `Drop` implementation ensures that when the LED controller is stopped, the LEDs will be
    /// set to off so they don't blind anyone.
    fn drop(&mut self) {
        self.stop(self.num);
    }
}

/// Builder for [`APA102CPiDraw`][0].
///
/// Allows for optional setting of the `data`, `clock`, and `timer` parameters of [`PiDraw::new`][1]. If a parameter is not
/// supplied, a default value will be inserted in its place. This default parameter will be the same as the corresponding
/// default parameter seen in [`PiDraw::default`][2].
///
/// [0]: struct.PiDraw.html
/// [1]: struct.PiDraw.html#method.new
/// [2]: struct.PiDraw.html#method.default
#[derive(Default, Copy, Clone)]
pub struct APA102CPiDrawBuilder {
    dat_pin: Option<u8>,
    clk_pin: Option<u8>,
}

impl APA102CPiDrawBuilder {
    /// Creates new `APA102CPiDrawBuilder` object.
    pub fn new() -> Box<Self> {
        Box::new(Default::default())
    }

    /// Sets the data pin.
    pub fn data(mut self: Box<Self>, pin: u8) -> Box<Self> {
        self.dat_pin = Some(pin);

        self
    }

    /// Sets the clock pin.
    pub fn clock(mut self: Box<Self>, pin: u8) -> Box<Self> {
        self.clk_pin = Some(pin);

        self
    }
}

impl DrawBuilder for APA102CPiDrawBuilder {

    fn build(self: Box<Self>, timer: Timer) -> Box<dyn Draw> {
        let gpio = gpio::Gpio::new().unwrap();
        Box::new(APA102CPiDraw::new(
            gpio.get(self.dat_pin.unwrap_or(DEFAULT_DAT_PIN))
                .unwrap()
                .into_output(),
            gpio.get(self.clk_pin.unwrap_or(DEFAULT_CLK_PIN))
                .unwrap()
                .into_output(),
            timer,
        ))
    }
}
