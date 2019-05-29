use std::error::Error;
use reqwest::Proxy;

use crate::types::atomic::{Order, Boards, Execution, Asset};

pub trait Market {
    // Immutable
    fn boards(&self, proxy: Option<Proxy>) -> Result<Boards, Box<Error>>;
    fn executions(&self, proxy: Option<Proxy>) -> Result<Vec<Execution>, Box<Error>>;
    fn orders(&self, proxy: Option<Proxy>) -> Result<Vec<Order>, Box<Error>>;
    fn assets(&self, proxy: Option<Proxy>) -> Result<Vec<Asset>, Box<Error>>;

    // Mutable
    fn send_order(&self, proxy: Option<Proxy>, order: Order) -> Result<Order, Box<Error>>;
    fn cancel_order(&self, proxy: Option<Proxy>, order: Order) -> Result<Order, Box<Error>>;
}
