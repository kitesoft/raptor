use std::error::Error;

use raptor::types::algo::{Algo, Action, State};

pub struct ExampleAlgo {}

impl Algo for ExampleAlgo {
    fn on_init(&self) {
        println!("on_init handler")
    }

    fn on_update(&self, state: &State, action: &Action) {
        println!("on_update handler")
    }

    fn on_error(&self, error: Box<Error>) {
        println!("{}", error);
    }
}
