use std::error::Error;

use crate::types::atomic::{Order, Boards, Execution};

pub struct State {
    pub boards: Boards,
    pub executions: Vec<Execution>,
    pub orders: Vec<Order>,
}

pub struct Action<'a> {
    pub send_order: Box<dyn Fn(Order) -> Result<Order, Box<Error>> + 'a>,
    pub cancel_order: Box<dyn Fn(Order) -> Result<Order, Box<Error>> + 'a>,
}

pub trait Algo
{
    fn ticker(&self, state: &State, action: &Action);
}
