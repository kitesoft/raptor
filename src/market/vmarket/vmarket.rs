use std::error::Error;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::types::market::Market;
use crate::types::atomic::{Boards, Execution, Order, Asset};
use crate::market::vmarket::vserver::VServer;
use crate::market::vmarket::vstate::VState;

pub struct VMarket {
    id: String,
    state: Arc<RwLock<VState>>,
}

impl VMarket {
    #[allow(dead_code)]
    fn new(market: Box<Market>, assets: Vec<Asset>, delay: Duration) -> Self {
        let id = market.unique_id();
        let market = Arc::new(RwLock::new(market));
        let state = Arc::new(RwLock::new(VState::new()));

        VServer::start(
            market,
            state,
            delay,
        );

        VMarket{
            id: id,
            state: state
        }
    }
}

impl Market for VMarket {
    fn unique_id(&self) -> String {
        format!("vmarket-{}", self.id)
    }

    fn boards(&self) -> Result<Boards, Box<Error>> {
        // self.server.boards()
    }

    fn executions(&self) -> Result<Vec<Execution>, Box<Error>> {
        // self.server.executions()
        Ok(vec!())
    }

    fn orders(&self) -> Result<Vec<Order>, Box<Error>> {
        match self.state.write() {
            Ok(v) => Ok(v.orders()),
            _ => Err(),
        }
    }

    fn assets(&self) -> Result<Vec<Asset>, Box<Error>> {
        match self.state.write() {
            Ok(v) => Ok(v.assets()),
            _ => Err(),
        }
    }

    fn send_order(&self, order: Order) -> Result<Order, Box<Error>> {
        match self.state.write() {
            Ok(v) => Ok(v.push(order)),
            _ => Err(),
        }
    }

    fn cancel_order(&self, order: Order) -> Result<Order, Box<Error>> {
        match self.state.write() {
            Ok(v) => Ok(v.remove(order)),
            _ => Err(),
        }
    }
}
