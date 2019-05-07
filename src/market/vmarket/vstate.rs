use crate::types::atomic::{Order, Asset};

pub struct VState {
    orders: Vec<Order>,
    assets: Vec<Asset>,
}

impl VState {
    pub fn new() -> Self {
        VState{
            orders: vec!(),
            assets: vec!(),
        }
    }

    pub fn assets(&self) -> Vec<Asset> {
        self.assets
    }

    pub fn orders(&self) -> Vec<Order> {
        self.orders
    }

    pub fn update(&self, now: f64) {
        // TODO ordersの取得
        // TODO status == ActiveなOrderの取得

        // TODO Orderの更新処理
        // TODO Assetの更新処理
    }

    pub fn push(&self, mut order: Order) -> Order {
        order.id = "dummy".to_string(); // TODO idをユニークにする
        self.orders.push(order.clone());
        order
    }

    // FIXME log(n)
    pub fn remove(&self, order: Order) -> Order {
        let mut ix = vec!();
        for (i, o) in self.orders.iter().enumerate() {
            if o.id == order.id {
                ix.push(i);
            }
        }

        for i in ix {
            self.orders.remove(i);
        }

        order
    }
}
