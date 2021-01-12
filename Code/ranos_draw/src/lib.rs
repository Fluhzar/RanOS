//! # Draw
//!
//! This module contains the types that will "draw" to the LEDs.
//!
//! There are two drawers defined in this module, one being the actual drawer
//! that will draw the colors to physical LEDs connected to a Raspberry Pi, and
//! the second is an emulated LED setup that draws to "LEDs" on the terminal
//! with a configurable number of "LEDs" per row.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

extern crate ranos_core;
extern crate ranos_display;

pub use null_draw::{NullDraw, NullDrawBuilder};
pub use term_draw::{TermDraw, TermDrawBuilder};

#[cfg(target_os = "linux")]
pub use pi_draw::{APA102CPiDraw, APA102CPiDrawBuilder, SK9822PiDraw, SK9822PiDrawBuilder};

use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}};

use ranos_core::{Timer, timer::TimerStats};
use ranos_display::{Display, DisplayBuilder};

pub mod null_draw;
pub mod term_draw;

#[cfg(target_os = "linux")]
pub mod pi_draw;

#[macro_use]
extern crate lazy_static;

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

/// Trait defining the ability to draw a frame of colors to LEDs.
pub trait Draw {
    /// Draws the internal frame to its destination.
    fn run(&mut self);

    /// Returns the statistics tracking object.
    fn stats(&self) -> &TimerStats;
}

/// Defines the behavior of a builder of a type that implements [`Draw`][crate::Draw].
///
/// Note: As the trait's functions return `Box<dyn DrawBuilder>` rather than `Box<Self>`, be sure to set any parameters for the
/// specific `Draw`-implementing type you're using before calling these functions, as the original type will be inaccessible
/// after calling one of the functions from this trait.
#[typetag::serde(tag = "type", content = "value")]
pub trait DrawBuilder {
    /// Sets the timer parameter from a pre-built object.
    fn timer(self: Box<Self>, timer: Timer) -> Box<dyn DrawBuilder>;

    /// Add a builder for a display that will be built at the same time as this builder.
    ///
    /// Be sure to add animations to the display builder before adding it to the drawer as it will be inaccessible afterwards.
    ///
    /// Note: Multiple [`DisplayBuilder`](ranos_display::DisplayBuilder)s can be added.
    fn display(self: Box<Self>, display: DisplayBuilder) -> Box<dyn DrawBuilder>;

    /// Builds [`Draw`][crate::Draw] object, returning it boxed up.
    fn build(self: Box<Self>) -> Box<dyn Draw>;
}

#[cfg(test)]
mod builder_test {
    use crate::{DrawBuilder, NullDraw};

    #[test]
    fn test_serialize() {
        let builder: Box<dyn DrawBuilder> = NullDraw::builder();

        let data = ron::ser::to_string(&builder).unwrap();

        // eprintln!("{}", data);
        assert_eq!(
            data,
            r#"(type:"NullDrawBuilder",value:(timer:(target_dt:None),displays:[]))"#
        );
    }

    #[test]
    fn test_deserialize() {
        let input =
            r#"(type:"NullDrawBuilder",value:(timer:(target_dt:None),displays:[]))"#;

        assert_eq!(
            ron::ser::to_string(
                &ron::de::from_str::<Box<dyn DrawBuilder>>(input).unwrap()
            )
            .unwrap(),
            input
        );
    }
}
