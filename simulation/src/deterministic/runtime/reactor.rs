//! Reactor module

use crate::buggify::disable_buggify;
use crate::deterministic::runtime::executor::TaskWaker;
use crate::deterministic::time::DeterministicTime;
use crossbeam_queue::ArrayQueue;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::cmp::Ordering;
use std::sync::Arc;
use std::task::{Wake, Waker};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Reactor {
    time: DeterministicTime,
    waits: Arc<Mutex<Vec<ReactorEntry>>>,
}

impl Default for Reactor {
    fn default() -> Reactor {
        Reactor {
            time: DeterministicTime::new(),
            waits: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Reactor {
    /// Returns a reference to the reactor.
    pub(crate) fn get() -> &'static Reactor {
        static REACTOR: Lazy<Reactor> = Lazy::new(|| Reactor::default());
        &REACTOR
    }

    pub fn get_deterministic_time(&self) -> DeterministicTime {
        self.time.clone()
    }

    pub fn register_wait(&self, duration: Duration, waker: Waker) {
        tracing::trace!("registering a wait for {:?}", duration);
        self.waits.lock().push(ReactorEntry::new(duration, waker));
    }

    /// Advancing simulation. It will chose the next Instant stored in  `waits` and apply it
    /// on the deterministicTime.
    pub fn advance_simulation(&self) -> Option<Duration> {
        let mut lock = self.waits.lock();
        if !lock.is_empty() {
            // sort entry per duration
            lock.sort();

            // get next wait
            let next = lock.remove(0);

            tracing::trace!("advancing from {:?}", next.duration);
            self.time.advance(next.duration);
            next.waker.wake();
            Some(next.duration)
        } else {
            None
        }
    }
}

struct ReactorEntry {
    duration: Duration,
    waker: Waker,
}

impl ReactorEntry {
    pub fn new(duration: Duration, waker: Waker) -> ReactorEntry {
        ReactorEntry { duration, waker }
    }
}

impl PartialOrd for ReactorEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.duration.partial_cmp(&other.duration)
    }
}

impl PartialEq for ReactorEntry {
    fn eq(&self, other: &Self) -> bool {
        self.duration.eq(&other.duration)
    }

    fn ne(&self, other: &Self) -> bool {
        self.duration.ne(&other.duration)
    }
}

impl Eq for ReactorEntry {}
impl Ord for ReactorEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.duration.cmp(&other.duration)
    }
}
