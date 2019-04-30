use std::error::Error;

use crate::types::atomic::{Order, Boards, Execution, Asset};
use crate::types::market::Market;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub boards: Boards,
    pub executions: Vec<Execution>,
    pub orders: Vec<Order>,
    pub assets: Vec<Asset>,
}

impl State {
    pub fn new<'a>(market: &Box<Market + Sync + Send + 'a >) -> Result<Self, Box<Error>> {
        // TODO 高速化のために並列でリクエストを飛ばす
        let boards = market.boards()?;
        let executions = market.executions()?;
        let orders = market.orders()?;
        let assets = market.assets()?;

        Ok(
            State{
                boards: boards,
                executions: executions,
                orders: orders,
                assets: assets,
            }
        )
    }
}

pub struct Action<'a> {
    pub send_order: Box<dyn Fn(Order) -> Result<Order, Box<Error>> + Sync + Send + 'a>,
    pub cancel_order: Box<dyn Fn(Order) -> Result<Order, Box<Error>> + Sync + Send + 'a>,
}

impl<'a> Action<'a> {
    pub fn new(market: &'a Box<Market + Sync + Send + 'a>) -> Self {
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
    fn on_error(&self, e: Box<Error>);
    fn on_destroy(&self);
}
