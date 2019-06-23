use chrono::prelude::*;

use crate::types::atomic::MarketExecution;
use crate::types::atomic::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidExecutions {
    pub models: Vec<LiquidExecution>,
    current_page: i64,
    total_pages: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidExecution {
    id: i64,
    quantity: f64,
    price: f64,
    taker_side: String,
    created_at: i64,
}

impl MarketExecution for LiquidExecution {
    fn side(&self) -> Side {if self.taker_side == "buy" {Side::Buy} else {Side::Sell}}
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.quantity}
    fn exec_date(&self) -> DateTime<Utc> {Utc.timestamp(self.created_at, 0)}
}
