use crate::types::atomic::{MarketBoard, MarketBoards};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidBoard {
    pub price: f64,
    pub size: f64,
}

impl MarketBoard for LiquidBoard {
    fn price(&self) -> f64 {self.price}
    fn size(&self) -> f64 {self.size}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidBoards {
    pub buy_price_levels: Vec<Vec<String>>,
    pub sell_price_levels: Vec<Vec<String>>,
}

impl MarketBoards<LiquidBoard> for LiquidBoards {
    fn bids(&self) -> Vec<LiquidBoard> {
        let mut board = vec!();
        for x in self.sell_price_levels.clone() {
            board.push(LiquidBoard{
                price: x[0].parse().unwrap(),
                size: x[1].parse().unwrap(),
            })
        }
        board
    }

    fn asks(&self) -> Vec<LiquidBoard> {
        let mut board = vec!();
        for x in self.buy_price_levels.clone() {
            board.push(LiquidBoard{
                price: x[0].parse().unwrap(),
                size: x[1].parse().unwrap(),
            })
        }
        board
    }
}
