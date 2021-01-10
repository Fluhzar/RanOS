use std::time::Duration;

use ranos_app::*;
use ranos_core::*;
use ranos_animation::*;
use ranos_display::*;
use ranos_draw::{Draw, DrawBuilder, TermDraw};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        // Serialization
        {
            let file = std::fs::File::create(args[1].as_str()).unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(
                    TermDraw::builder()
                        .max_width(8)
                        .timer(Timer::new(Some(Duration::from_secs_f64(1.0/60.0))))
                        .display(
                            Display::builder()
                                .brightness(1.0)
                                .size(64)
                                .add_animation_builder(
                                    Rainbow::builder()
                                        .runtime(Duration::from_secs(8))
                                        .rainbow_length(Duration::from_secs(2))
                                        .saturation(1.0)
                                        .value(1.0)
                                        .arc(1.0)
                                        .step(8)
                                )
                        )
                    as Box<dyn DrawBuilder>
                ),
                ron::ser::PrettyConfig::default(),
            ).unwrap();
        }

        // Deserialization
        {
            let config = std::fs::File::open(args[1].as_str()).unwrap();
            ron::de::from_reader::<_, Box<dyn DrawBuilder>>(config).unwrap().build().run();
        }
    }
}
