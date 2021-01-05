//! # App
//!
//! This module contains the application interface that controls the LEDs.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

#[macro_use]
extern crate lazy_static;

use clap;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use ranos_animation;
use ranos_draw::{Draw, DrawStats, draw_info, match_draw};
use ranos_core::{Timer, info};

lazy_static! {
    static ref SIGINT: Arc<AtomicBool> = {
        let arc = Arc::new(AtomicBool::new(false));

        {
            let arc = arc.clone();
            ctrlc::set_handler(move || arc.store(true, Ordering::Relaxed)).unwrap();
        }

        arc
    };
}

/// The app defining the interface to dynamically control the LEDs at runtime.
///
/// # Command-line Arguments
///
/// This app uses [`clap`][0] to handle command-line argument parsing, with the
/// following help message describing the accepted options. An example of the
/// help message would be the following:
///
/// ```txt
/// RanOS LED Animation App 0.1.0
/// Fluhzar <fluhzar@pm.me>
/// Renders some animations through a given LED drawer.
/// 
/// USAGE:
///     ranos_app [FLAGS] [OPTIONS] --drawer <drawer>
/// 
/// FLAGS:
///     -h, --help       
///             Prints help information
/// 
///     -l, --loop       
///             Sets whether or not to loop the animations endlessly. If set, use SIGINT to terminate the program when the
///             currently running animation is finished or SIGTERM to end the program immediately.
///     -V, --version    
///             Prints version information
/// 
/// 
/// OPTIONS:
///     -a, --animation <animations>...    
///             Select the name of the animation(s) to use in the order you'd like, separated by a ',':
///             
///             Breath     Animates a breathing display that will either walk through a provided list of
///                        colors or select random colors, each color fading along a parabolic curve from
///                        black to the chosen color and back down to black.
///             
///             Cycle      Animates a static color for a given amount of time before cutting to the next
///                        color in a given order
///             
///             Rainbow    Classic RGB rainbow puke that we all know and love but instead of displaying on
///                        a fancy RGB keyboard it's just these stupid LEDs puking out everything.
///             
///             Strobe     Animates a flickering light similar to the strobe lights one might see at
///                        concerts or otherwise.
///              [possible values: breath, cycle, rainbow, strobe]
///     -b, --brightness <brightness>      
///             Sets the given brightness level the LEDs shall be set to when running. Must be a value in the range [0, 1].
/// 
///     -d, --drawer <drawer>              
///             Select the name of the drawer to use:
///             
///             PiDraw      Draws APA102C/SK9822 LEDs through a Raspberry Pi's GPIO pins. This
///                         implementation maintains compatibility with both APA102C and SK9822 LEDs.
///             
///             TermDraw    Emulates an LED display by writing whitespace with specified colored
///                         backgrounds to a terminal that supports full RGB colors.
///             
///             NullDraw    Drawer that doesn't have any form of output.
///              [possible values: pidraw, termdraw, nulldraw]
///     -s, --speed <speed>                
///             Sets the desired speed to run the program at. The value is interpreted as frames per second (FPS) and should
///             be a numerical value, e.g. 60, 29.97, etc. If this value is omitted, then the program will run at full speed
///             as fast as it can run on your hardware.
/// ```
///
/// [0]: clap
pub struct App {
    drawer: Box<dyn Draw>,
    looping: bool,
    should_exit: Arc<AtomicBool>,
}

