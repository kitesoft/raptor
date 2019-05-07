use std::thread;
use std::thread::sleep;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::types::market::Market;
use crate::market::vmarket::vstate::VState;

pub struct VServer {}

impl VServer {
    pub fn start(market: Arc<RwLock<Box<Market>>>, mut state: Arc<RwLock<VState>>, delay: Duration) {
        thread::spawn(move || {
            loop {
                // TODO executionsの取得
                // for execution in executions {
                //   state.update(execution)
                // }

                sleep(delay);
            }
        });
    }
}
