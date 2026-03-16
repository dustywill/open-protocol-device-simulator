use crate::handler::data::CommandAccepted;
use crate::handler::{HandlerError, MidHandler};
use crate::protocol::{Message, Response};

pub fn parse_relay_function(message: &Message) -> Result<u16, HandlerError> {
    let relay = std::str::from_utf8(&message.data)
        .map_err(|_| HandlerError::InvalidData(216))?
        .trim()
        .parse::<u16>()
        .map_err(|_| HandlerError::InvalidData(216))?;

    match relay {
        20 | 22 => Ok(relay),
        _ => Err(HandlerError::InvalidData(216)),
    }
}

pub struct RelayFunctionSubscribeHandler;

impl MidHandler for RelayFunctionSubscribeHandler {
    fn handle(&self, message: &Message) -> Result<Response, HandlerError> {
        let relay = parse_relay_function(message)?;
        println!("MID 0216: Relay function subscribe - relay {}", relay);
        Ok(Response::from_data(5, message.revision, CommandAccepted::with_mid(216)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_relay_function() {
        let message = Message {
            length: 23,
            mid: 216,
            revision: 1,
            data: b"022".to_vec(),
        };
        assert_eq!(parse_relay_function(&message).unwrap(), 22);
    }
}
