use std::error::Error;

use reqwest::Proxy;
use serde_json;

use crate::market::bitflyer::boards::BitFlyerBoards;
use crate::market::bitflyer::execution::BitFlyerExecution;
use crate::market::bitflyer::order::BitFlyerOrder;
use crate::market::bitflyer::utils::BitFlyerUtils;
use crate::types::atomic::{Boards, Execution, Order};
use crate::types::market::Market;
use crate::utils::market::MarketUtils;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BitFlyer {
    pub url: String,
    pub api_version: String,
    pub api_key: String,
    pub api_secret: String,
    pub product_code: String,
}

impl Market for BitFlyer {
    fn boards(&self, proxy: Option<Proxy>) -> Result<Boards, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let url: &str = &format!("{}/{}{}", self.url, self.api_version, "/board");
        let params = [("product_code", self.product_code.clone())];

        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<BitFlyerBoards, serde_json::Error> = serde_json::from_str(&text);
        // TODO ソートする
        match json {
            Ok(res) => return Ok(MarketUtils::to_boards(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn executions(&self, proxy: Option<Proxy>) -> Result<Vec<Execution>, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let url: &str = &format!("{}/{}{}", self.url, self.api_version, "/executions");
        let params = [("product_code", self.product_code.clone())];

        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<Vec<BitFlyerExecution>, serde_json::Error> = serde_json::from_str(&text);
        // TODO ソートする
        match json {
            Ok(res) => return Ok(MarketUtils::to_executions(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn orders(&self, proxy: Option<Proxy>) -> Result<Vec<Order>, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let method = "GET";
        let path = &format!("/{}/me/getchildorders?product_code={}", self.api_version, self.product_code);
        let headers = BitFlyerUtils::get_header(&self.api_key, &self.api_secret, method, path, "");
        let url: &str = &format!("{}/{}", self.url, path);

        let text = client.get(url).headers(headers).send()?.text()?;
        let json: Result<Vec<BitFlyerOrder>, serde_json::Error> = serde_json::from_str(&text);
        // TODO ソートする
        match json {
            Ok(res) => return Ok(MarketUtils::to_orders(res)),
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
