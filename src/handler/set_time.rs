use crate::handler::data::CommandAccepted;
use crate::handler::{HandlerError, MidHandler};
use crate::observable_state::ObservableState;
use crate::protocol::{Message, Response};
use chrono::NaiveDateTime;

pub struct SetTimeHandler {
    state: ObservableState,
}

impl SetTimeHandler {
    pub fn new(state: ObservableState) -> Self {
        Self { state }
    }
}

impl MidHandler for SetTimeHandler {
    fn handle(&self, message: &Message) -> Result<Response, HandlerError> {
        let timestamp = std::str::from_utf8(&message.data).map_err(|_| HandlerError::InvalidData(82))?;
        let timestamp = NaiveDateTime::parse_from_str(timestamp.trim(), "%Y-%m-%d:%H:%M:%S")
            .map_err(|_| HandlerError::InvalidData(82))?;

        println!("MID 0082: Set controller time to {}", timestamp.format("%Y-%m-%d:%H:%M:%S"));

        self.state.write().set_controller_time(timestamp);

        Ok(Response::from_data(5, message.revision, CommandAccepted::with_mid(82)))
    }
}
