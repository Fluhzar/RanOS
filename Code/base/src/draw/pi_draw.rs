//! # Raspberry Pi Draw

use super::*;

use crate::util::{frame::Frame, rgb::RGB};
use rppal::gpio;
use std::cell::RefCell;

/// Type used to represent a GPIO pin with interior mutability.
pub type Pin = RefCell<gpio::OutputPin>;

/// Struct that draws [APA102C][0] LEDs through the Raspberry Pi's GPIO pins.
/// 
/// [0]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
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
    /// defined by the [datasheet][0]
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
    /// [this blog post][1].
    /// 
    /// [0]: https://cdn-shop.adafruit.com/datasheets/APA102.pdf
    /// [1]: https://cpldcpu.wordpress.com/2014/11/30/understanding-the-apa102-superled/
    #[inline]
    fn end_frame(&mut self, size: usize) {
        if size <= 64 {
            self.write_byte(0xFF);
            self.write_byte(0xFF);
            self.write_byte(0xFF);
            self.write_byte(0xFF);
        } else {
            for _ in 0..(size >> 4) {
                self.write_byte(0xFF);
            }
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
            self.write_byte(self.frame.brightness_apa102c());
            self.write_byte(led.blue());
            self.write_byte(led.green());
            self.write_byte(led.red());
        }
        self.end_frame(self.frame.len());

        Ok(())
    }

    fn as_slice(&self) -> &[RGB] {
        self.frame.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [RGB] {
        self.frame.as_mut_slice()
    }
}
