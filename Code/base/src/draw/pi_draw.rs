//! # Raspberry Pi Draw

use rppal::gpio;
use std::collections::VecDeque;
use std::time::Duration;

use crate::ds::collections::frame::Frame;
use crate::ds::rgb::*;
use crate::util::{Info, Timer};

use super::*;

/// Type alias for the
pub type SK9822PiDrawInfo = APA102CPiDrawInfo;

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

/// Struct that draws [APA102C][0] LEDs through the Raspberry Pi's GPIO pins.
///
/// This struct is also compatible with the SK9822 LEDs, which are more or less a clone of the APA102C LED, though there are
/// some notable differences seen [here][1] that are accounted for in this struct.
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
#[derive(Debug)]
pub struct APA102CPiDraw {
    data: Pin,
    clock: Pin,

    queue: VecDeque<Box<dyn Animation>>,
    timer: Timer,

    known_len: usize,

    stats: DrawStats,
}

impl APA102CPiDraw {
    /// Creates a new `APA102CPiDraw` object.
    ///
    /// # Parameters
    ///
    /// * `data` - The data pin for the LEDs.
    /// * `clock` - The clock pin for the LEDs.
    pub fn new(data: Pin, clock: Pin) -> Self {
        Self {
            data: data,
            clock: clock,

            queue: VecDeque::new(),
            timer: Timer::new(None),

            known_len: 0,

            stats: DrawStats::new(),
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

        for led in frame.iter() {
            self.write_byte(0xE0 | frame.brightness_apa102c());
            let color = led.as_tuple(RGBOrder::BGR);
            self.write_byte(color.0);
            self.write_byte(color.1);
            self.write_byte(color.2);
        }

        self.end_frame(frame.len());
    }
}

impl Draw for APA102CPiDraw {
    fn push_queue(&mut self, a: Box<dyn Animation>) {
        self.queue.push_back(a);
    }

    fn queue_len(&self) -> usize {
        self.queue.len()
    }

    fn run(&mut self) -> Vec<Box<dyn Animation>>{
        // Reset timer and stats to track just this run
        self.timer.reset();
        self.stats.reset();

        // Variable for a nicer conditional for the inner while-loop and avoids unnecessary re-initialization of an unchanging
        // value
        let zero_duration = Duration::new(0, 0);

        let mut out = Vec::with_capacity(self.queue.len());

        // Loop while there are still animations to run
        while let Some(mut ani) = self.queue.pop_front() {
            // While the animation has time left to run
            while ani.time_remaining() > zero_duration {
                // Update the animation with the current delta-time and write the frames to the LEDs
                ani.update(self.timer.ping());
                self.write_frame(ani.frame());

                // Track stats
                self.stats.inc_frames();
            }

            self.stats.set_num(ani.frame().len());
            // Mark the end of the animation for stats-tracking. We know this is safe to do multiple times bc `DrawStats::end`'s
            // documentation says so
            self.stats.end();

            // If the current animation's frame is longer than the previous known longest, save it
            if ani.frame().len() > self.known_len {
                self.known_len = ani.frame().len();
            }

            out.push(ani);
        }

        out
    }

    fn stats(&self) -> DrawStats {
        self.stats
    }
}

impl Drop for APA102CPiDraw {
    /// For our eye's sake, this custom `Drop` implementation ensures that when the LED controller is stopped, the LEDs will be
    /// set to off so they don't blind anyone.
    fn drop(&mut self) {
        self.stop(self.known_len);
    }
}

impl Default for APA102CPiDraw {
    fn default() -> Self {
        let gpio = gpio::Gpio::new().unwrap();
        APA102CPiDraw::new(
            gpio.get(6).unwrap().into_output(),
            gpio.get(5).unwrap().into_output()
        )
    }
}
