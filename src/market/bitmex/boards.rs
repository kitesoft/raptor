use crate::types::atomic::MarketBoard;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BitmexBoard {
    pub symbol: String,
    pub id: i64,
    pub side: String,
    pub size: f64,
    pub price: f64,
}

impl MarketBoard for BitmexBoard {
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
}
