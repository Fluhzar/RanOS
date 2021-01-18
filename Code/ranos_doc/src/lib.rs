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
    generator::cycle();
    generator::rainbow();
    generator::solid();
    generator::generator();

    filter::strobe();
    filter::breath();
    filter::filter();

    display::display();

    draw::null();
    draw::pi();
    draw::term();
    draw::draw();
}

pub(self) mod generator {
    use std::{fs::File, time::Duration};

    use ranos_ds::rgb::{RGBOrder, RGB};
    use ranos_generator::{ColorOrder, Cycle, GeneratorBuilder, Rainbow, Solid};

    pub(super) fn cycle() {
        let pretty = ron::ser::PrettyConfig::default();

        // cycle_random
        {
            let file = File::create("ignore/generator/cycle_random.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Cycle::builder()
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
                &(Solid::builder().color(RGB::from_code(0x00_FF_FF, RGBOrder::RGB))),
                pretty,
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

pub(self) mod filter {
    use std::{fs::File, time::Duration};

    use ranos_filter::{self, Breath, FilterBuilder, Strobe};

    pub(super) fn breath() {
        let pretty = ron::ser::PrettyConfig::default();

        // breath
        {
            let file = File::create("ignore/filter/breath.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Breath::builder().breath_duration(Duration::from_secs(16))),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn strobe() {
        let pretty = ron::ser::PrettyConfig::default();

        // strobe
        {
            let file = File::create("ignore/filter/strobe.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Strobe::builder().frequency(1.0).duty(0.5) as Box<dyn FilterBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }

    pub(super) fn filter() {
        let pretty = ron::ser::PrettyConfig::default();

        // filter
        {
            let file = File::create("ignore/filter/filter.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &(Breath::builder() as Box<dyn FilterBuilder>),
                pretty.clone(),
            )
            .unwrap();
        }
    }
}

pub(self) mod display {
    use std::{fs::File, time::Duration};

    use ranos_display::{Display, Runtime};
    use ranos_filter::{Breath, Strobe};
    use ranos_generator::{ColorOrder, Cycle, Rainbow};

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
                            .rainbow_length(Duration::from_secs(4))
                            .saturation(1.0)
                            .value(1.0)
                            .arc(1.0)
                            .step(1),
                        Runtime::Time(Duration::from_secs(8)),
                    )
                    .generator(
                        Cycle::builder()
                            .cycle_period(Duration::from_secs_f32(0.25))
                            .order(ColorOrder::Random),
                        Runtime::Trigger,
                    ),
                pretty.clone(),
            )
            .unwrap();
        }

        // display_with_filters
        {
            let file = File::create("ignore/display/display_with_filters.ron").unwrap();

            ron::ser::to_writer_pretty(
                file,
                &Display::builder()
                    .filter(Breath::builder().breath_duration(Duration::from_secs(16)))
                    .filter(Strobe::builder().frequency(1.0).duty(0.5)),
                pretty.clone(),
            )
            .unwrap();
        }
    }
}

pub(self) mod draw {
    use std::{fs::File, time::Duration};

    use ranos_core::Timer;
    use ranos_display::{Display, Runtime};
    use ranos_draw::{APA102CPiDraw, DrawBuilder, NullDraw, TermDraw};
    use ranos_filter::Breath;
    use ranos_generator::{ColorOrder, Cycle, Rainbow};

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
                                    .rainbow_length(Duration::from_secs(4))
                                    .saturation(1.0)
                                    .value(1.0)
                                    .arc(1.0)
                                    .step(1),
                                Runtime::Time(Duration::from_secs(8)),
                            )
                            .generator(
                                Cycle::builder()
                                    .cycle_period(Duration::from_secs_f32(0.25))
                                    .order(ColorOrder::Random),
                                Runtime::Trigger,
                            )
                            .filter(Breath::builder().breath_duration(Duration::from_secs(16))),
                    ),
                pretty.clone(),
            )
            .unwrap();
        }
    }
}
