use base::animation::*;
use base::ds::rgb::*;
use base::draw::*;

use std::time::Duration;
use rppal::*;


fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut size = 16; // Safe default value in case no args can be converted to a usize

    for arg in args {
        if let Ok(s) = arg.parse::<usize>() {
            size = s;
            break;
        }
    }

    let brightness = 0.25;
    let size = size;

    let mut drawer: Box<dyn Draw> = {
        #[cfg(not(any(feature = "pi_draw", feature = "term_draw")))]
        {
            Box::new(NullDraw::new()) as Box<dyn Draw>
        }
        #[cfg(feature = "pi_draw")]
        {
            let gpio = gpio::Gpio::new().unwrap();
            Box::new(APA102CPiDraw::new(
                gpio.get(6).unwrap().into_output(),
                gpio.get(5).unwrap().into_output(),
            )) as Box<dyn Draw>
        }
        #[cfg(feature = "term_draw")]
        {
            Box::new(TermDraw::new((size as f64).sqrt().round() as usize)) as Box<dyn Draw>
        }
    };

    let random = false;

    let order: ColorOrder = if random {
        ColorOrder::Random
    } else {
        ColorOrder::Ordered(vec![
            RGB::from_hsv(0.0, 1.0, 1.0),
            RGB::from_hsv(30.0, 1.0, 1.0),
            RGB::from_hsv(60.0, 1.0, 1.0),
            RGB::from_hsv(120.0, 1.0, 1.0),
            RGB::from_hsv(210.0, 1.0, 1.0),
            RGB::from_hsv(280.0, 1.0, 1.0),
        ])
    };

    let breath = Breath::default();
    let rainbow = Rainbow::default();
    let strobe = Strobe::default();

    loop {
        drawer.push_queue(Box::new(breath.clone()));
        drawer.push_queue(Box::new(rainbow.clone()));
        drawer.push_queue(Box::new(strobe.clone()));

        if let Err(s) = drawer.run() {
            eprintln!("\nUnexpected exit: {}", s);
            return;
        } else {
            println!("\n{}", drawer.stats());
        }
    }
}
