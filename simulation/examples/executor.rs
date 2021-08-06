use circus_simulation::deterministic::runtime::executor::new_executor_and_spawner;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
/// higher priority will be executed before
enum PriorityTask {
    Low,
    Default,
    High,
}

impl From<PriorityTask> for usize {
    fn from(p: PriorityTask) -> Self {
        match p {
            PriorityTask::Low => 2,
            PriorityTask::Default => 1,
            PriorityTask::High => 0,
        }
    }
}

fn main() {
    let (mut executor, spawner) = new_executor_and_spawner();

    spawner.spawn(PriorityTask::Default.into(), async {
        println!("Default priority!");
    });

    spawner.spawn(PriorityTask::High.into(), async {
        println!("higher priority!")
    });

    spawner.spawn(PriorityTask::Low.into(), async {
        println!("Lower priority!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    executor.run();
}
