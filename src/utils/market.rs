use crate::types::atomic::Side;

pub fn parse_side(side: &str) -> Side {
    if side.to_lowercase() == "buy" {
        return Side::Buy
    }
    Side::Sell
}
