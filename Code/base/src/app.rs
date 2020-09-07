use clap;
use std::collections::HashMap;

use crate::animation;
use crate::draw;
use crate::util::{info, Info};

pub struct App {
    drawer: Box<dyn draw::Draw>,
    looping: bool,
}

impl App {
    pub fn new() -> Self {
        let mut string_registrar = HashMap::new();

        string_registrar.insert("App.name", "RanOS LED Animation App".to_owned());
        string_registrar.insert("App.version", env!("CARGO_PKG_VERSION").to_owned());
        string_registrar.insert("App.author", "Fluhzar <fluhzar@pm.me>".to_owned());
        string_registrar.insert("App.about", "Animates some animations through a given drawer.".to_owned());

        string_registrar.insert("LoopingArg.name", "looping".to_owned());
        string_registrar.insert("LoopingArg.short", "l".to_owned());
        string_registrar.insert("LoopingArg.long", "loop".to_owned());
        string_registrar.insert("LoopingArg.help", "Sets whether or not to loop the animations endlessly. If set, use SIGINT to terminate the program when the currently running animation is finished or SIGTERM to end the program immediately.".to_owned());

        string_registrar.insert("AnimationArg.name", "animations".to_owned());
        string_registrar.insert("AnimationArg.short", "a".to_owned());
        string_registrar.insert("AnimationArg.long", "animation".to_owned());
        let ani_info = animation::animation_info();
        let ani_names_string: Vec<_> = ani_info.iter()
            .map(|i| i.name().to_lowercase())
            .collect();
        let ani_names: Vec<_> = ani_names_string.iter().map(|s| s.as_str()).collect();
        string_registrar.insert("AnimationArg.ani_details", info::format_info(&ani_info, 80));
        string_registrar.insert("AnimationArg.help", "Select the name of the animation(s) to use in the order you'd like, separated by a ','".to_owned());
        string_registrar.insert("AnimationArg.long_help", format!("{}:\n{}", string_registrar.get("AnimationArg.help").unwrap(), string_registrar.get("AnimationArg.ani_details").unwrap()));

        string_registrar.insert("DrawerArg.name", "drawer".to_owned());
        string_registrar.insert("DrawerArg.short", "d".to_owned());
        string_registrar.insert("DrawerArg.long", "drawer".to_owned());
        let draw_info = draw::draw_info();
        let draw_names_string: Vec<_> = draw_info.iter()
            .map(|i| i.name().to_lowercase())
            .collect();
        let draw_names: Vec<_> = draw_names_string.iter().map(|s| s.as_str()).collect();
        string_registrar.insert("DrawerArg.draw_details", info::format_info(&draw_info, 80));
        string_registrar.insert("DrawerArg.help", "Select the name of the drawer to use.".to_owned());
        string_registrar.insert("DrawerArg.long_help", format!("{}:\n{}", string_registrar.get("DrawerArg.help").unwrap(), string_registrar.get("DrawerArg.draw_details").unwrap()));

        // Create the app
        let app = clap::App::new(
                string_registrar.get("App.name").unwrap()
            )
            .version(string_registrar.get("App.version").unwrap().as_str())
            .author(string_registrar.get("App.author").unwrap().as_str())
            .about(string_registrar.get("App.about").unwrap().as_str())
            // Add looping option
            .arg(
                clap::Arg::with_name(string_registrar.get("LoopingArg.name").unwrap())
                    .short(string_registrar.get("LoopingArg.short").unwrap())
                    .long(string_registrar.get("LoopingArg.long").unwrap())
                    .help(string_registrar.get("LoopingArg.help").unwrap())
            )
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
                    .long_help(string_registrar.get("AnimationArg.long_help").unwrap())
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
                    .long_help(string_registrar.get("DrawerArg.long_help").unwrap())
            );

        let matches = app.get_matches();

        let looping = matches.is_present("looping");
        let animations: Vec<_> = matches.values_of("animations").unwrap().map(|a| animation::match_animation(a).unwrap()).collect();
        let mut drawer = draw::match_draw(matches.value_of("drawer").unwrap()).unwrap();

        for a in animations {
            drawer.push_queue(a);
        }

        Self {
            drawer,
            looping,
        }
    }

    pub fn run(&mut self) {
        loop {
            let result = self.drawer.run();

            if let Err(s) = result {
                eprintln!("\nUnexpected exit: {}", s);
                return;
            } else {
                println!("\n{}", self.drawer.stats());
            }

            if self.looping {
                if let Ok(v) = result {
                    for a in v {
                        self.drawer.push_queue(a);
                    }
                }
            } else {
                break;
            }
        }
    }
}
