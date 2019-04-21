use std::sync::{Arc, Mutex};
use std::error::Error;

use crate::types::algo::Algo;
use crate::types::config::Config;
use crate::types::atomic::{Side, OrderType, ProductCode, Order, Board, Boards, Execution, MarketOrder, MarketExecution, MarketBoard, MarketBoards};

pub trait Market {
    fn new(config: Config) -> Arc<Mutex<Self>>;

    // Get
    fn boards(&mut self, product_code: ProductCode) -> Result<Boards, Box<Error>>;
    fn executions(&mut self, product_code: ProductCode) -> Result<Vec<Execution>, Box<Error>>;
    fn orders(&mut self, product_code: ProductCode) -> Result<Vec<Order>, Box<Error>>;

    // Actions
    fn send_order(&self, order: Order) -> Result<Order, Box<Error>>;
    fn cancel_order(&self, order: Order) -> Result<Order, Box<Error>>;

    // Getter, Setter
    fn get_algos(&mut self) -> &mut Vec<Box<Algo>>;

    // Observer
    fn register<T>(&mut self, algo: Box<T>) where T: Algo + 'static { self.get_algos().push(algo); } // TODO lifetimeのstaticをやめる
    // TODO fn run

    // Converter
    fn to_side(&self, side: Side) -> Result<String, Box<Error>>;
    fn to_order_type(&self, order_type: OrderType) -> Result<String, Box<Error>>;
    fn to_product_code(&self, product_code: ProductCode) -> Result<String, Box<Error>>;

    fn to_order<T>(&self, order: T) -> Order where T: MarketOrder {
        Order{
            id: order.id(),
            product_code: order.product_code(),
            order_type: order.order_type(),
            side: order.side(),
            price: order.price(),
            size: order.size(),
        }
    }

    fn to_orders<T>(&self, orders: Vec<T>) -> Vec<Order> where T: MarketOrder {
        let mut items: Vec<Order> = vec!();
        for item in orders {
            items.push(Order{
                id: item.id(),
                product_code: item.product_code(),
                order_type: item.order_type(),
                side: item.side(),
                price: item.price(),
                size: item.size(),
            });
        }
        items
    }

    fn to_executions<T>(&self, executions: Vec<T>) -> Vec<Execution> where T: MarketExecution {
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
