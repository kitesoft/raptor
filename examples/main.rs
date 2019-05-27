#[macro_use] extern crate serde_derive;
extern crate raptor;
extern crate reqwest;
extern crate serde_yaml;

mod algos;
mod config;

use std::time::Duration;
use crate::raptor::scheduler::sched::Scheduler;

use crate::config::Config;
use crate::algos::example_algo::ExampleAlgo;

fn main() {
    // markets
    let config = Config::load_config("./examples/config.yml".to_string()).unwrap();
    let _markets = config.clone().raptor;

    // algorithms
    let mm = Box::new(ExampleAlgo{});

    // scheduler
    let tick = Duration::from_secs(1);
    let mut sched = Scheduler::new(tick);
    sched.register(mm);
    sched.run();
}
