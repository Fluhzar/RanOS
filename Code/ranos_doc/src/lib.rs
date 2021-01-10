//! # App
//!
//! This module contains the application interface that controls the LEDs.

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]
#![warn(clippy::all)]

#[macro_use]
extern crate lazy_static;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub mod config;

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

struct App {

}
