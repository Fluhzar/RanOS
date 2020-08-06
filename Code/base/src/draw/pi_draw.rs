//! # Raspberry Pi Draw

use super::*;

use crate::util::{frame::Frame, rgb::*};
use rppal::gpio;
use std::cell::RefCell;

/// Type used to represent a GPIO pin with interior mutability. This is required bc iterating over a frame borrows `self`, and
/// setting the pin values requires mutation through `self` inside the iteration.
pub type Pin = RefCell<gpio::OutputPin>;

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

    frame: Frame,
}

impl APA102CPiDraw {
    /// Creates a new `APA102CPiDraw` object.
    /// 
    /// # Parameters
    /// 
    /// * `data` - The data pin for the LEDs
    /// * `clock` - The clock pin for the LEDs
    /// * `brightness` - Value in the range of \[0, 1\]. Note: the actual value sent to LEDs is an integer value in the range of \[0, 31\].
    /// * `size` - The number of LEDs the drawer will draw to.
    pub fn new(data: gpio::OutputPin, clock: gpio::OutputPin, brightness: f32, size: usize) -> Self {
        Self {
            data: RefCell::new(data),
            clock: RefCell::new(clock),

            frame: Frame::new(brightness, size),
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
    fn end_frame(&mut self) {
        for _ in 0..(self.frame.len()>>4) {
            self.write_byte(0x00);
        }
    }

    /// Writes a single byte of data to the `data` pin sequentially one bit at a
    /// time starting with the MSB.
    #[inline]
    fn write_byte(&self, byte: u8) {
        use rppal::gpio::Level;

        self.data.borrow_mut().write(if byte>>7 & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();

        self.data.borrow_mut().write(if byte>>6 & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();

        self.data.borrow_mut().write(if byte>>5 & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();

        self.data.borrow_mut().write(if byte>>4 & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();

        self.data.borrow_mut().write(if byte>>3 & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();

        self.data.borrow_mut().write(if byte>>2 & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();

        self.data.borrow_mut().write(if byte>>1 & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();

        self.data.borrow_mut().write(if byte & 1 > 0 { Level::High } else { Level::Low });
        self.clock.borrow_mut().toggle();
        self.clock.borrow_mut().toggle();
    }

    /// Simple function used to ensure the pins are set to low before sending a
    /// message to the LEDs.
    #[inline]
    fn set_pins_low(&mut self) {
        self.data.borrow_mut().set_low();
        self.clock.borrow_mut().set_low();
    }
}

impl Draw for APA102CPiDraw {
    /// Writes a frame to the LEDs. Uses color order BGR as defined in the
    /// datasheet.
    fn write_frame(&mut self) -> Result<(), String> {
        self.start_frame();

        for led in self.frame.iter() {
            self.write_byte(0xE0 | self.frame.brightness_apa102c());
            let color = led.as_tuple(RGBOrder::BGR);
            self.write_byte(color.0);
            self.write_byte(color.1);
            self.write_byte(color.2);
        }
        self.end_frame();

        Ok(())
    }

    fn as_slice(&self) -> &[RGB] {
        self.frame.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [RGB] {
        self.frame.as_mut_slice()
    }
}
