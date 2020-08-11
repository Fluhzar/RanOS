//! # Runner

pub mod breath;
pub mod rainbow;

use crate::draw::Draw;
use crate::util::rgb::RGB;
use crate::util::timer::Timer;
use std::time::{Instant, Duration};

/// Trait for types that implement animations that sets the LEDs to a given
/// frame of the animation before being drawn.
pub trait Animation {
    /// Updates the frame with the next frame of the animation given the input `dt`.
    fn update(&mut self, dt: Duration, frame: &mut [RGB]);
}

/// This struct is the manager of all the other systems. It ensures that an
/// animation is updated before being drawn and properly tracks the passage of
/// time to provide accurate delta-time readings to the animation.
pub struct Runner<A>//, D>
where
    A: Animation,
    //D: Draw
{
    animation: A,
    drawer: Box<dyn Draw>,//D,
    timer: Timer,
    max_duration: Duration,
}

impl<A>/*, D>*/ Runner<A>//, D>
where
    A: Animation,
    //D: Draw
{
    /// Constructs a `Runner` from the given animation and drawer, and with a
    /// maximum duration that `Runner::run` is allowed to run for.
    pub fn new(animation: A, drawer: Box<dyn Draw>, target_frame_duration: Option<Duration>, max_duration: Duration) -> Self {
        Self {
            animation,
            drawer,
            timer: Timer::new(target_frame_duration),
            max_duration,
        }
    }

    /// Runs for the set max duration of time, updating the animation each loop
    /// before drawing the animation.
    pub fn run(&mut self) -> Result<(), String> {
        let begin = Instant::now();

        while Instant::now() - begin < self.max_duration {
            let dt = self.timer.ping();
            self.animation.update(dt, self.drawer.as_mut_slice());
            self.drawer.write_frame()?;
        }

        Ok(())
    }
}