impl App {
    /// Creates a new application built from the command-line parameters
    /// provided at runtime.
    pub fn new() -> Self {
        // This structure is used to allow the setting of option `&str`s for
        // `clap` to parse without having their owning `String`s be dropped too
        // early. The format of the keys is fairly self-explanatory, `A.x` where
        // `A` represents a group of options while `x` represents the option.
        let mut string_registrar = HashMap::new();

        string_registrar.insert("App.name", "RanOS LED Animation App".to_owned());
        string_registrar.insert("App.version", env!("CARGO_PKG_VERSION").to_owned());
        string_registrar.insert("App.author", "Fluhzar <fluhzar@pm.me>".to_owned());
        string_registrar.insert(
            "App.about",
            "Renders some animations through a given LED drawer.".to_owned(),
        );

        string_registrar.insert("AnimationArg.name", "animations".to_owned());
        string_registrar.insert("AnimationArg.short", "a".to_owned());
        string_registrar.insert("AnimationArg.long", "animation".to_owned());
        let ani_info = ranos_animation::animation_info();
        let ani_names_string: Vec<_> = ani_info.iter().map(|i| i.name().to_lowercase()).collect();
        let ani_names: Vec<_> = ani_names_string.iter().map(|s| s.as_str()).collect();
        string_registrar.insert("AnimationArg.ani_details", info::format_info(&ani_info, 80));
        string_registrar.insert("AnimationArg.help", "Select the name of the animation(s) to use in the order you'd like, separated by a ','".to_owned());
        string_registrar.insert(
            "AnimationArg.long_help",
            format!(
                "{}:\n{}",
                string_registrar.get("AnimationArg.help").unwrap(),
                string_registrar.get("AnimationArg.ani_details").unwrap()
            ),
        );

        string_registrar.insert("BrightnessArg.name", "brightness".to_owned());
        string_registrar.insert("BrightnessArg.short", "b".to_owned());
        string_registrar.insert("BrightnessArg.long", "brightness".to_owned());
        string_registrar.insert("BrightnessArg.help", "Sets the given brightness level the LEDs shall be set to when running. Must be a value in the range [0, 1].".to_owned());

        string_registrar.insert("DrawerArg.name", "drawer".to_owned());
        string_registrar.insert("DrawerArg.short", "d".to_owned());
        string_registrar.insert("DrawerArg.long", "drawer".to_owned());
        let draw_info = draw_info();
        let draw_names_string: Vec<_> = draw_info.iter().map(|i| i.name().to_lowercase()).collect();
        let draw_names: Vec<_> = draw_names_string.iter().map(|s| s.as_str()).collect();
        string_registrar.insert("DrawerArg.draw_details", info::format_info(&draw_info, 80));
        string_registrar.insert(
            "DrawerArg.help",
            "Select the name of the drawer to use".to_owned(),
        );
        string_registrar.insert(
            "DrawerArg.long_help",
            format!(
                "{}:\n{}",
                string_registrar.get("DrawerArg.help").unwrap(),
                string_registrar.get("DrawerArg.draw_details").unwrap()
            ),
        );

        string_registrar.insert("LoopingArg.name", "looping".to_owned());
        string_registrar.insert("LoopingArg.short", "l".to_owned());
        string_registrar.insert("LoopingArg.long", "loop".to_owned());
        string_registrar.insert("LoopingArg.help", "Sets whether or not to loop the animations endlessly. If set, use SIGINT to terminate the program when the currently running animation is finished or SIGTERM to end the program immediately.".to_owned());

        string_registrar.insert("SpeedArg.name", "speed".to_owned());
        string_registrar.insert("SpeedArg.short", "s".to_owned());
        string_registrar.insert("SpeedArg.long", "speed".to_owned());
        string_registrar.insert("SpeedArg.help", "Sets the desired speed to run the program at in frames per second (FPS), e.g. 60, 29.97, etc.".to_owned());
        string_registrar.insert("SpeedArg.long_help", "Sets the desired speed to run the program at. The value is interpreted as frames per second (FPS) and should be a numerical value, e.g. 60, 29.97, etc. If this value is omitted, then the program will run at full speed as fast as it can run on your hardware.".to_owned());

        string_registrar.insert("SizeArg.name", "size".to_owned());
        string_registrar.insert("SizeArg.short", "n".to_owned()); // `n` short for `number`
        string_registrar.insert("SizeArg.long", "size".to_owned());
        string_registrar.insert("SizeArg.help", "Sets the desired number of pixels to run. Must be a positive integer.".to_owned());

        // Create the app
        let app = clap::App::new(string_registrar.get("App.name").unwrap())
            .version(string_registrar.get("App.version").unwrap().as_str())
            .author(string_registrar.get("App.author").unwrap().as_str())
            .about(string_registrar.get("App.about").unwrap().as_str())
            // Add Animation options
            .arg(
                clap::Arg::with_name(string_registrar.get("AnimationArg.name").unwrap())
                    .short(string_registrar.get("AnimationArg.short").unwrap())
                    .long(string_registrar.get("AnimationArg.long").unwrap())
                    .takes_value(true)
                    .multiple(true)
                    .use_delimiter(false)
                    .possible_values(ani_names.as_slice())
                    .help(string_registrar.get("AnimationArg.help").unwrap())
                    .long_help(string_registrar.get("AnimationArg.long_help").unwrap()),
            )
            // Add Brightness options
            .arg(
                clap::Arg::with_name(string_registrar.get("BrightnessArg.name").unwrap())
                    .short(string_registrar.get("BrightnessArg.short").unwrap())
                    .long(string_registrar.get("BrightnessArg.long").unwrap())
                    .takes_value(true)
                    .multiple(false)
                    .help(string_registrar.get("BrightnessArg.help").unwrap()),
            )
            // Add Drawer options
            .arg(
                clap::Arg::with_name(string_registrar.get("DrawerArg.name").unwrap())
                    .short(string_registrar.get("DrawerArg.short").unwrap())
                    .long(string_registrar.get("DrawerArg.long").unwrap())
                    .required(true)
                    .takes_value(true)
                    .use_delimiter(false)
                    .number_of_values(1)
                    .possible_values(draw_names.as_slice())
                    .help(string_registrar.get("DrawerArg.help").unwrap())
                    .long_help(string_registrar.get("DrawerArg.long_help").unwrap()),
            )
            // Add looping flag
            .arg(
                clap::Arg::with_name(string_registrar.get("LoopingArg.name").unwrap())
                    .short(string_registrar.get("LoopingArg.short").unwrap())
                    .long(string_registrar.get("LoopingArg.long").unwrap())
                    .help(string_registrar.get("LoopingArg.help").unwrap()),
            )
            // Add speed option
            .arg(
                clap::Arg::with_name(string_registrar.get("SpeedArg.name").unwrap())
                    .short(string_registrar.get("SpeedArg.short").unwrap())
                    .long(string_registrar.get("SpeedArg.long").unwrap())
                    .help(string_registrar.get("SpeedArg.help").unwrap())
                    .takes_value(true)
                    .long_help(string_registrar.get("SpeedArg.long_help").unwrap()),
            )
            // Add size option
            .arg(
                clap::Arg::with_name(string_registrar.get("SizeArg.name").unwrap())
                    .required(true)
                    .short(string_registrar.get("SizeArg.short").unwrap())
                    .long(string_registrar.get("SizeArg.long").unwrap())
                    .help(string_registrar.get("SizeArg.help").unwrap())
                    .takes_value(true)
            )
        ;

        let matches = app.get_matches();

        let animations: Vec<_> = matches
            .values_of(string_registrar.get("AnimationArg.name").unwrap())
            .unwrap()
            .map(|a| ranos_animation::match_animation(a).unwrap())
            .collect();

        let drawer_builder = match_draw(matches.value_of(string_registrar.get("DrawerArg.name").unwrap()).unwrap()).unwrap();

        let speed = if let Some(s) = matches.value_of(string_registrar.get("SpeedArg.name").unwrap()) {
            if let Ok(s) = s.parse::<f32>() {
                Timer::new(Some(std::time::Duration::from_secs_f32(1.0/s)))
            } else {
                Timer::new(None)
            }
        } else {
            Timer::new(None)
        };

        let brightness = if let Some(b) = matches.value_of(string_registrar.get("BrightnessArg.name").unwrap()) {
            if let Ok(b) = b.parse::<f32>() {
                b
            } else {
                0.25
            }
        } else {
            0.25
        };

        let size = matches.value_of(string_registrar.get("SizeArg.name").unwrap()).unwrap().parse::<usize>().unwrap();

        let mut drawer = drawer_builder.build(speed, brightness, size);

        for a in animations {
            drawer.push_queue(a);
        }

        let looping = matches.is_present(string_registrar.get("LoopingArg.name").unwrap());

        Self {
            drawer,
            looping,
            should_exit: SIGINT.clone(),
        }
    }

    /// Runs the application, cycling through the provided animations until the
    /// program has completed.
    ///
    /// If the `looping` option was set then this will loop through the
    /// animations until SIGINT is signalled, where it will terminate after the
    /// current animation finishes, or SIGTERM is signalled, which terminates
    /// the program automatically.
    pub fn run(&mut self) {
        let mut total_stats = DrawStats::new();

        loop {
            let anis = self.drawer.run();
            total_stats += self.drawer.stats();

            // If an interrupt has occurred, exit the run function.
            if self.should_exit.load(Ordering::Relaxed) == true {
                self.looping = false;
            }

            if self.looping {
                for mut a in anis {
                    a.reset();
                    self.drawer.push_queue(a);
                }
            } else {
                break;
            }
        }

        println!("\n{}", total_stats);
    }
}
