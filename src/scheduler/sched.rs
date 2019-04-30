use std::sync::{Arc, RwLock};
use std::error::Error;
use std::thread;
use std::process::exit;
use crate::scheduler::task::Task;
use signal_hook::{iterator::Signals, SIGINT};

pub struct Scheduler {
    tasks: Vec<Arc<RwLock<Task>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler{
            tasks: vec!(),
        }
    }

    pub fn register(&mut self, task: Arc<RwLock<Task>>) -> Result<(), Box<Error>> {
        match &task.read() {
            Ok(task) => {
                task.sched_register();
            }
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Register failed"))),
        }

        self.tasks.push(task);
        Ok(())
    }
    pub fn run(&self) {
        self.set_drop_handlers();

        loop {
            for task in &self.tasks {
                let read_task = task.read().unwrap();

                read_task.sched_in();
                match read_task.sched() {
                    Err(e) => read_task.sched_error(e),
                    _ => {},
                }

                let mut write_task = task.write().unwrap();
                write_task.sched_out();
            }
        }
    }

    fn set_drop_handlers(&self) {
        let signals = Signals::new(&[SIGINT]).unwrap();
        let tasks = self.tasks.clone();

        thread::spawn(move || {
            for _ in signals.forever() {
                println!("   waiting for finish... (call on_destroy handlers.)");
                for task in tasks {
                    task.read().unwrap().sched_drop();
                }
                exit(0);
            }
        });
    }
}
