use chrono::prelude::*;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use reqwest::header::{HeaderMap};

pub struct BitFlyerUtils {}

impl BitFlyerUtils {
    fn sign(input: String, key: String) -> String {
        let mut hmac = Hmac::new(Sha256::new(), key.as_bytes());
        hmac.input(input.as_bytes());
        hmac.result().code().iter().map(|n| format!("{:02x}", n)).collect::<String>()
    }

    pub fn get_header(api_key: &str, api_secret: &str, method: &str, path: &str, body: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let timestamp = Utc::now().timestamp().to_string();
        let sign = BitFlyerUtils::sign(format!("{}{}{}{}", timestamp, method, path, body), String::from(api_secret));

        headers.insert("ACCESS-KEY",       api_key.parse().unwrap());
        headers.insert("ACCESS-TIMESTAMP", timestamp.parse().unwrap());
        headers.insert("ACCESS-SIGN",      sign.parse().unwrap());
        headers.insert("Content-Type",     "application/json".parse().unwrap());

        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let expected = "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad";
        let input = "".to_string();
        let key = "".to_string();
        assert_eq!(expected, BitFlyerUtils::sign(input, key));
    }
}
