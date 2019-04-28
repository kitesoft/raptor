use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use crate::market::bitflyer::bitflyer::BitFlyer;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub bitflyer: BitFlyer,
}

impl Config {
    pub fn load_config(filename: String) -> Result<Config, Box<Error>>{
        let mut contents = String::new();
        let mut file = File::open(filename)?;
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loaod_config() {
        let filename = "config.yml".to_string();
        let config = Config::load_config(filename).unwrap();
        assert_eq!(config, Config{
            bitflyer: BitFlyer{
                endpoint: "https://api.bitflyer.com".to_string(),
                api_key: "".to_string(),
                api_secret: "".to_string(),
                product_code: "BTC_JPY".to_string(),
            },
        });
    }
}
