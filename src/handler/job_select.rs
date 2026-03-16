//! MID 0038 - Job selection handler
//!
//! Selects a specific job for subsequent tightening operations.

use crate::handler::data::command_accepted::CommandAccepted;
use crate::handler::{HandlerError, MidHandler};
use crate::protocol::{Message, Response};
use crate::state::DeviceState;
use std::sync::{Arc, RwLock};

fn parse_job_id(message: &Message) -> Result<u32, HandlerError> {
    let expected_len = match message.revision {
        1 => 2,
        2 => 4,
        _ => {
            return Err(HandlerError::RevisionUnsupported {
                mid: message.mid,
                revision: message.revision,
            });
        }
    };

    if message.data.len() != expected_len {
        return Err(HandlerError::InvalidData(message.mid));
    }

    let raw = std::str::from_utf8(&message.data).map_err(|_| HandlerError::InvalidData(message.mid))?;
    if !raw.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(HandlerError::InvalidData(message.mid));
    }

    raw.parse::<u32>()
        .map_err(|_| HandlerError::InvalidData(message.mid))
}

/// MID 0038 - Select Job
pub struct JobSelectHandler {
    state: Arc<RwLock<DeviceState>>,
}

impl JobSelectHandler {
    pub fn new(state: Arc<RwLock<DeviceState>>) -> Self {
        Self { state }
    }
}

impl MidHandler for JobSelectHandler {
    fn handle(&self, message: &Message) -> Result<Response, HandlerError> {
        let job_id = parse_job_id(message)?;

        println!("MID 0038: Select job - Job ID: {}", job_id);

        {
            let mut state = self.state.write().unwrap();
            state.set_job_id(job_id);
        }

        Ok(Response::from_data(
            5,
            message.revision,
            CommandAccepted::with_mid(38),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_job_id_revision_1() {
        let message = Message {
            length: 22,
            mid: 38,
            revision: 1,
            data: b"07".to_vec(),
        };

        assert_eq!(parse_job_id(&message).unwrap(), 7);
    }

    #[test]
    fn test_parse_job_id_revision_2() {
        let message = Message {
            length: 24,
            mid: 38,
            revision: 2,
            data: b"0042".to_vec(),
        };

        assert_eq!(parse_job_id(&message).unwrap(), 42);
    }

    #[test]
    fn test_parse_job_id_rejects_invalid_data() {
        let message = Message {
            length: 22,
            mid: 38,
            revision: 1,
            data: b"A7".to_vec(),
        };

        assert!(matches!(
            parse_job_id(&message),
            Err(HandlerError::InvalidData(38))
        ));
    }
}
