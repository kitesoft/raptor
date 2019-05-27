use std::error::Error;
use crate::scheduler::task::Task;

pub struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler{
            tasks: vec!(),
        }
    }

    pub fn register(&mut self, task: Task) -> Result<(), Box<Error>> {
        task.sched_register();
        self.tasks.push(task);
        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            for task in &mut self.tasks {
                task.sched_in();
                task.sched();
                task.sched_out();
            }
        }
    }
}
