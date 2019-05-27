use std::thread::sleep;
use std::time::{Duration};

use crate::types::algo::Algo;

pub struct Scheduler {
    algos: Vec<Box<Algo>>,
    tick: Duration,
}

impl Scheduler {
    pub fn new(tick: Duration) -> Self {
        Scheduler{
            algos: vec!(),
            tick: tick
        }
    }

    pub fn register(&mut self, mut algo: Box<Algo>) {
        algo.on_init();
        self.algos.push(algo);
    }

    pub fn run(&mut self) {
        // TODO deadline scheduling
        loop {
            for algo in &mut self.algos {
                match algo.on_update() {
                    Err(e) => algo.on_error(e),
                    _ => {}
                }
                sleep(self.tick);
            }
        }
    }
}
