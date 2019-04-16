use std::sync::{Arc, Mutex};

use crate::market::bitflyer::boards::BitflyerBoards;
use crate::market::bitflyer::execution::BitflyerExecution;
use crate::types::atomic::{Boards, Execution, Order, ProductCode, Side, OrderType};
use crate::types::market::Market;
use crate::types::algo::Algo;

pub struct BitFlyer {
    client: reqwest::Client,
    endpoint: String,
    limit: i64,
    algos: Vec<Box<Algo>>,
    product_code: ProductCode,
}

impl Market for BitFlyer {
    // TODO 色んな所からupdate, executionsとかを参照されても,過去に取得していればそれを返すようにする
    // TODO product_codeの変換
    // TODO APIの通信にproduct_codeを含める

    fn new() -> Arc<Mutex<BitFlyer>> {
        Arc::new(Mutex::new(BitFlyer{
            // TODO 定数はファイルとかから取ってくるようにする (lazy_static?)
            client: reqwest::Client::new(),
            endpoint: String::from("https://api.bitflyer.com/v1/"),
            limit: 500,
            product_code: ProductCode::BtcJpy,
            algos: vec!(),
        }))
    }

    fn boards(&mut self) -> Result<Boards, reqwest::Error> {
        let url: &str = &format!("{}{}", self.endpoint, "/board");
        let res: BitflyerBoards = self.client.get(url).send()?.json()?;
        Ok(self.to_boards(res))
    }

    fn executions(&mut self) -> Result<Vec<Execution>, reqwest::Error> {
        let url: &str = &format!("{}{}", self.endpoint, "/executions");
        let res: Vec<BitflyerExecution> = self.client.get(url).send()?.json()?;
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
