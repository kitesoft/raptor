use std::error::Error;
use reqwest::Proxy;

use crate::types::atomic::{Order, Boards, Execution};

pub trait Market {
    // Immutable
    fn boards(&self, proxy: Option<Proxy>) -> Result<Boards, Box<Error>>;
    fn executions(&self, proxy: Option<Proxy>) -> Result<Vec<Execution>, Box<Error>>;
    fn orders(&self, proxy: Option<Proxy>) -> Result<Vec<Order>, Box<Error>>;
}
