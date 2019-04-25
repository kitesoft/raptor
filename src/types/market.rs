use std::error::Error;
use std::hash::{Hash, Hasher};

use crate::types::atomic::{Order, Boards, Execution};

pub trait Market {
    fn get_market_name(&self) -> String;

    // Immutable
    fn boards(&self) -> Result<Boards, Box<Error>>;
    fn executions(&self) -> Result<Vec<Execution>, Box<Error>>;
    fn orders(&self) -> Result<Vec<Order>, Box<Error>>;

    // Mutable
    fn send_order(&self, order: Order) -> Result<Order, Box<Error>>;
    fn cancel_order(&self, order: Order) -> Result<Order, Box<Error>>;
}

impl Hash for Box<Market> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.get_market_name().hash(state)
    }
}

impl PartialEq for Box<Market> {
    fn eq(&self, other: &Box<Market>) -> bool {
        self.get_market_name() == other.get_market_name()
    }
}

impl Eq for Box<Market> {}

