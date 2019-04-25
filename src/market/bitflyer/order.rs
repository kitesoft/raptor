use crate::types::atomic::{MarketOrder, Side, OrderType};

#[derive(Debug, Serialize, Deserialize)]
pub struct BitFlyerOrder {
    pub id: i64,
    pub child_order_id: String,
    pub product_code: String,
    pub side: String,
    pub child_order_type: String,
    pub price: f64,
    pub average_price: f64,
    pub size: f64,
    pub child_order_state: String,
    pub expire_date: String,
    pub child_order_date: String,
    pub child_order_acceptance_id: String,
    pub outstanding_size: f64,
    pub cancel_size: f64,
    pub executed_size: f64,
    pub total_commission: f64,
}

impl MarketOrder for BitFlyerOrder {
    fn id(&self) -> String {self.child_order_id.clone()}
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
    fn side(&self) -> Side {if self.side == "BUY" {Side::Buy} else {Side::Sell}}

    fn order_type(&self) -> OrderType {
        match self.child_order_type.as_str() {
            "MARKET" => return OrderType::Market,
            "LIMIT" => return OrderType::Limit,
            _ => return OrderType::None,
        }
    }
}
