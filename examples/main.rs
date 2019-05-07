#[macro_use] extern crate serde_derive;
extern crate raptor;
extern crate reqwest;
extern crate serde_yaml;

mod algos;
mod config;

use std::time::Duration;
use std::sync::{Arc, RwLock};
use crate::raptor::types::algo::Algo;
use crate::raptor::scheduler::sched::Scheduler;
use crate::raptor::scheduler::task::Task;

use crate::config::Config;
use crate::algos::example_algo::ExampleAlgo;

fn main() {
    let config = Config::load_config("./examples/config.yml".to_string()).unwrap();
    let markets = config.clone().raptor;
    let mut algos: Vec<Box<Algo + Send + Sync>> = vec!();

    // algorithms
    let mm = Box::new(ExampleAlgo{});

    // markets
    let bitflyer = Box::new(markets.bitflyer);

    algos.push(mm);

    let mut sched = Scheduler::new();
    let task = Arc::new(RwLock::new(Task::new(bitflyer, algos, Duration::from_secs(1))));
    sched.register(task);
    sched.run();
}
