use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::sync::Arc;
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
        // call on_init
        for (_, algos) in &self.algos {
            for algo in algos {algo.on_init()}
        }

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
                    Scheduler::spawn_algos_and_join(market, algos, tick).unwrap_or_else(|e| {
                        let error = Arc::new(e);
                        for algo in algos {algo.on_error(error.clone());} // notify error to algo
                    });
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
        let tick = self.ticks.get(market_id)?;
        let duration = Instant::now().duration_since(*last_sched_at);

        match tick.checked_sub(duration) {
            Some(t) => Some(t),
            _ => Some(Duration::new(0, 0)),
        }
    }

    // std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>`
    fn spawn_algos_and_join(market: &Box<Market + Sync + Send>, algos: &Vec<Box<Algo + Sync + Send>>, tick: Duration) -> Result<(), Box<Error>> {
        // sleep next tick
        sleep(tick);

        // spawn algos
        crossbeam::scope(|scope| -> Result<(), Box<Error>> {
            let mut handles = vec!();

            for algo in algos {
                let state = State::new(market)?;
                let action = Action::new(market);
                let handle = scope.spawn(move |_| {algo.on_update(&state, &action);}); // TODO set deadline
                handles.push(handle);
            }

            // wait for finish algos
            for handle in handles {let _ = handle.join();}

            Ok(())
        }).unwrap_or_else(|_| {Ok(())})
     }
}

// TODO テストを書く
