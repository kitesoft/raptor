use chrono::prelude::*;
use crate::types::atomic::{MarketOrder, Side, OrderType, OrderStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmexOrder {
    pub orderID: String,
    pub clOrdID: String,
    pub clOrdLinkID: String,
    pub account: f64,
    pub symbol: String,
    pub side: String,
    pub simpleOrderQty: f64,
    pub orderQty: f64,
    pub price: f64,
    pub displayQty: f64,
    pub stopPx: f64,
    pub pegOffsetValue: f64,
    pub pegPriceType: String,
    pub currency: String,
    pub settlCurrency: String,
    pub ordType: String,
    pub timeInForce: String,
    pub execInst: String,
    pub contingencyType: String,
    pub exDestination: String,
    pub ordStatus: String,
    pub triggered: String,
    pub workingIndicator: bool,
    pub ordRejReason: String,
    pub simpleLeavesQty: f64,
    pub leavesQty: f64,
    pub simpleCumQty: f64,
    pub cumQty: f64,
    pub avgPx: f64,
    pub multiLegReportingType: String,
    pub text: String,
    pub transactTime: String,
    pub timestamp: String,
}

impl MarketOrder for BitmexOrder {
    fn id(&self) -> String {self.orderID.clone()}
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.orderQty}
    fn side(&self) -> Side {if self.side == "Buy" {Side::Buy} else {Side::Sell}}
    fn order_status(&self) -> OrderStatus {OrderStatus::None}
    fn order_at(&self) -> DateTime<Utc> {self.timestamp.parse::<DateTime<Utc>>().unwrap()}

    fn order_type(&self) -> OrderType {
        match self.ordType.as_str() {
            "Market" => return OrderType::Market,
            "Limit" => return OrderType::Limit,
            _ => return OrderType::None,
        }
    }
}
