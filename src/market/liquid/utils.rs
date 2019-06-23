use chrono::prelude::*;
use reqwest::header::{HeaderMap};
use jwt::{encode, Header};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    path: String,
    nonce: String,
    token_id: String,
}

pub struct LiquidUtils {}

impl LiquidUtils {
    pub fn get_header(token: &str, secret: &str, path: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let timestamp = Utc::now().timestamp().to_string();
        let sign = LiquidUtils::sign(path.to_string(), timestamp, token.to_string(), secret.to_string());

        headers.insert("X-Quoine-API-Version",      "2".parse().unwrap());
        headers.insert("X-Quoine-Auth",             sign.parse().unwrap());
        headers.insert("Content-Type",              "application/json".parse().unwrap());

        headers
    }

    fn sign(path: String, nonce: String, token: String, secret: String) -> String {
        let payload = Payload{
            path: path,
            nonce: nonce,
            token_id: token,
        };
        encode(&Header::default(), &payload, secret.as_ref()).unwrap()
    }
}
