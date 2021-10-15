extern crate circus_simulation;

use circus_simulation::deterministic::platform::SimulationPlatform;
use circus_simulation::deterministic::runtime::executor::DeterministicExecutor;
use circus_simulation::deterministic::runtime::reactor::DeterministicReactor;
use circus_simulation::deterministic::runtime::task::Task;
use circus_simulation::platform::{Platform, PlatformProvider};
use std::time::Duration;
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    // let's create an Deterministic executor and runtime
    let reactor = DeterministicReactor::default();
    let mut executor = DeterministicExecutor::new_with_reactor(reactor.clone());

    // let's create a simulated platform. You can swap implementation between:
    // * production, allowing you to talk to your OS,
    // * dev, with an buggified deterministic simulation.
    let platform: PlatformProvider = SimulationPlatform::new(42, reactor).into();

    // let's run our async function
    executor.spawn(Task::new(run_platform(platform)));
    executor.run();
}

async fn run_platform(mut platform: PlatformProvider) {
    // opening a file with the simulated platform can trigger:
    // * `buggified` errors,
    // * random time to open the file.
    let start_time = platform.now();

    // We are going to loop opening a file.
    for i in 0..10 {
        let file_result = platform.open("/etc/hosts".as_ref()).await;
        // here, with the seed 42, the first `open` will take 817ms.
        // Don't worry, this is simulated time, so we are not waiting 817ms!
        if i == 0 {
            assert!(platform
                .now()
                .duration_since(start_time)
                .eq(&Duration::from_millis(817)));
        }
        if i == 4 {
            // using the seed 42, the fourth opening will trigger an error.
            assert!(file_result.is_err());
        } else {
            assert!(file_result.is_ok());
        }
    }
}
