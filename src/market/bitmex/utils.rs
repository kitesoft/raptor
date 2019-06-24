use std::error::Error;
use chrono::prelude::*;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use reqwest::header::{HeaderMap};

use crate::types::atomic::{Side, OrderType};

pub struct BitmexUtils {}

impl BitmexUtils {
    fn sign(secret: String, method: String, path: String, expires: String, body: String) -> String {
        let input = method+&path+&expires+&body;
        let mut hmac = Hmac::new(Sha256::new(), secret.as_bytes());
        hmac.input(input.as_bytes());
        hmac.result().code().iter().map(|n| format!("{:02x}", n)).collect::<String>()
    }

    pub fn get_header(api_key: String, api_secret: String, method: String, path: String, body: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let expires = (Utc::now().timestamp() + 5).to_string();
        let sign = BitmexUtils::sign(api_secret, method, path, expires.clone(), body);
        
        headers.insert("api-key",           api_key.parse().unwrap());
        headers.insert("api-expires",       expires.parse().unwrap());
        headers.insert("api-signature",     sign.parse().unwrap());
        headers.insert("Content-Type",      "application/json".parse().unwrap());

        headers
    }
}
