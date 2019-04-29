
use crate::market::bitflyer::bitflyer::BitFlyer;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub bitflyer: BitFlyer,
}
