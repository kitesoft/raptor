use std::error::Error;

pub trait Algo
{
    fn on_init(&mut self);
    fn on_update(&mut self) -> Result<(), Box<Error>>;
    fn on_error(&mut self, error: Box<Error>);
}
