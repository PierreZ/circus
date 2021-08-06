//! Executor module

use crate::deterministic::runtime::reactor::Reactor;
use crate::deterministic::runtime::task::{Task, TaskId};
use crossbeam_queue::ArrayQueue;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Wake, Waker};

pub struct DeterministicExecutor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
    reactor: Reactor,
}

impl DeterministicExecutor {
    pub fn new() -> Self {
        DeterministicExecutor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
            reactor: Reactor::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.run_ready_tasks();

            if self.task_queue.is_empty() {
                // we have nothing to do here, we can advance simulation
                self.reactor.advance_simulation();
            }
            if self.waker_cache.is_empty() && self.task_queue.is_empty() && self.tasks.is_empty() {
                break;
            }
        }
    }
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
            reactor,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }
}

pub struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    pub fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}

#[cfg(test)]
mod tests {
    use crate::deterministic::runtime::executor::DeterministicExecutor;
    use crate::deterministic::runtime::task::Task;

    async fn async_number() -> u32 {
        42
    }

    async fn example_task() {
        let number = async_number().await;
        println!("async number: {}", number);
    }

    #[test]
    fn test_runtime() {
        let mut executor = DeterministicExecutor::new();
        executor.spawn(Task::new(example_task()));
        executor.run();
    }
}
