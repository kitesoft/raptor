pub struct Config {
    pub bitflyer: BitFlyer,
}

pub struct BitFlyer {
    pub api_key: String,
    pub api_secret: String,
    pub endpoint: String,
}
