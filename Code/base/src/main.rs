use base::draw::*;
use base::runner::Runner;
use base::runner::breath::{Breath, ColorOrder};
use base::runner::rainbow::Rainbow;
use std::time::Duration;

#[cfg(feature = "pi_draw")]
use {rppal::*, base::draw::pi_draw::APA102CPiDraw};

#[cfg(feature = "term_draw")]
use base::draw::term_draw::TermDraw;

fn main() {
    let mut stats = DrawStats::new();

    /*loop*/ {
        {
            let drawer: Box<dyn Draw> = {
                #[cfg(feature = "pi_draw")] {
                    let gpio = gpio::Gpio::new().unwrap();
                    Box::new(APA102CPiDraw::new(gpio.get(17).unwrap().into_output(), gpio.get(27).unwrap().into_output(), 0.125, 256)) as Box<dyn Draw>
                }
                #[cfg(feature = "term_draw")]
                {
                    Box::new(TermDraw::new(16, 1.0, 256)) as Box<dyn Draw>
                }
            };

            //use base::util::rgb::RGB;
            //let order = ColorOrder::Ordered(vec![RGB::from_hsv(0.0, 1.0, 1.0), RGB::from_hsv(30.0, 1.0, 1.0), RGB::from_hsv(60.0, 1.0, 1.0), RGB::from_hsv(120.0, 1.0, 1.0), RGB::from_hsv(210.0, 1.0, 1.0), RGB::from_hsv(280.0, 1.0,1.0)]);
            let order = ColorOrder::Random;

            let breath = Breath::new(Duration::from_secs(2), order);
            let mut breath_runner = Runner::new(breath, drawer, None/*Some(Duration::from_secs_f64(1.0/144.0))*/, Duration::from_secs(16));

            if let Err(s) = breath_runner.run() {
                stats += breath_runner.stats();
                println!("{}\nExiting", s);
                return;//break;
            } else {
                stats += breath_runner.stats();
            }
        }

        {
           let drawer: Box<dyn Draw> = {
                #[cfg(feature = "pi_draw")]
                {
                    let gpio = gpio::Gpio::new().unwrap();
                    Box::new(APA102CPiDraw::new(gpio.get(17).unwrap().into_output(), gpio.get(27).unwrap().into_output(), 0.125, 256)) as Box<dyn Draw>
                }
                #[cfg(feature = "term_draw")]
                {
                    Box::new(TermDraw::new(16, 1.0, 256)) as Box<dyn Draw>
                }
            };

            let rainbow = Rainbow::new(Duration::from_secs_f64(5.0), 1.0, 1.0, 1.0, 8*1);
            let mut rainbow_runner = Runner::new(rainbow, drawer, None/*Some(Duration::from_secs_f64(1.0/144.0))*/, Duration::from_secs(16));

            if let Err(s) = rainbow_runner.run() {
                stats += rainbow_runner.stats();
                println!("{}\nExiting", s);
                return;//break;
            } else {
                stats += rainbow_runner.stats();
            }
        }
    }

    println!("{}", stats);
}
