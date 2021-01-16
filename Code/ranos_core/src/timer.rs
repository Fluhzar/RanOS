//! Type for tracking the passage of time and intervals of time.

use std::{
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

/// Statistical tracker for the [`Timer`] struct. Tracks start and end
/// time as well as the number of pings encountered by the timer.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct TimerStats {
    start: Instant,
    end: Instant,
    pings: usize,
}

impl TimerStats {
    /// Creates a new stat object.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            end: Instant::now(),
            pings: 0,
        }
    }

    /// Notifies the stat tracker of the start time.
    pub fn start(&mut self) {
        self.start = Instant::now()
    }

    /// Notifies the stat tracker of the end time.
    pub fn end(&mut self) {
        self.end = Instant::now()
    }

    /// Notifies the stat tracker of a ping occurrence.
    pub fn ping(&mut self) {
        self.pings += 1;
    }

    /// Resets the timer.
    pub fn reset(&mut self) {
        *self = TimerStats::new();
    }
}

impl Display for TimerStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let duration = (self.end - self.start).as_secs_f32();
        write!(f, "Duration: {}\n", duration)?;
        write!(f, "Pings: {}\n", self.pings)?;
        write!(
            f,
            "Average ping rate: {} pings/s\n",
            self.pings as f32 / duration
        )
    }
}

fn default_instant() -> Instant {
    Instant::now()
}

/// Timer struct that will keep track of the time spent between pings.
#[derive(Debug, Copy, Clone, PartialOrd, Serialize, Deserialize)]
pub struct Timer {
    #[serde(skip, default = "TimerStats::new")]
    stats: TimerStats,
    #[serde(skip, default = "default_instant")]
    ctime: Instant,
    #[serde(skip, default = "default_instant")]
    ptime: Instant,
    #[serde(skip)]
    dt: Duration,
    target_dt: Option<Duration>,
}

impl Timer {
    /// Creates a new [`Timer`] object with the given optional target delta time.
    pub fn new(target_dt: Option<Duration>) -> Self {
        Self {
            stats: TimerStats::new(),
            ctime: Instant::now(),
            ptime: Instant::now(),
            dt: Duration::new(0, 0),
            target_dt,
        }
    }

    /// Allows immutable access to the internal stat tracker, typically for display purposes.
    pub fn stats(&self) -> &TimerStats {
        &self.stats
    }

    /// Resets the [`Timer`] to a brand-new state, as if it were just initialized.
    pub fn reset(&mut self) {
        *self = Timer::new(self.target_dt);
        self.stats.reset();
    }

    /// Pings the timer, returning the amount of time that has passed since the
    /// last ping, optionally waiting for the `target_dt` duration to pass.
    pub fn ping(&mut self) -> Duration {
        self.stats.ping();
        self.stats.end();

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

impl std::cmp::PartialEq<Timer> for Timer {
    fn eq(&self, other: &Timer) -> bool {
        self.target_dt == other.target_dt
    }
}

#[cfg(test)]
mod timer_test {
    use super::*;

    #[test]
    fn target_dt() {
        let mut acc_dt = Duration::new(0, 0);

        let max_iteration = 144;

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
