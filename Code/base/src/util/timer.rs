//! # Timer

use std::time::{Instant, Duration};

/// Timer struct that will keep track of the time spent between pings.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Timer {
    ctime: Instant,
    ptime: Instant,
    dt: Duration,
    target_dt: Option<Duration>
}

impl Timer {
    /// Creates a new `Timer` object with the given optional target delta time.
    pub fn new(target_dt: Option<Duration>) -> Self {
        Self {
            ctime: Instant::now(),
            ptime: Instant::now(),
            dt: Duration::new(0,0),
            target_dt,
        }
    }

    /// Pings the timer, returning the amount of time that has passed since the
    /// last ping, optionally waiting for the `target_dt` duration to pass.
    pub fn ping(&mut self) -> Duration {
        if let Some(target_dt) = self.target_dt {
            while Instant::now() - self.ptime < target_dt {}
        }

        self.ptime = self.ctime;
        self.ctime = Instant::now();

        self.dt = self.ctime - self.ptime;

        self.dt
    }
}
