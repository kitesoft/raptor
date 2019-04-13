use crate::types::market::Side;

pub fn parse_side(side: &str) -> Side {
    if side.to_lowercase() == "buy" {
        return Side::Buy
    }
    Side::Sell
}
