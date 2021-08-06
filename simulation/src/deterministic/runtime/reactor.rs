//! Reactor module

use crate::deterministic::runtime::executor::TaskWaker;
use crate::deterministic::time::DeterministicTime;
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::sync::Arc;
use std::task::{Wake, Waker};
use std::time::{Duration, Instant};

pub struct Reactor {
    time: DeterministicTime,
    waits: Vec<ReactorEntry>,
}

impl Default for Reactor {
    fn default() -> Reactor {
        Reactor {
            time: DeterministicTime::new(),
            waits: vec![],
        }
    }
}

impl Reactor {
    pub fn register_wait(&mut self, duration: Duration, waker: Arc<TaskWaker>) {
        self.waits.push(ReactorEntry::new(duration, waker));
    }

    /// Advancing simulation. It will chose the next Instant stored in  `waits` and apply it
    /// on the deterministicTime.
    pub fn advance_simulation(&mut self) -> Option<Duration> {
        if !self.waits.is_empty() {
            // sorting waits per duration
            self.waits.sort();

            // get first
            match self.waits.first() {
                None => {}
                Some(entry) => {
                    self.time.advance(entry.duration);
                    entry.waker.clone().wake();
                }
            }
        }

        None
    }
}

struct ReactorEntry {
    duration: Duration,
    waker: Arc<TaskWaker>,
}

impl ReactorEntry {
    pub fn new(duration: Duration, waker: Arc<TaskWaker>) -> ReactorEntry {
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
