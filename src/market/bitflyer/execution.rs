use chrono::prelude::*;

use crate::types::atomic::MarketExecution;
use crate::types::atomic::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct BitFlyerExecution {
    pub id: i64,
    pub side: String,
    pub price: f64,
    pub size: f64,
    pub exec_date: String,
    pub buy_child_order_acceptance_id: String,
    pub sell_child_order_acceptance_id: String,
}

impl MarketExecution for BitFlyerExecution {
    fn side(&self) -> Side {if self.side == "BUY" {Side::Buy} else {Side::Sell}}
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
    fn exec_date(&self) -> DateTime<Utc> {format!("{}Z", self.exec_date).parse::<DateTime<Utc>>().unwrap()}
}
