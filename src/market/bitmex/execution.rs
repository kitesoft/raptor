use chrono::prelude::*;

use crate::types::atomic::MarketExecution;
use crate::types::atomic::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmexExecution {
    pub timestamp: String,
    pub symbol: String,
    pub side: String,
    pub size: f64,
    pub price: f64,
    pub tickDirection: String,
    pub trdMatchID: String,
    pub grossValue: i64,
    pub homeNotional: f64,
    pub foreignNotional: i64,
}

impl MarketExecution for BitmexExecution {
    fn side(&self) -> Side {if self.side == "Buy" {Side::Buy} else {Side::Sell}}
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
    fn exec_date(&self) -> DateTime<Utc> {self.timestamp.parse::<DateTime<Utc>>().unwrap()}
}
