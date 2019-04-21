#[derive(Serialize, Deserialize, Debug)]
pub struct SendOrderParam {
    pub product_code: String,
    pub child_order_type: String,
    pub side: String,
    pub price: f64,
    pub size: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendOrderResponse {
    pub child_order_acceptance_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrderParam {
    pub product_code: String,
    pub child_order_acceptance_id: String,
}
