//! # Timer

use std::time::{Duration, Instant};

/// Timer struct that will keep track of the time spent between pings.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Timer {
    ctime: Instant,
    ptime: Instant,
    dt: Duration,
    target_dt: Option<Duration>,
}

impl Timer {
    /// Creates a new `Timer` object with the given optional target delta time.
    pub fn new(target_dt: Option<Duration>) -> Self {
        Self {
            ctime: Instant::now(),
            ptime: Instant::now(),
            dt: Duration::new(0, 0),
            target_dt,
        }
    }

    /// Resets the `Timer` to a brand-new state, as if it were just initialized.
    pub fn reset(&mut self) {
        *self = Timer::new(self.target_dt);
    }

    /// Pings the timer, returning the amount of time that has passed since the
    /// last ping, optionally waiting for the `target_dt` duration to pass.
    pub fn ping(&mut self) -> Duration {
        self.ptime = self.ctime;

        if let Some(target_dt) = self.target_dt {
            while (self.ctime - self.ptime) < target_dt {
                self.ctime = Instant::now();
            }
        } else {
            self.ctime = Instant::now();
        }

        self.dt = self.ctime - self.ptime;
        self.dt
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod timer_test {
    use super::*;

    #[test]
    fn target_dt() {
        let mut acc_dt = Duration::new(0, 0);

        let max_iteration = 1024 * 4;

        let mut timer = Timer::new(Some(Duration::from_secs_f64(1.0 / 144.0)));
        for _ in 0..max_iteration {
            acc_dt += timer.ping();
        }

        let t = acc_dt.as_secs_f64();
        let target_t = 1.0 / 144.0 * max_iteration as f64;

        eprintln!("expected t: {}\nactual t: {}", target_t, t);

        assert!((t - target_t).abs() < (1.0 / 144.0) * 0.5);
    }
}
