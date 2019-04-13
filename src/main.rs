extern crate lazy_static;
extern crate chrono;
extern crate reqwest;
#[macro_use] extern crate serde_derive;

mod market;
mod algo;
mod types;
mod utils;

use std::thread;

use crate::algo::market_make::MarketMake;
use crate::market::bitflyer::bitflyer::BitFlyer;
use crate::types::algo::Algo;
use crate::types::market::Market;

fn spawn_algo<T, U>(algo: U) -> thread::JoinHandle<()>
    where T: Market + Send + 'static,
          U: Algo<T> + Send + 'static
{
    thread::spawn(move || {
        algo.run();
    })
}

fn main() {
    let market = BitFlyer::new();

    let algo = MarketMake::new(market.clone());
    let algo2 = MarketMake::new(market.clone());

    let handle = spawn_algo(algo);
    let _ = spawn_algo(algo2);

    handle.join().expect("Couldn't join");

    // market.lock().unwrap().board().unwrap();
    market.lock().unwrap().executions().unwrap();
}
