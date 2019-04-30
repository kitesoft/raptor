use std::error::Error;
use std::thread::sleep;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::types::market::Market;
use crate::types::algo::{Algo, Action, State};

type MarketID = String;

pub struct Task {
    pub id: MarketID,
    pub market: Box<Market + Sync + Send>,
    pub algos: Vec<Box<Algo + Sync + Send>>,
    tick: Duration,
    last_sched_at: Instant,
}

impl Task {
    pub fn new(market: Box<Market + Sync + Send>, algos: Vec<Box<Algo + Sync + Send>>, tick: Duration) -> Self {
        Task {
            id: market.unique_id(),
            tick: tick,
            algos: algos,
            market: market,
            last_sched_at: Instant::now() - Duration::from_secs(1000),
        }
    }

    pub fn sched_register(&self) {
        // TODO マルチスレッド化する
        for algo in &self.algos {
            algo.on_init();
        }
    }

    pub fn sched_in(&self) {
        let duration = Instant::now().duration_since(self.last_sched_at);
        let tick = match self.tick.checked_sub(duration) {
            Some(t) => t,
            _ => Duration::new(0, 0),
        };
        sleep(tick);
    }

    pub fn sched_out(&mut self) {
        self.last_sched_at = Instant::now();
    }

    pub fn sched_error(&self, error: Box<Error>) {
        // TODO マルチスレッド化する
        let error = Arc::new(error);
        for algo in &self.algos {
            algo.on_error(error.clone());
        }
    }

    pub fn sched_drop(&self) {
        // TODO マルチスレッド化する
        for algo in &self.algos {
            algo.on_destroy();
        }
    }

    pub fn sched(&self) -> Result<(), Box<Error>> {
        // TODO マルチスレッド化する
        for algo in &self.algos {
            let state = State::new(&self.market)?;
            let action = Action::new(&self.market);
            algo.on_update(&state, &action);
        }

        Ok(())
    }
}
