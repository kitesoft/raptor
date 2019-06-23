use chrono::prelude::*;
use crate::types::atomic::{MarketOrder, Side, OrderType, OrderStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidOrders {
    pub models: Vec<LiquidOrder>,
    current_page: i32,
    total_pages: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidOrder {
    pub id: i64,
    pub order_type: String,
    pub quantity: f64,
    pub disc_quantity: f64,
    pub iceberg_total_quantity: f64,
    pub side: String,
    pub filled_quantity: f64,
    pub price: f64,
    pub created_at: i64,
    pub updated_at: i64,
    pub status: String,
    pub leverage_level: i32,
    pub source_exchange: String,
    pub product_id: i32,
    pub product_code: String,
    pub funding_currency: String,
    pub currency_pair_code: String,
    pub order_fee: f64,
}

impl MarketOrder for LiquidOrder {
    fn id(&self) -> String {self.id.to_string()}
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.quantity}
    fn side(&self) -> Side {if self.side == "buy" {Side::Buy} else {Side::Sell}}
    fn order_status(&self) -> OrderStatus {OrderStatus::None}

    fn order_type(&self) -> OrderType {
        match self.order_type.as_str() {
            "market" => return OrderType::Market,
            "limit" => return OrderType::Limit,
            _ => return OrderType::None,
        }
    }

    fn order_at(&self) -> DateTime<Utc> {Utc.timestamp(self.created_at, 0)}
}
