use std::sync::{Arc, Mutex};

use crate::types::algo::Algo;
use crate::types::atomic::{Order, Board, Boards, Execution, MarketOrder, MarketExecutions, MarketBoard, MarketBoards};

pub trait Market {
    fn new() -> Arc<Mutex<Self>>;

    // Get
    fn boards(&mut self) -> Result<Boards, reqwest::Error>;
    fn executions(&mut self) -> Result<Vec<Execution>, reqwest::Error>;
    fn orders(&mut self) -> Result<Vec<Order>, reqwest::Error>;

    // Actions
    fn send_order(&self, order: Order) -> Result<Order, reqwest::Error>;
    fn cancel_order(&self, order: Order) -> Result<Order, reqwest::Error>;

    // Getter, Setter
    fn get_algos(&mut self) -> &mut Vec<Box<Algo>>;

    // Observer
    fn register<T>(&mut self, algo: Box<T>) where T: Algo + 'static { self.get_algos().push(algo); } // TODO lifetimeのstaticをやめる

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
