use crate::types::atomic::Side;

pub fn parse_side(side: &str) -> Side {
    let side = side.replace(" ", "");
    if side.to_lowercase() == "buy" {
        return Side::Buy
    }
    Side::Sell
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_side() {
        // normal
        assert_eq!(Side::Buy, parse_side("buy"));
        assert_eq!(Side::Sell, parse_side("sell"));
        
        // uppercase
        assert_eq!(Side::Buy, parse_side("BUY"));
        assert_eq!(Side::Sell, parse_side("SELL"));
        
        // whitespace
        assert_eq!(Side::Buy, parse_side(" Buy"));
        assert_eq!(Side::Sell, parse_side(" Sell"));
    }
}
