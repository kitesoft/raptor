use crate::types::atomic::{Currency, MarketAsset};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitFlyerAsset {
    pub currency_code: String,
    pub amount: f64,
    pub available: f64,
}

impl MarketAsset for BitFlyerAsset {
    fn currency(&self) -> Currency {
        match self.currency_code.as_str() {
            "BTC" => Currency::BTC,
            "JPY" => Currency::JPY,
            "ETH" => Currency::ETH,
            _ => Currency::None
        }
    }

    fn amount(&self) -> f64 {
        self.available
    }
}
