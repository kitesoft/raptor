extern crate lazy_static;
extern crate chrono;
extern crate reqwest;
#[macro_use] extern crate serde_derive;

mod market;
mod algo;
mod types;
mod utils;

use crate::algo::market_make::MarketMake;
use crate::market::bitflyer::bitflyer::BitFlyer;
use crate::types::market::Market;

fn main() {
    // TODO ライブラリ化する
    // TODO README.mdを用意する
    // TODO テストを書く
    // TODO フレームワーク化する
    // TODO Exampleを用意する
    let market = BitFlyer::new();

    let algo = MarketMake{};
    let algo2 = MarketMake{};

    let mut market = market.lock().unwrap();
    market.register(Box::new(algo));
    market.register(Box::new(algo2));

    // TODO marketのスタート
}
