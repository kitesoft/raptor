use std::error::Error;

use crate::types::atomic::{Order, Boards, Execution, Asset};

pub trait Market {
    fn unique_id(&self) -> String;

    // Immutable
    fn boards(&self) -> Result<Boards, Box<Error>>;
    fn executions(&self) -> Result<Vec<Execution>, Box<Error>>;
    fn orders(&self) -> Result<Vec<Order>, Box<Error>>;
    fn assets(&self) -> Result<Vec<Asset>, Box<Error>>;

    // Mutable
    fn send_order(&self, order: Order) -> Result<Order, Box<Error>>;
    fn cancel_order(&self, order: Order) -> Result<Order, Box<Error>>;
}
