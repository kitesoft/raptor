use crate::types::atomic::{MarketBoard, MarketBoards};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BitFlyerBoard {
    pub price: f64,
    pub size: f64,
}

impl MarketBoard for BitFlyerBoard {
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BitFlyerBoards {
    pub mid_price: f64,
    pub bids: Vec<BitFlyerBoard>,
    pub asks: Vec<BitFlyerBoard>,
}

impl MarketBoards<BitFlyerBoard> for BitFlyerBoards {
    fn bids(&self) -> Vec<BitFlyerBoard> {self.bids.clone()}
    fn asks(&self) -> Vec<BitFlyerBoard> {self.asks.clone()}
}
