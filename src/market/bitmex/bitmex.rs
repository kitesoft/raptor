use std::error::Error;

use reqwest::Proxy;
use serde_json;

use crate::market::bitmex::boards::BitmexBoard;
use crate::market::bitmex::execution::BitmexExecution;
use crate::market::bitmex::order::BitmexOrder;
use crate::market::bitmex::utils::BitmexUtils;
use crate::types::atomic::{Boards, Execution, Order};
use crate::types::market::Market;
use crate::utils::market::MarketUtils;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Bitmex {
    pub symbol: String,
    pub api_key: String,
    pub api_secret: String,
}

impl Market for Bitmex {
    fn boards(&self, proxy: Option<Proxy>) -> Result<Boards, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let url: &str = "https://www.bitmex.com/api/v1/orderBook/L2";
        let params = [
            ("symbol", self.symbol.clone()),
            ("depth", "0".to_string()),
        ];
        
        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<Vec<BitmexBoard>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => {
                let mut bid: Vec<BitmexBoard> = res.iter().filter(|&x| x.side == "Sell").cloned().collect();
                let mut ask: Vec<BitmexBoard> = res.iter().filter(|&x| x.side == "Buy").cloned().collect();

                bid.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
                ask.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

                Ok(Boards{
                    bid: MarketUtils::to_board(bid),
                    ask: MarketUtils::to_board(ask),
                })
            },
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn executions(&self, proxy: Option<Proxy>) -> Result<Vec<Execution>, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let url: &str = "https://www.bitmex.com/api/v1/trade";
        let params = [
            ("symbol", self.symbol.clone()),
            ("count", "100".to_string()),
            ("reverse", "true".to_string()),
        ];
        
        let text = client.get(url).query(&params).send()?.text()?;
        let json: Result<Vec<BitmexExecution>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_executions(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }

    fn orders(&self, proxy: Option<Proxy>) -> Result<Vec<Order>, Box<Error>> {
        let client = MarketUtils::get_client(proxy);
        let path = "/api/v1/order?count=100&reverse=true";
        let url = format!("https://www.bitmex.com{}", path);
        let headers = BitmexUtils::get_header(self.api_key.clone(), self.api_secret.clone(), "GET".to_string(), path.to_string(), "".to_string());
        
        let text = client.get(&url).headers(headers).send()?.text()?;
        let json: Result<Vec<BitmexOrder>, serde_json::Error> = serde_json::from_str(&text);
        match json {
            Ok(res) => return Ok(MarketUtils::to_orders(res)),
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitflyer() {
        let bm = Bitmex{
            symbol: "XBT".to_string(),
            api_key: "".to_string(),
            api_secret: "".to_string(),
        };

        // let boards = bm.boards(None);
        // println!("{:?}", boards);

        // let executions = bm.executions(None);
        // println!("{:?}", executions);

        // let orders = bm.orders(None);
        // println!("{:?}", orders);
    }
}

