use crate::handler::{HandlerError, MidHandler};
use crate::protocol::{Message, Response};

/// MID 0016 - Parameter set selected acknowledge
/// Client sends this to acknowledge receipt of MID 0015.
/// No response is sent back for this acknowledgement.
pub struct PsetSelectedAckHandler;

impl MidHandler for PsetSelectedAckHandler {
    fn handle(&self, _message: &Message) -> Result<Response, HandlerError> {
        println!("MID 0016: Parameter set selected acknowledged by client");

        // The main loop suppresses responses for ACK-only MIDs.
        Ok(Response::new(5, 1, Vec::new()))
    }
}
