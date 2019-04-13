use crate::types::market::{MarketBoard, MarketBoards};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BitflyerBoard {
    pub price: f64,
    pub size: f64,
}

impl MarketBoard for BitflyerBoard {
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BitflyerBoards {
    pub mid_price: f64,
    pub bids: Vec<BitflyerBoard>,
    pub asks: Vec<BitflyerBoard>,
}

impl MarketBoards<BitflyerBoard> for BitflyerBoards {
    fn bids(&self) -> Vec<BitflyerBoard> {self.bids.clone()}
    fn asks(&self) -> Vec<BitflyerBoard> {self.asks.clone()}
}
