use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use crate::raptor::types::config::Config as RaptorConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub raptor: RaptorConfig,
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