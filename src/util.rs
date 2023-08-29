use std::time::{Duration, Instant};

pub struct Timer {
    target_duration: Duration,
    starting_point: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer {
            target_duration: duration,
            starting_point: Instant::now(),
        }
    }

    pub fn set_duration(&mut self, target_duration: Duration) {
        self.target_duration = target_duration;
    }

    pub fn reset(&mut self) {
        self.starting_point = Instant::now();
    }

    pub fn has_elapsed(&mut self) -> bool {
        self.starting_point.elapsed() >= self.target_duration
    }

    pub fn has_elapsed_reset(&mut self) -> bool {
        let has_elapsed = self.has_elapsed();
        self.reset();
        has_elapsed
    }
}
