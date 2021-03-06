//! Deterministic time
use parking_lot::Mutex;
use std::sync::Arc;
use std::time;

#[derive(Debug, Clone)]
/// A mock source of time, allowing for deterministic control of the progress
/// of time.
pub struct DeterministicTime {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
struct Inner {
    /// Time basis for which mock time is derived.
    base: time::Instant,
    /// The amount of mock time which has elapsed.
    advance: time::Duration,
}

impl Inner {
    /// create a new Inner
    pub fn new() -> Self {
        Self {
            base: time::Instant::now(),
            advance: time::Duration::from_millis(0),
        }
    }
}

impl Default for Inner {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DeterministicTime {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicTime {
    /// create a new `DeterministicTime`
    pub fn new() -> Self {
        Self {
            inner: Arc::from(Mutex::new(Inner::default())),
        }
    }

    /// advance of some duration
    pub fn advance(&self, duration: time::Duration) {
        self.inner.lock().advance += duration;
    }

    /// return base+advance time
    pub fn now(&self) -> time::Instant {
        let lock = self.inner.lock();
        lock.base + lock.advance
    }

    /// reset time
    pub fn reset(&mut self) {
        let mut lock = self.inner.lock();
        lock.base = time::Instant::now();
        lock.advance = time::Duration::from_millis(0);
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
        let mut time = DeterministicTime::default();
        let now = time::Instant::now();
        time.inner.lock().base = now;

        for i in 1..1000 {
            time.advance(Duration::from_secs(1));
            assert_eq!(now.add(Duration::from_secs(i)), time.now());
            assert_eq!(time.inner.lock().base, now);
        }

        time.reset();
        assert!(!time.inner.lock().base.eq(&now));
        dbg!(&time);
    }
}
