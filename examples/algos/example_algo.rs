use std::error::Error;

use raptor::types::algo::Algo;

pub struct ExampleAlgo {}

impl Algo for ExampleAlgo {
    fn on_init(&mut self) {
        println!("on_init handler")
    }

    fn on_update(&mut self) -> Result<(), Box<Error>> {
        println!("on_update handler");
        Ok(())
    }

    fn on_error(&mut self, error: Box<Error>) {
        println!("{}", error);
    }
}
