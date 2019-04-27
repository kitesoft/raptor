use std::collections::HashMap;
use std::error::Error;
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

    pub fn run(&mut self) {
        loop {self.tick();}
    }

    fn tick(&mut self) {
        let _ = crossbeam::scope(|scope| {

            let mut handles = vec!();

            for id in &self.ids {
                let market = self.markets.get(id).unwrap();
                let algos = self.algos.get(id).unwrap();
                let tick = self.get_tick(id).unwrap();

                let handle = scope.spawn(move |_| {
                    let _ = Scheduler::spawn_algos_and_join(market, algos, tick);
                    market.unique_id()
                });
                handles.push(handle);
            }

            for handle in handles {
                let id = handle.join().unwrap();
                self.last_sched_at.insert(id, Instant::now());
            }

        });
    }

    fn get_tick(&self, market_id: &MarketID) -> Option<Duration> {
        let last_sched_at = self.last_sched_at.get(market_id)?;
        let ticks = self.ticks.get(market_id)?;
        let duration = Instant::now().duration_since(*last_sched_at);

        Some(*ticks - duration)
    }

    fn spawn_algos_and_join(market: &Box<Market + Sync + Send>, algos: &Vec<Box<Algo + Sync + Send>>, tick: Duration) -> Result<(), Box<Error>> {
        // sleep next tick
        sleep(tick);

        // get market state
        let state = State::new(market)?;

        // spawn algos
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
        Ok(())
    }
}

// TODO テストを書く
