use chrono::prelude::*;
use sha2::Sha256;
use hmac::{Hmac, Mac};

pub struct BitFlyerUtils {}

impl BitFlyerUtils {
    pub fn get_header(api_key: &'static str, api_secret: &'static str, method: &'static str, path: &'static str, body: &'static str) {
        let timestamp = Utc::now().timestamp().to_string();
        let text = format!("{}{}{}{}", timestamp, method, path, body);
        let mut mac = Hmac<Sha256>::new_varkey(b"my secret and secure key") .expect("HMAC can take key of any size");
        mac.input(text.as_bytes());
        let sign = mac.result();
    }
}

// var timestamp = Date.now().toString();
// var method = 'POST';
// var path = '/v1/me/sendchildorder';
// var body = JSON.stringify({
//     product_code: 'BTC_JPY',
//     child_order_type: 'LIMIT',
//     side: 'BUY',
//     price: 30000,
//     size: 0.1
// });
//
// var text = timestamp + method + path + body;
// var sign = crypto.createHmac('sha256', secret).update(text).digest('hex');
//
// var options = {
//     url: 'https://api.bitflyer.com' + path,
//     method: method,
//     body: body,
//     headers: {
//         'ACCESS-KEY': key,
//         'ACCESS-TIMESTAMP': timestamp,
//         'ACCESS-SIGN': sign,
//         'Content-Type': 'application/json'
//     }
// };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_header() {
        // TODO get_header
        BitFlyerUtils::get_header("", "", "", "", "");
    }
}
