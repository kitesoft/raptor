use std::sync::{Arc, Mutex};

use crate::market::bitflyer::boards::BitFlyerBoards;
use crate::market::bitflyer::execution::BitFlyerExecution;
use crate::market::bitflyer::utils::BitFlyerUtils;
use crate::types::atomic::{Boards, Execution, Order, ProductCode, Side, OrderType};
use crate::types::market::Market;
use crate::types::algo::Algo;

pub struct BitFlyer {
    client: reqwest::Client,
    endpoint: String,
    limit: i64,
    algos: Vec<Box<Algo>>,
    product_code: ProductCode,
    api_key: String,
    api_secret: String,
}

impl Market for BitFlyer {
    // TODO 色んな所からupdate, executionsとかを参照されても,過去に取得していればそれを返すようにする
    // TODO product_codeの変換
    // TODO APIの通信にproduct_codeを含める

    fn new() -> Arc<Mutex<BitFlyer>> {
        Arc::new(Mutex::new(BitFlyer{
            // TODO 定数はファイルとかから取ってくるようにする (lazy_static?)
            client: reqwest::Client::new(),
            algos: vec!(),
            product_code: ProductCode::BtcJpy,

            endpoint: String::from("https://api.bitflyer.com/v1/"),
            limit: 500,
            api_key: String::from(""),
            api_secret: String::from(""),
        }))
    }

    fn boards(&mut self) -> Result<Boards, reqwest::Error> {
        let url: &str = &format!("{}{}", self.endpoint, "/board");
        let res: BitFlyerBoards = self.client.get(url).send()?.json()?;
        Ok(self.to_boards(res))
    }

    fn executions(&mut self) -> Result<Vec<Execution>, reqwest::Error> {
        let url: &str = &format!("{}{}", self.endpoint, "/executions");
        let res: Vec<BitFlyerExecution> = self.client.get(url).send()?.json()?;
        Ok(self.to_executions(res))
    }

    fn orders(&mut self) -> Result<Vec<Order>, reqwest::Error> {
        // TODO 実装する
        Ok(vec!(Order{
            id: String::from("id"),
            order_type: OrderType::Market,
            side: Side::Buy,
            price: 1.0,
            size: 1.0
        }))
    }

    fn send_order(&self, order: Order) -> Result<Order, reqwest::Error> {
        // let _ = BitFlyerUtils::get_header();
        // TODO 実装する
        Ok(order)
    }

    fn cancel_order(&self, order: Order) -> Result<Order, reqwest::Error> {
        // TODO 実装する
        Ok(order)
    }

    fn get_algos(&mut self) -> &mut Vec<Box<Algo>> {
        &mut self.algos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitflyer() {
        // TODO reqwestをモックしてテストを書く
        let _ = BitFlyer::new();
    }
}
