use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use crate::types::atomic::Order;
use crate::types::market::Market;
use crate::types::algo::{Algo, Action, State};

pub struct Scheduler {
    tasks: HashMap<Box<Market>, Vec<Box<Algo>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        return Scheduler{
            tasks: HashMap::new(),
        }
    }

    pub fn register(&mut self, market: Box<Market>, algos: Vec<Box<Algo>>) {
        self.tasks.insert(market, algos);
    }

    pub fn run(&mut self) {
        for (market, algos) in &mut self.tasks {
            // TODO 前回飛ばしたリクエスト数とかリミットからいい感じにsleepさせる
            let secs = Duration::from_secs(10);
            sleep(secs);

            // TODO 高速化のために並列でリクエストを飛ばす
            let boards = market.boards().unwrap();
            let executions = market.executions().unwrap();
            let orders = market.orders().unwrap();

            let state = State{
                boards: boards,
                executions: executions,
                orders: orders,
            };

            // TODO API通信のための回数制限を設ける
            let action = Action{
                send_order: Box::new(|order: Order| {market.send_order(order)}),
                cancel_order: Box::new(|order: Order| {market.cancel_order(order)}),
            };

            for algo in algos {
                // TODO  N秒で終わることを義務付ける
                algo.ticker(&state, &action);
            }
        }
    }
}
