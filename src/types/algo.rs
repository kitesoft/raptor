use std::error::Error;
use std::sync::Arc;

use crate::types::atomic::{Order, Boards, Execution};
use crate::types::market::Market;

#[derive(Debug, Clone)]
pub struct State {
    pub boards: Boards,
    pub executions: Vec<Execution>,
    pub orders: Vec<Order>,
}

impl State {
    pub fn new<'a>(market: &Box<Market + Sync + Send + 'a >) -> Result<Self, Box<Error>> {
        // TODO 高速化のために並列でリクエストを飛ばす
        let boards = market.boards()?;
        let executions = market.executions()?;
        let orders = market.orders()?;

        Ok(
            State{
                boards: boards,
                executions: executions,
                orders: orders,
            }
        )
    }
}

pub struct Action<'a> {
    pub send_order: Box<dyn Fn(Order) -> Result<Order, Box<Error>> + Sync + Send + 'a>,
    pub cancel_order: Box<dyn Fn(Order) -> Result<Order, Box<Error>> + Sync + Send + 'a>,
}

impl<'a> Action<'a> {
    pub fn new(market: &'a Box<Market + Sync + Send + 'a >) -> Self {
        Action{
            send_order: Box::new(move |order: Order| {market.send_order(order)}),
            cancel_order: Box::new(move |order: Order| {market.cancel_order(order)}),
        }
    }
}

pub trait Algo
{
    fn on_init(&self);
    fn on_update(&self, state: &State, action: &Action);
    fn on_error(&self, e: Arc<Box<Error>>);
}
