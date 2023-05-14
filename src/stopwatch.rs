use std::time::{Duration, Instant};

pub struct Stopwatch {
    start_time: Option<Instant>,
    elapsed: Duration,
}

impl Default for Stopwatch {
    fn default() -> Stopwatch {
        Stopwatch {
            start_time: None,
            elapsed: Duration::from_secs(0),
        }
    }
}

impl Stopwatch {
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.elapsed = self.elapsed();
        self.start_time = None;
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time
            .map_or(self.elapsed, |t| t.elapsed() + self.elapsed)
    }
}
