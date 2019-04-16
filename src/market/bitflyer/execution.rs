use chrono::prelude::*;

use crate::utils::market::parse_side;
use crate::types::atomic::MarketExecutions;
use crate::types::atomic::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct BitflyerExecution {
    pub id: i64,
    pub side: String,
    pub price: f64,
    pub size: f64,
    pub exec_date: String,
    pub buy_child_order_acceptance_id: String,
    pub sell_child_order_acceptance_id: String,
}

impl MarketExecutions for BitflyerExecution {
    fn side(&self) -> Side {parse_side(&self.side)}
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
    fn exec_date(&self) -> DateTime<Utc> {format!("{}Z", self.exec_date).parse::<DateTime<Utc>>().unwrap()}
}
