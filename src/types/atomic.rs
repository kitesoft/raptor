use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Boards {
    pub bid: Vec<Board>,
    pub ask: Vec<Board>,
}

#[derive(Debug, Clone, Copy)]
pub struct Execution {
    pub side: Side,
    pub price: f64,
    pub size: f64,
    pub exec_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub order_type: OrderType,
    pub side: Side,
    pub price: f64,
    pub size: f64,
}

pub trait MarketOrder {
    fn id(&self) -> String;
    fn order_type(&self) -> OrderType;
    fn side(&self) -> Side;
    fn price(&self) -> f64;
    fn size(&self) -> f64;
}

pub trait MarketExecution {
    fn side(&self) -> Side;
    fn price(&self) -> f64;
    fn size(&self) -> f64;
    fn exec_date(&self) -> DateTime<Utc>;
}

pub trait MarketBoard {
    fn price(&self) -> f64;
    fn size(&self) -> f64;
}

pub trait MarketBoards<T>
where T: MarketBoard
{
    fn bids(&self) -> Vec<T>;
    fn asks(&self) -> Vec<T>;
}

