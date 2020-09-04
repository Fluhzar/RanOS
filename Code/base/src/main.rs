use base::animation::breath::{Breath, ColorOrder};
use base::animation::rainbow::Rainbow;
use base::draw::*;
use std::time::Duration;

#[cfg(feature = "pi_draw")]
use {base::draw::pi_draw::APA102CPiDraw, rppal::*};

#[cfg(feature = "term_draw")]
use base::draw::term_draw::TermDraw;

#[cfg(not(any(feature = "pi_draw", feature = "term_draw")))]
use base::draw::null_draw::NullDraw;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut size = 16; // Safe default value in case no args can be converted to a usize

    for arg in args {
        if let Ok(s) = arg.parse::<usize>() {
            size = s;
            break;
        }
    }

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
        use base::ds::rgb::RGB;
        ColorOrder::Ordered(vec![
            RGB::from_hsv(0.0, 1.0, 1.0),
            RGB::from_hsv(30.0, 1.0, 1.0),
            RGB::from_hsv(60.0, 1.0, 1.0),
            RGB::from_hsv(120.0, 1.0, 1.0),
            RGB::from_hsv(210.0, 1.0, 1.0),
            RGB::from_hsv(280.0, 1.0, 1.0),
        ])
    };

    let breath = Breath::new(
        Duration::from_secs(16),
        Duration::from_secs(4),
        1.0,
        size,
        order,
    );
    let rainbow = Rainbow::new(
        Duration::from_secs(16),
        Duration::from_secs_f64(5.0),
        1.0,
        size,
        1.0,
        1.0,
        1.0,
        1,
    );

    loop {
        drawer.push_queue(Box::new(breath.clone()));
        drawer.push_queue(Box::new(rainbow.clone()));

        if let Err(s) = drawer.run() {
            eprintln!("\nUnexpected exit: {}", s);
            return;
        } else {
            println!("\n{}", drawer.stats());
        }
    }
}
