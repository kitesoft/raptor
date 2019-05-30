use std::thread::sleep;
use std::sync::{Arc, RwLock};
use std::time::{Duration};

use crate::types::algo::Algo;

pub struct Scheduler {
    algos: Vec<Arc<RwLock<Box<Algo + Send + Sync>>>>,
    ticks: Vec<Duration>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler{
            algos: vec!(),
            ticks: vec!() 
        }
    }

    pub fn register(&mut self, mut algo: Box<Algo + Send + Sync>, tick: Duration) {
        algo.on_init();
        self.ticks.push(tick);
        self.algos.push(Arc::new(RwLock::new(algo)));
    }

    pub fn run(&mut self) {
        crossbeam::scope(|scope| {
            for idx in 0..self.algos.len() {
                let algos = self.algos.clone();
                let ticks = self.ticks.clone();
                scope.spawn(move |_| {
                    let algo = algos.get(idx).unwrap();
                    let tick = ticks.get(idx).unwrap();
                    match algo.write() {
                        Ok(mut algo) => {
                            loop {
                                match algo.on_update() {
                                    Err(e) => algo.on_error(e),
                                    _ => {},
                                }
                                sleep(*tick);
                            }
                        }
                        _ => {},
                    }
                });
            }
        });
    }
}
