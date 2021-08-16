//! Reactor module

use crate::deterministic::time::DeterministicTime;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::cmp::Ordering;
use std::sync::Arc;
use std::task::Waker;
use std::time::Duration;

/// The DeterministicReactor is used to simulate "real I/O". It is only compatible with
/// simulation structures, as they cooperate with him. Instead of registering I/O to a loop,
/// simulation structures can only register timers. When the runtime cannot make any futures advances,
/// we can choose the smallest wait in the list and "advance time".
#[derive(Clone)]
pub struct DeterministicReactor {
    time: DeterministicTime,
    waits: Arc<Mutex<Vec<ReactorEntry>>>,
}

impl Default for DeterministicReactor {
    /// Create a default `DeterministicReactor`
    fn default() -> DeterministicReactor {
        DeterministicReactor {
            time: DeterministicTime::new(),
            waits: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl DeterministicReactor {
    /// Returns a reference to the reactor.
    pub(crate) fn get() -> &'static DeterministicReactor {
        static REACTOR: Lazy<DeterministicReactor> = Lazy::new(DeterministicReactor::default);
        &REACTOR
    }

    /// Returns the deterministic time used by the static reactor
    pub fn get_deterministic_time(&self) -> DeterministicTime {
        self.time.clone()
    }

    /// Register a wait
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

#[doc(hidden)]
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
}

impl Eq for ReactorEntry {}
impl Ord for ReactorEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.duration.cmp(&other.duration)
    }
}
