//! Timer module

use crate::deterministic::runtime::reactor::DeterministicReactor;
use crate::deterministic::time::DeterministicTime;
use futures::Future;
use std::ops::Add;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A timer that can be used in simulation
pub struct DeterministicTimer {
    time: DeterministicTime,
    duration: Duration,
    expired_at: Instant,
    // TODO: Once
    already_registered: bool,
    reactor: DeterministicReactor,
}

impl DeterministicTimer {
    /// Wait in simulation
    #[allow(dead_code)]
    pub fn wait_with_reactor(
        time: DeterministicTime,
        reactor: DeterministicReactor,
        duration: Duration,
    ) -> DeterministicTimer {
        DeterministicTimer {
            time: time.clone(),
            duration,
            expired_at: time.now().add(duration),
            already_registered: false,
            reactor,
        }
    }

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if !self.already_registered {
            self.reactor
                .register_wait(self.duration, cx.waker().clone());
            self.already_registered = true;
        }

        let now = self.time.now();
        tracing::trace!("polling timer, it is now {:?}", now);
        if self.expired_at.le(&now) {
            tracing::trace!("firing timer with {:?}", self.duration);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Future for DeterministicTimer {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use crate::deterministic::runtime::executor::DeterministicExecutor;
    use crate::deterministic::runtime::reactor::DeterministicReactor;
    use crate::deterministic::runtime::task::Task;
    use crate::deterministic::runtime::timer::DeterministicTimer;
    use crate::deterministic::time::DeterministicTime;
    use std::time::{Duration, Instant};
    use tracing::Level;

    async fn example_task(
        reactor: DeterministicReactor,
        time: DeterministicTime,
        duration: Duration,
    ) {
        DeterministicTimer::wait_with_reactor(time, reactor, duration).await;
        println!("waited for {:?}", duration);
    }

    #[test]
    fn test_timer() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_test_writer()
            .try_init();

        let reactor = DeterministicReactor::default();
        let mut executor = DeterministicExecutor::new_with_reactor(reactor.clone());
        let time = reactor.get_deterministic_time();

        // spawning a future
        executor.spawn(Task::new(example_task(
            reactor,
            time.clone(),
            // waiting for 30 years in simulation
            Duration::from_secs(60 * 24 * 31 * 12 * 30),
        )));
        executor.run();

        assert!(
            time.now().gt(&Instant::now()),
            "simulated time {:?} is not greater than now {:?}",
            time.now(),
            &Instant::now()
        );
    }
}
