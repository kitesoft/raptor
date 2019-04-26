use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::types::market::Market;
use crate::types::algo::{Algo, Action, State};

type MarketID = String;

pub struct Scheduler {
    ids: Vec<String>,
    markets: HashMap<MarketID, Box<Market + Sync + Send>>,
    algos: HashMap<MarketID, Vec<Box<Algo + Sync + Send>>>,
    ticks: HashMap<MarketID, Duration>,
    last_sched_at: HashMap<MarketID, Instant>,
}

impl Scheduler {
    pub fn new() -> Self {
        return Scheduler{
            ids: vec!(),
            markets: HashMap::new(),
            algos: HashMap::new(),
            ticks: HashMap::new(),
            last_sched_at: HashMap::new(),
        }
    }

    pub fn register(&mut self, market: Box<Market + Sync + Send>, algos: Vec<Box<Algo + Sync + Send>>, tick: Duration) {
        self.last_sched_at.insert(market.unique_id(), Instant::now() - Duration::from_secs(1000));
        self.ids.push(market.unique_id());
        self.ticks.insert(market.unique_id(), tick);
        self.algos.insert(market.unique_id(), algos);
        self.markets.insert(market.unique_id(), market);
    }

    pub fn tick(&mut self) {
        let _ = crossbeam::scope(|scope| {

            let mut handles = vec!();

            for id in &self.ids {
                let market = self.markets.get(id).unwrap(); // TODO remove unwrap
                let algos = self.algos.get(id).unwrap(); // TODO remove unwrap
                let tick = self.get_tick(id);

                let handle = scope.spawn(move |_| {
                    Scheduler::spawn_algos_and_join(market, algos, tick);
                    market.unique_id()
                });
                handles.push(handle);

            }

            for handle in handles {
                let id = handle.join().unwrap(); // TODO remove unwrap
                self.last_sched_at.insert(id, Instant::now());
            }
        });
    }

    fn get_tick(&self, market_id: &MarketID) -> Duration {
        let market = self.markets.get(market_id).unwrap(); // TODO remove unwrap
        let last_sched_at = self.last_sched_at.get(&market.unique_id()).unwrap(); // remove unwrap
        let ticks = self.ticks.get(&market.unique_id()).unwrap(); // TODO remove unwrap
        let duration = Instant::now().duration_since(*last_sched_at);
        *ticks - duration
    }

    fn spawn_algos_and_join(market: &Box<Market + Sync + Send>, algos: &Vec<Box<Algo + Sync + Send>>, tick: Duration) {
        // sleep next tick
        sleep(tick);

        // spawn algos
        let state = State::new(market);
        let _ = crossbeam::scope(|scope| {
            let mut handles = vec!();

            for algo in algos {
                let s = state.clone();
                let action = Action::new(market);
                let handle = scope.spawn(move |_| {algo.ticker(&s, &action);}); // TODO set deadline
                handles.push(handle);
            }

            // wait for finish algos
            for handle in handles {let _ = handle.join();}
        });
    }
}
