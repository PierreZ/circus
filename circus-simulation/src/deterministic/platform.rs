//! Deterministic platform module
use crate::deterministic::fs::file::SimulatedFile;
use crate::deterministic::random::DeterministicRandom;
use crate::deterministic::runtime::reactor::DeterministicReactor;
use crate::deterministic::runtime::timer::DeterministicTimer;
use crate::deterministic::time::DeterministicTime;
use crate::file::File;
use crate::platform::Platform;
use async_trait::async_trait;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;

use circus_buggify::Buggifier;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::time::{Duration, Instant};

/// Simulated version of the plateform. Every API exposed is subject to an deterministic output,
/// including:
/// * time,
/// * random,
/// * buggified I/O calls.
#[derive(Clone)]
pub struct SimulationPlatform {
    time: DeterministicTime,
    random: DeterministicRandom,
    reactor: DeterministicReactor,
    buggifier: Arc<Buggifier>,
}

impl SimulationPlatform {
    /// This will:
    /// * enable buggify
    /// * start the simulation reactor
    pub fn new(seed: u64, reactor: DeterministicReactor) -> Self {
        let random = DeterministicRandom::new_with_seed(seed);

        SimulationPlatform {
            time: reactor.get_deterministic_time(),
            random,
            reactor,
            buggifier: Arc::new(Buggifier::new(SmallRng::seed_from_u64(seed))),
        }
    }
}

#[async_trait]
impl Platform for SimulationPlatform {
    // https://forums.foundationdb.org/t/simulation-of-disk-i-o/2937
    async fn open(&mut self, path: &Path) -> io::Result<File> {
        if self.buggifier.buggify() {
            let probability = self.random.random_01();
            // we cannot use float range in match
            // issue #41620 <https://github.com/rust-lang/rust/issues/41620>
            let error = if probability < 0.1 {
                Error::from(ErrorKind::UnexpectedEof)
            } else if probability < 0.2 {
                Error::from(ErrorKind::PermissionDenied)
            } else {
                // The system cannot find the file specified. (os error 2)
                Error::from_raw_os_error(2)
            };
            tracing::info!("buggified open file {:?}: {:?}", path, error);
            return io::Result::Err(error);
        }
        let result = std::fs::File::open(path);

        let wait_duration = Duration::from_millis(self.random.random_between(300u64..2000u64));
        DeterministicTimer::wait_with_reactor(
            self.time.clone(),
            self.reactor.clone(),
            wait_duration,
        )
        .await;

        match result {
            Ok(file) => Ok(SimulatedFile::new(file).into()),
            Err(error) => Err(error),
        }
    }

    fn now(&self) -> Instant {
        self.time.now()
    }
}

#[cfg(test)]
mod tests {
    use crate::deterministic::platform::SimulationPlatform;
    use crate::deterministic::runtime::executor::DeterministicExecutor;
    use crate::deterministic::runtime::reactor::DeterministicReactor;
    use crate::deterministic::runtime::task::Task;
    use crate::platform::Platform;
    use std::path::Path;
    use std::time::Duration;
    use tracing::Level;

    async fn example_task_open_file(reactor: DeterministicReactor) {
        let mut platform = SimulationPlatform::new(42, reactor);
        let start = platform.now();
        let file_result = platform.open(Path::new("/etc/hosts")).await;
        let end = platform.now();

        assert!(file_result.is_ok(), "could not open /etc/hosts");
        assert!(
            start.lt(&end),
            "simulated time did not moved: start={:?}, end={:?}",
            start,
            end
        );
        // using seed 42, time should have been moved by 817ms exactly
        assert_eq!(
            end.duration_since(start),
            Duration::from_millis(817),
            "None deterministic time found!"
        );
    }

    #[test]
    fn test_open() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_test_writer()
            .try_init();

        let reactor = DeterministicReactor::default();

        let mut executor = DeterministicExecutor::new_with_reactor(reactor.clone());
        executor.spawn(Task::new(example_task_open_file(reactor)));
        executor.run();
    }

    #[test]
    fn test_buggified_open() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_test_writer()
            .try_init();

        let reactor = DeterministicReactor::default();

        let mut executor = DeterministicExecutor::new_with_reactor(reactor.clone());
        executor.spawn(Task::new(async move {
            let mut platform = SimulationPlatform::new(42, reactor);
            for i in 0..10 {
                let file_result = platform.open(Path::new("/etc/hosts")).await;
                if i == 8 {
                    assert!(file_result.is_err());
                } else {
                    assert!(file_result.is_ok());
                }
            }
        }));
        executor.run();
    }
}
