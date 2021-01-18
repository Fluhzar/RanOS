//! # Doc
//!
//! This crate contains code to write out the basic forms of all types that are serializable.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

/// Writes default config files to `Code/ignore`.
///
/// Note: the folder `ignore` as well as its sub-folders `generator`, `display`, and `draw` must all exist before this is run.
pub fn write_base_rons() {
    generator::breath();
    generator::cycle();
    generator::rainbow();
    generator::solid();
    generator::strobe();
    generator::generator();

    display::display();

    draw::null();
    draw::pi();
    draw::term();
    draw::draw();
}

pub(self) mod generator {
    use std::{fs::File, time::Duration};

    use ranos_generator::{GeneratorBuilder, Breath, ColorOrder, Cycle, Rainbow, Solid, Strobe};
    use ranos_ds::rgb::{RGBOrder, RGB};

    pub(super) fn breath() {
        let pretty = ron::ser::PrettyConfig::default();

        // breath_random
        {
            let file = File::create("ignore/generator/breath_random.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Breath::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .breath_duration(Duration::from_secs(4))
                    .order(ColorOrder::Random) as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }

        // breath_random_bright
        {
            let file = File::create("ignore/generator/breath_random_bright.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Breath::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .breath_duration(Duration::from_secs(4))
                    .order(ColorOrder::RandomBright)
                    as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }

        // breath_ordered
        {
            let file = File::create("ignore/generator/breath_ordered.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Breath::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .breath_duration(Duration::from_secs(4))
                    .order(ColorOrder::Ordered(vec![
                        RGB::from_hsv(0.0, 1.0, 1.0),
                        RGB::from_hsv(60.0, 1.0, 1.0),
                        RGB::from_hsv(120.0, 1.0, 1.0),
                        RGB::from_hsv(180.0, 1.0, 1.0),
                        RGB::from_hsv(240.0, 1.0, 1.0),
                        RGB::from_hsv(300.0, 1.0, 1.0),
                    ])) as Box<dyn GeneratorBuilder>),
                pretty,
            )
            .unwrap();
        }
    }

    pub(super) fn cycle() {
        let pretty = ron::ser::PrettyConfig::default();

        // cycle_random
        {
            let file = File::create("ignore/generator/cycle_random.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Cycle::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .cycle_period(Duration::from_secs_f64(0.25))
                    .order(ColorOrder::Random) as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }

        // cycle_random_bright
        {
            let file = File::create("ignore/generator/cycle_random_bright.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Cycle::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .cycle_period(Duration::from_secs_f64(0.25))
                    .order(ColorOrder::RandomBright)
                    as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }

        // cycle_ordered
        {
            let file = File::create("ignore/generator/cycle_ordered.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Cycle::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .cycle_period(Duration::from_secs_f64(0.25))
                    .order(ColorOrder::Ordered(vec![
                        RGB::from_code(0xFF_00_00, ranos_ds::rgb::RGBOrder::RGB),
                        RGB::from_code(0x00_FF_00, ranos_ds::rgb::RGBOrder::RGB),
                        RGB::from_code(0x00_00_FF, ranos_ds::rgb::RGBOrder::RGB),
                    ])) as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn rainbow() {
        let pretty = ron::ser::PrettyConfig::default();

        // rainbow
        {
            let file = File::create("ignore/generator/rainbow.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Rainbow::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .rainbow_length(Duration::from_secs(4))
                    .saturation(1.0)
                    .value(1.0)
                    .arc(1.0)
                    .step(1) as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn solid() {
        let pretty = ron::ser::PrettyConfig::default();

        // solid
        {
            let file = File::create("ignore/generator/solid.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Solid::builder()
                    .runtime(Duration::from_secs(8))
                    .color(RGB::from_code(0x00_FF_FF, RGBOrder::RGB))
                ),
                pretty
            )
            .unwrap();
        }
    }

    pub(super) fn strobe() {
        let pretty = ron::ser::PrettyConfig::default();

        // strobe
        {
            let file = File::create("ignore/generator/strobe.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Strobe::builder()
                    .runtime(Duration::from_secs_f64(8.0))
                    .period(Duration::from_secs(1))
                    .duty(0.5)
                    .color(RGB::from_code(0xFF_FF_FF, RGBOrder::RGB))
                    as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn generator() {
        let pretty = ron::ser::PrettyConfig::default();

        // generator
        {
            let file = File::create("ignore/generator/generator.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Cycle::builder() as Box<dyn GeneratorBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }
}

pub(self) mod display {
    use std::{fs::File, time::Duration};

    use ranos_generator::{Breath, ColorOrder, Rainbow};
    use ranos_display::Display;

    pub(super) fn display() {
        let pretty = ron::ser::PrettyConfig::default();

        // display
        {
            let file = File::create("ignore/display/display.ron").unwrap();

            ron::ser::to_writer_pretty(file, &Display::builder(), pretty.clone()).unwrap();
        }

        // display_with_generators
        {
            let file = File::create("ignore/display/display_with_generators.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &Display::builder()
                    .generator(
                        Rainbow::builder()
                            .runtime(Duration::from_secs_f64(8.0))
                            .rainbow_length(Duration::from_secs(4))
                            .saturation(1.0)
                            .value(1.0)
                            .arc(1.0)
                            .step(1),
                    )
                    .generator(
                        Breath::builder()
                            .runtime(Duration::from_secs_f64(8.0))
                            .breath_duration(Duration::from_secs(4))
                            .order(ColorOrder::Random),
                    ),
                pretty.clone(),
            )
            .unwrap();
        }
    }
}

pub(self) mod draw {
    use std::{fs::File, time::Duration};

    use ranos_generator::{Breath, ColorOrder, Rainbow};
    use ranos_core::Timer;
    use ranos_display::Display;
    use ranos_draw::{APA102CPiDraw, DrawBuilder, NullDraw, TermDraw};

    pub(super) fn null() {
        let pretty = ron::ser::PrettyConfig::default();

        // null
        {
            let file = File::create("ignore/draw/null.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(NullDraw::builder().timer(Timer::new(Some(Duration::from_secs_f64(1.0 / 60.0))))
                    as Box<dyn DrawBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn pi() {
        let pretty = ron::ser::PrettyConfig::default();

        // pi
        {
            let file = File::create("ignore/draw/pi.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(APA102CPiDraw::builder()
                    .timer(Timer::new(Some(Duration::from_secs_f64(1.0 / 60.0))))
                    as Box<dyn DrawBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn term() {
        let pretty = ron::ser::PrettyConfig::default();

        // term
        {
            let file = File::create("ignore/draw/term.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(TermDraw::builder()
                    .max_width(8)
                    .timer(Timer::new(Some(Duration::from_secs_f64(1.0 / 60.0))))
                    as Box<dyn DrawBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn draw() {
        let pretty = ron::ser::PrettyConfig::default();

        // draw_full
        {
            let file = File::create("ignore/draw/draw_full.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(APA102CPiDraw::builder()
                    .timer(Timer::new(Some(Duration::from_secs_f64(1.0 / 60.0))))
                    as Box<dyn DrawBuilder>)
                    .display(
                        Display::builder()
                            .generator(
                                Rainbow::builder()
                                    .runtime(Duration::from_secs_f64(8.0))
                                    .rainbow_length(Duration::from_secs(4))
                                    .saturation(1.0)
                                    .value(1.0)
                                    .arc(1.0)
                                    .step(1),
                            )
                            .generator(
                                Breath::builder()
                                    .runtime(Duration::from_secs_f64(8.0))
                                    .breath_duration(Duration::from_secs(4))
                                    .order(ColorOrder::Random),
                            ),
                    ),
                pretty.clone(),
            )
            .unwrap();
        }
    }
}
