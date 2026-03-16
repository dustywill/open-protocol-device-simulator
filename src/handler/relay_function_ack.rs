use crate::handler::{HandlerError, MidHandler};
use crate::protocol::{Message, Response};

pub struct RelayFunctionAckHandler;

impl MidHandler for RelayFunctionAckHandler {
    fn handle(&self, _message: &Message) -> Result<Response, HandlerError> {
        println!("MID 0218: Relay function acknowledged by client");
        Ok(Response::new(5, 1, Vec::new()))
    }
}
