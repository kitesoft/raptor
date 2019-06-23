use std::error::Error;

use reqwest::Proxy;
use serde_json;

use crate::market::liquid::boards::LiquidBoards;
use crate::market::liquid::execution::LiquidExecutions;
use crate::market::liquid::order::LiquidOrders;
use crate::market::liquid::utils::LiquidUtils;
use crate::types::atomic::{Boards, Execution, Order};
use crate::types::market::Market;
use crate::utils::market::MarketUtils;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Liquid {
    pub token: String,
    pub secret: String,
    pub product_id: String,
}

impl Market for Liquid {
    fn boards(&self, proxy: Option<Proxy>) -> Result<Boards, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let url: &str = &format!("https://api.liquid.com/products/{}/price_levels", self.product_id);

        let text = client.get(url).send()?.text()?;
        let json: Result<LiquidBoards, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return {
                let mut boards = MarketUtils::to_boards(res);
                boards.bid.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
                boards.ask.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
                Ok(boards)
            },
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn executions(&self, proxy: Option<Proxy>) -> Result<Vec<Execution>, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let url: &str = &format!("https://api.liquid.com/{}", "/executions");
        let params = [
            ("product_id", self.product_id.clone()),
            ("limit", "1000".to_string()),
        ];

        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<LiquidExecutions, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_executions(res.models)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn orders(&self, proxy: Option<Proxy>) -> Result<Vec<Order>, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let path = &format!("/orders?product_id={}&limit=1000", self.product_id);
        let headers = LiquidUtils::get_header(&self.token, &self.secret, path);
        let url = &format!("https://api.liquid.com{}", path);
        
        let text = client.get(url).headers(headers).send()?.text()?;
        let json: Result<LiquidOrders, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_orders(res.models)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitflyer() {
        let liquid = Liquid{
            token: "".to_string(),
            secret: "".to_string(),
            product_id: "5".to_string(),
        };

        // let boards = liquid.boards(None);
        // println!("{:?}", boards);

        // let executions = liquid.executions(None).unwrap();
        // println!("{:?}", executions[0]);
        // println!("{:?}", executions.last().unwrap());

        // let orders = liquid.orders(None);
        // println!("{:?}", orders);
    }
}
