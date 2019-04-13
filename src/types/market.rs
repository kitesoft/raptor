use std::sync::{Arc, Mutex};
use chrono::prelude::*;

// TODO 大きすぎるので分割する

#[derive(Debug, Clone)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    Market,
    Limit,
}

pub enum ProductCode {
    BTC_JPY,
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

#[derive(Debug, Clone)]
pub struct Execution {
    pub side: Side,
    pub price: f64,
    pub size: f64,
    pub exec_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
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

pub trait MarketExecutions {
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

pub trait Market {
    fn new() -> Arc<Mutex<Self>>;

    // Get
    fn boards(&mut self) -> Result<Boards, reqwest::Error>;
    fn executions(&mut self) -> Result<Vec<Execution>, reqwest::Error>;
    fn orders(&mut self) -> Result<Vec<Order>, reqwest::Error>;

    // Actions
    fn send_order(&self, order: Order) -> Result<Order, reqwest::Error>;
    fn cancel_order(&self, order: Order) -> Result<Order, reqwest::Error>;

    // Converter
    fn to_order<T>(&self, order: T) -> Order where T: MarketOrder {
        Order{
            id: order.id(),
            order_type: order.order_type(),
            side: order.side(),
            price: order.price(),
            size: order.size(),
        }
    }

    fn to_executions<T>(&self, executions: Vec<T>) -> Vec<Execution> where T: MarketExecutions {
        let mut items: Vec<Execution> = vec!();
        for item in executions {
            items.push(Execution{
                side: item.side(),
                price: item.price(),
                size: item.size(),
                exec_date: item.exec_date(),
            });
        }
        items
    }

    fn to_board<T>(&self, boards: Vec<T>) -> Vec<Board> where T: MarketBoard {
        let mut items: Vec<Board> = vec!();
        for item in boards {
            items.push(Board{
                price: item.price(),
                size: item.size(),
            });
        }
        items
    }

    fn to_boards<T, U>(&self, board: T) -> Boards where U: MarketBoard + Clone, T: MarketBoards<U> + Clone {
        Boards{
            bid: self.to_board(board.bids()),
            ask: self.to_board(board.asks()),
        }
    }
}
