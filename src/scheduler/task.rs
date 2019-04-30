use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_utils::thread;

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
        let _ = thread::scope(|s| {
            for algo in &self.algos {
                s.spawn(move |_| {algo.on_init();});
            }
        });
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

    pub fn sched_drop(&self) {
        let _ = thread::scope(|s| {
            for algo in &self.algos {
                s.spawn(move |_| {algo.on_destroy();});
            }
        });
    }

    pub fn sched(&self) {
        let _ = thread::scope(|s| {
            for algo in &self.algos {
                s.spawn(move |_| {
                    match State::new(&self.market) {
                        Ok(state) => {
                            let action = Action::new(&self.market);
                            algo.on_update(&state, &action);
                        },
                        Err(e) => {
                            algo.on_error(e);
                        }
                    }
                });
            }
        });
    }
}
