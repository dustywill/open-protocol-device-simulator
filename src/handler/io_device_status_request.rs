use crate::handler::data::IoDeviceStatus;
use crate::handler::{HandlerError, MidHandler};
use crate::observable_state::ObservableState;
use crate::protocol::{Message, Response};

pub fn parse_device_number(message: &Message) -> Result<u8, HandlerError> {
    let device = std::str::from_utf8(&message.data)
        .map_err(|_| HandlerError::InvalidData(214))?
        .trim()
        .parse::<u8>()
        .map_err(|_| HandlerError::InvalidData(214))?;

    if device == 0 {
        Ok(device)
    } else {
        Err(HandlerError::InvalidData(214))
    }
}

pub struct IoDeviceStatusRequestHandler {
    state: ObservableState,
}

impl IoDeviceStatusRequestHandler {
    pub fn new(state: ObservableState) -> Self {
        Self { state }
    }
}

impl MidHandler for IoDeviceStatusRequestHandler {
    fn handle(&self, message: &Message) -> Result<Response, HandlerError> {
        let device_number = parse_device_number(message)?;
        let state = self.state.read();
        let response = IoDeviceStatus::new(
            device_number,
            vec![
                (20, state.tool_start_switch_active),
                (22, state.direction_ccw_relay_active()),
                (0, false),
                (0, false),
                (0, false),
                (0, false),
                (0, false),
                (0, false),
            ],
            vec![(0, false); 8],
        );
        println!("MID 0214: IO device status request for device {:02}", device_number);
        Ok(Response::from_data(215, message.revision, response))
    }
}
