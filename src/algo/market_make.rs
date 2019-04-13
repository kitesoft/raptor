use std::sync::{Arc, Mutex};

use crate::types::algo::Algo;
use crate::types::market::Market;

pub struct MarketMake<T> 
where T: Market
{
    market: Arc<Mutex<T>>,
}

impl<T> Algo<T> for MarketMake<T>
where T: Market
{
    fn new(market: Arc<Mutex<T>>) -> MarketMake<T> {
        MarketMake{
            market: market,
        }
    }

    fn run(&self) {
        // TODO ロジックを実装する
    }
}
