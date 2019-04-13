use std::sync::{Arc, Mutex};

use crate::types::market::Market;

pub trait Algo<T>
where T: Market
{
    fn new(market: Arc<Mutex<T>>) -> Self;
    fn run(&self);
}
