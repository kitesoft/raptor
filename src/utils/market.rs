use reqwest::Client;
use reqwest::Proxy;

use crate::types::atomic::{Order, Board, Boards, Execution, MarketOrder, MarketExecution, MarketBoard, MarketBoards};

pub struct MarketUtils {}

impl MarketUtils {
    pub fn get_client(proxy: Option<Proxy>) -> Client {
        match proxy {
            Some(proxy) => {
                match Client::builder().proxy(proxy).build() {
                    Ok(client) => client,
                    _ => Client::new(),
                }
            },
            _ => Client::new(),
        }
    }

    pub fn to_order<T>(order: T) -> Order where T: MarketOrder {
        Order{
            id: order.id(),
            order_type: order.order_type(),
            order_status: order.order_status(),
            side: order.side(),
            price: order.price(),
            size: order.size(),
        }
    }

    pub fn to_orders<T>(orders: Vec<T>) -> Vec<Order> where T: MarketOrder {
        let mut items: Vec<Order> = vec!();
        for item in orders {
            items.push(Order{
                id: item.id(),
                order_type: item.order_type(),
                order_status: item.order_status(),
                side: item.side(),
                price: item.price(),
                size: item.size(),
            });
        }
        items
    }

    pub fn to_executions<T>(executions: Vec<T>) -> Vec<Execution> where T: MarketExecution {
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

    pub fn to_board<T>(boards: Vec<T>) -> Vec<Board> where T: MarketBoard {
        let mut items: Vec<Board> = vec!();
        for item in boards {
            items.push(Board{
                price: item.price(),
                size: item.size(),
            });
        }
        items
    }

    pub fn to_boards<T, U>(board: T) -> Boards where U: MarketBoard + Clone, T: MarketBoards<U> + Clone {
        Boards{
            bid: MarketUtils::to_board(board.bids()),
            ask: MarketUtils::to_board(board.asks()),
        }
    }
}
