use std::sync::{Arc, Mutex};
use std::error::Error;
use reqwest::Client;
use serde_json;

use crate::market::bitflyer::boards::BitFlyerBoards;
use crate::market::bitflyer::execution::BitFlyerExecution;
use crate::market::bitflyer::order::{BitFlyerOrder};
use crate::market::bitflyer::params::{SendOrderParam, SendOrderResponse, CancelOrderParam};
use crate::market::bitflyer::utils::BitFlyerUtils;
use crate::types::atomic::{Boards, Execution, Order, Side, OrderType, ProductCode};
use crate::types::market::Market;
use crate::types::algo::Algo;
use crate::types::config::Config;

pub struct BitFlyer {
    endpoint: String,
    algos: Vec<Box<Algo>>,
    api_key: String,
    api_secret: String,
}

impl Market for BitFlyer {
    fn new(config: Config) -> Arc<Mutex<BitFlyer>> {
        Arc::new(Mutex::new(BitFlyer{
            algos: vec!(),
            endpoint: config.bitflyer.endpoint,
            api_key: config.bitflyer.api_key,
            api_secret: config.bitflyer.api_secret,
        }))
    }

    fn boards(&mut self, product_code: ProductCode) -> Result<Boards, Box<Error>> {
        let client = Client::new();
        let url: &str = &format!("{}{}", self.endpoint, "/v1/board");
        let params = [("product_code", self.to_product_code(product_code)?)];

        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<BitFlyerBoards, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(self.to_boards(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn executions(&mut self, product_code: ProductCode) -> Result<Vec<Execution>, Box<Error>> {
        let client = Client::new();
        let url: &str = &format!("{}{}", self.endpoint, "/v1/executions");
        let params = [("product_code", self.to_product_code(product_code)?)];

        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<Vec<BitFlyerExecution>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(self.to_executions(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn orders(&mut self, product_code: ProductCode) -> Result<Vec<Order>, Box<Error>> {
        let client = Client::new();
        let method = "GET";
        let path = &format!("/v1/me/getchildorders?product_code={}", self.to_product_code(product_code)?);
        let headers = BitFlyerUtils::get_header(&self.api_key, &self.api_secret, method, path, "");
        let url: &str = &format!("{}{}", self.endpoint, path);

        let text = client.get(url).headers(headers).send()?.text()?;
        let json: Result<Vec<BitFlyerOrder>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(self.to_orders(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn send_order(&self, mut order: Order) -> Result<Order, Box<Error>> {
        let client = Client::new();
        let method = "POST";
        let path = "/v1/me/sendchildorder";
        let params = SendOrderParam{
            product_code: self.to_product_code(order.product_code)?,
            child_order_type: self.to_order_type(order.order_type)?,
            side: self.to_side(order.side)?,
            price: order.price,
            size: order.size,
        };
        let body = serde_json::to_string(&params).unwrap();
        let headers = BitFlyerUtils::get_header(&self.api_key, &self.api_secret, method, path, &body);
        let url: &str = &format!("{}{}", self.endpoint, path);

        let text = client.post(url).headers(headers).body(body).send()?.text()?;
        let json: Result<SendOrderResponse, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => {
                order.id = res.child_order_acceptance_id;
                return Ok(order);
            },
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn cancel_order(&self, order: Order) -> Result<Order, Box<Error>> {
        let client = Client::new();
        let method = "POST";
        let path = "/v1/me/cancelchildorder";
        let params = CancelOrderParam{
            product_code: self.to_product_code(order.product_code)?,
            child_order_acceptance_id: order.id.clone(),
        };
        let body = serde_json::to_string(&params).unwrap();
        let headers = BitFlyerUtils::get_header(&self.api_key, &self.api_secret, method, path, &body);
        let url: &str = &format!("{}{}", self.endpoint, path);

        let text = client.post(url).headers(headers).body(body).send()?.text()?;
        let json: Result<SendOrderResponse, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(_) => return Ok(order),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn get_algos(&mut self) -> &mut Vec<Box<Algo>> {
        &mut self.algos
    }

    fn to_side(&self, side: Side) -> Result<String, Box<Error>> {
        match side {
            Side::Buy => return Ok("BUY".to_string()),
            Side::Sell => return Ok("SELL".to_string()),
        }
    }

    fn to_order_type(&self, order_type: OrderType) -> Result<String, Box<Error>>{
        match order_type {
            OrderType::Market => return Ok("MARKET".to_string()),
            OrderType::Limit => return Ok("LIMIT".to_string()),
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported"))),
        }
    }

    fn to_product_code(&self, product_code: ProductCode) -> Result<String, Box<Error>>{
        match product_code {
            ProductCode::BtcJpy => return Ok("BTC_JPY".to_string()),
            ProductCode::FxBtcJpy => return Ok("FX_BTC_JPY".to_string()),
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported"))),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_bitflyer() {
        // TODO reqwestをモックしてテストを書く
        // let market = BitFlyer::new();
        // let mut market = market.lock().unwrap();
        //
        // // Executions test
        // match market.executions(ProductCode::FxBtcJpy) {
        //     Ok(executions) => println!("{:?}", executions[0]),
        //     Err(e) => println!("{}", e),
        // }
        //
        // // Boards test
        // match market.boards(ProductCode::FxBtcJpy) {
        //     Ok(boards) => println!("{:?}", boards.bid[0]),
        //     Err(e) => println!("{}", e),
        // }
        //
        // // Orders test
        // match market.orders(ProductCode::FxBtcJpy) {
        //     Ok(orders) => println!("{:?}", orders[0]),
        //     Err(e) => println!("{}", e),
        // }
        //
        // // send_orders test
        // let order = Order{
        //     id: "Dummy".to_string(),
        //     order_type: OrderType::Market,
        //     product_code: ProductCode::BtcJpy,
        //     side: Side::Buy,
        //     price: 0.0,
        //     size: 0.0001,
        // };
        // match market.send_order(order) {
        //     Ok(_) => println!("success send order"),
        //     Err(e) => println!("error message(send_order): {}", e),
        // }
        //
        // // cancel order
        // let order = Order{
        //     id: "Dummy".to_string(),
        //     order_type: OrderType::Market,
        //     product_code: ProductCode::BtcJpy,
        //     side: Side::Buy,
        //     price: 0.0,
        //     size: 0.0001,
        // };
        // match market.send_order(order) {
        //     Ok(_) => println!("success send order"),
        //     Err(e) => println!("error message(send_order): {}", e),
        // }
    }
}
