//! Deterministic time
use std::time;

#[derive(Debug)]
/// A mock source of time, allowing for deterministic control of the progress
/// of time.
struct DeterministicTime {
    /// Time basis for which mock time is derived.
    base: time::Instant,
    /// The amount of mock time which has elapsed.
    advance: time::Duration,
}

impl DeterministicTime {
    /// create a new `DeterministicTime`
    fn new() -> Self {
        Self {
            base: time::Instant::now(),
            advance: time::Duration::from_millis(0),
        }
    }

    /// advance of some duration
    fn advance(&mut self, duration: time::Duration) {
        self.advance += duration;
    }

    /// return base+advance time
    fn now(&self) -> time::Instant {
        self.base + self.advance
    }
}
#[cfg(test)]
mod tests {
    use crate::deterministic::time::DeterministicTime;
    use std::ops::Add;
    use std::time;
    use std::time::Duration;

    #[test]
    fn deterministic_random() {
        let mut time = DeterministicTime::new();
        let now = time::Instant::now();
        time.base = now.clone();

        for i in 1..1000 {
            time.advance(Duration::from_secs(1));
            assert_eq!(now.add(Duration::from_secs(i * 1)), time.now());
            assert_eq!(time.base, now);
        }
    }
}
