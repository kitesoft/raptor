use crate::types::atomic::{Order, Board, Boards, Execution, Asset, MarketOrder, MarketExecution, MarketBoard, MarketBoards, MarketAsset};

pub struct MarketUtils {}

impl MarketUtils {
    pub fn to_order<T>(order: T) -> Order where T: MarketOrder {
        Order{
            id: order.id(),
            order_type: order.order_type(),
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

    pub fn to_assets<T>(assets: Vec<T>) -> Vec<Asset> where T: MarketAsset + Clone {
        let mut items: Vec<Asset> = vec!();
        for item in assets {
            items.push(Asset{
                currency: item.currency(),
                amount: item.amount(),
            })
        }
        items
    }
}
