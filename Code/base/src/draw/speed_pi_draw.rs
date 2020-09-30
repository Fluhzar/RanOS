use rppal::gpio;
use std::collections::VecDeque;
use std::time::Duration;

use crate::ds::collections::frame::Frame;
use crate::ds::rgb::*;
use crate::util::{Info, Timer};
use super::pi_draw::Pin;

use super::*;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SpeedPiDrawInfo();

impl Info for SpeedPiDrawInfo {
    fn new() -> Box<dyn Info> {
        Box::new(SpeedPiDrawInfo::default())
    }

    fn name(&self) -> String {
        "SpeedPiDraw".to_owned()
    }

    fn details(&self) -> String {
        "Like PiDraw, but with Speed.".to_owned()
    }
}

#[inline]
fn bit_to_level(data: u32) -> gpio::Level {
    if data & 0x80_00_00_00 != 0 {
        gpio::Level::High
    } else {
        gpio::Level::Low
    }
}

#[derive(Debug)]
pub struct SpeedPiDraw {
    data: [Pin ; 4],
    clock: [Pin ; 4],

    queue: VecDeque<Box<dyn Animation>>,
    timer: Timer,

    known_len: usize,

    stats: DrawStats,
}

impl SpeedPiDraw {
    pub fn new(data: [Pin ; 4], clock: [Pin ; 4]) -> Self {
        Self {
            data,
            clock,

            queue: VecDeque::new(),
            timer: Timer::new(None),

            known_len: 0,

            stats: DrawStats::new(),
        }
    }

    #[inline]
    fn start_frame(&mut self) {
        self.set_pins_low();

        self.write_32(0x00_00_00_00);
    }

    #[inline]
    fn end_frame(&mut self, len: usize) {
        for _ in 0..(len >> 4) {
            self.write_32(0x00_00_00_00);
        }
    }

    #[inline]
    fn write_32(&mut self, mut data: u32) {
        for i in 0..32 {
            self.data[i%4].write(bit_to_level(data));
            self.clock[i%4].toggle();
            self.clock[i%4].toggle();
            data <<= 1; // Data output in MSB-first order
        }
    }

    #[inline]
    fn set_pins_low(&mut self) {
        for p in &mut self.data {
            p.set_low();
        }
        for p in &mut self.clock {
            p.set_low();
        }
    }

    fn stop(&mut self, len: usize) {
        self.start_frame();

        for _ in 0..len {
            self.write_32(0xE0_00_00_00);
        }

        self.end_frame(len);
    }

    fn write_frame(&mut self, frame: &Frame) {
        self.start_frame();

        for led in frame.iter() {
            let color = led.as_tuple(RGBOrder::BGR);

            self.write_32(
                ((0xE0 as u32 | frame.brightness_apa102c() as u32) << 24) |
                ((color.0 as u32) << 16) |
                ((color.1 as u32) << 8) |
                ((color.0 as u32) << 0)
            );
        }

        self.end_frame(frame.len());
    }
}

impl Draw for SpeedPiDraw {
    fn push_queue(&mut self, a: Box<dyn Animation>) {
        self.queue.push_back(a);
    }

    fn queue_len(&self) -> usize {
        self.queue.len()
    }

    fn run(&mut self) -> Vec<Box<dyn Animation>> {
        self.timer.reset();
        self.stats.reset();

        let zero_duration = Duration::new(0, 0);

        let mut out = Vec::with_capacity(self.queue.len());

        while let Some(mut ani) = self.queue.pop_front() {
            while ani.time_remaining() > zero_duration {
                ani.update(self.timer.ping());
                self.write_frame(ani.frame());

                self.stats.inc_frames();
            }

            self.stats.set_num(ani.frame().len());
            self.stats.end();

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

impl Drop for SpeedPiDraw {
    fn drop(&mut self) {
        self.stop(self.known_len);
    }
}

impl Default for SpeedPiDraw {
    fn default() -> Self {
        let gpio = gpio::Gpio::new().unwrap();
        Self::new(
            [
                gpio.get(25).unwrap().into_output(),
                gpio.get( 8).unwrap().into_output(),
                gpio.get( 7).unwrap().into_output(),
                gpio.get( 1).unwrap().into_output(),
            ],
            [
                gpio.get( 5).unwrap().into_output(),
                gpio.get( 6).unwrap().into_output(),
                gpio.get(13).unwrap().into_output(),
                gpio.get(19).unwrap().into_output(),
            ]
        )
    }
}
