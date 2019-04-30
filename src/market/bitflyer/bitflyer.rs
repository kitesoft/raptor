use std::error::Error;

use reqwest::Client;
use serde_json;

use crate::market::bitflyer::boards::BitFlyerBoards;
use crate::market::bitflyer::execution::BitFlyerExecution;
use crate::market::bitflyer::order::BitFlyerOrder;
use crate::market::bitflyer::asset::BitFlyerAsset;
use crate::market::bitflyer::params::{SendOrderParam, SendOrderResponse, CancelOrderParam};
use crate::market::bitflyer::utils::BitFlyerUtils;
use crate::types::atomic::{Boards, Execution, Order, Asset};
use crate::types::market::Market;
use crate::utils::market::MarketUtils;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BitFlyer {
    pub endpoint: String,
    pub api_key: String,
    pub api_secret: String,
    pub product_code: String,
}

impl Market for BitFlyer {
    fn unique_id(&self) -> String {
        format!("BitFlyer{}{}{}{}", self.endpoint, self.api_key, self.api_secret, self.product_code)
    }

    fn boards(&self) -> Result<Boards, Box<Error>> {
        let client = Client::new();
        let url: &str = &format!("{}{}", self.endpoint, "/v1/board");
        let params = [("product_code", self.product_code.clone())];

        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<BitFlyerBoards, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_boards(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn executions(&self) -> Result<Vec<Execution>, Box<Error>> {
        let client = Client::new();
        let url: &str = &format!("{}{}", self.endpoint, "/v1/executions");
        let params = [("product_code", self.product_code.clone())];

        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<Vec<BitFlyerExecution>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_executions(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn orders(&self) -> Result<Vec<Order>, Box<Error>> {
        let client = Client::new();
        let method = "GET";
        let path = &format!("/v1/me/getchildorders?product_code={}", self.product_code);
        let headers = BitFlyerUtils::get_header(&self.api_key, &self.api_secret, method, path, "");
        let url: &str = &format!("{}{}", self.endpoint, path);

        let text = client.get(url).headers(headers).send()?.text()?;
        let json: Result<Vec<BitFlyerOrder>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_orders(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn assets(&self) -> Result<Vec<Asset>, Box<Error>> {
        let client = Client::new();
        let method = "GET";
        let path = &format!("/v1/me/getbalance");
        let headers = BitFlyerUtils::get_header(&self.api_key, &self.api_secret, method, path, "");
        let url: &str = &format!("{}{}", self.endpoint, path);

        let text = client.get(url).headers(headers).send()?.text()?;
        let json: Result<Vec<BitFlyerAsset>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_assets(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn send_order(&self, mut order: Order) -> Result<Order, Box<Error>> {
        let client = Client::new();
        let method = "POST";
        let path = "/v1/me/sendchildorder";
        let params = SendOrderParam{
            product_code: self.product_code.clone(),
            child_order_type: BitFlyerUtils::to_order_type(order.order_type)?,
            side: BitFlyerUtils::to_side(order.side)?,
            price: order.price,
            size: order.size,
        };
        let body = serde_json::to_string(&params)?;
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
            product_code: self.product_code.clone(),
            child_order_acceptance_id: order.id.clone(),
        };
        let body = serde_json::to_string(&params)?;
        let headers = BitFlyerUtils::get_header(&self.api_key, &self.api_secret, method, path, &body);
        let url: &str = &format!("{}{}", self.endpoint, path);

        let text = client.post(url).headers(headers).body(body).send()?.text()?;
        let json: Result<SendOrderResponse, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(_) => return Ok(order),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_bitflyer() {
        // TODO reqwestをモックしてテストを書く
    }
}
