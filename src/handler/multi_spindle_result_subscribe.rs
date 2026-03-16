use crate::handler::data::CommandAccepted;
use crate::handler::{HandlerError, MidHandler};
use crate::protocol::{Message, Response};
use crate::subscriptions::MultiSpindleResultSubscription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultiSpindleResultSubscribeRequest {
    pub data_no_system: Option<u32>,
    pub send_only_new_data: bool,
}

pub fn parse_subscribe_request(
    message: &Message,
) -> Result<MultiSpindleResultSubscribeRequest, HandlerError> {
    match message.revision {
        1 => {
            if message.data.is_empty() {
                Ok(MultiSpindleResultSubscribeRequest {
                    data_no_system: None,
                    send_only_new_data: false,
                })
            } else {
                Err(HandlerError::InvalidData(message.mid))
            }
        }
        2 => {
            if message.data.len() != 10 {
                return Err(HandlerError::InvalidData(message.mid));
            }

            let data_no_system = std::str::from_utf8(&message.data)
                .ok()
                .and_then(|value| value.parse::<u32>().ok())
                .ok_or(HandlerError::InvalidData(message.mid))?;

            Ok(MultiSpindleResultSubscribeRequest {
                data_no_system: Some(data_no_system),
                send_only_new_data: false,
            })
        }
        3 => {
            if message.data.len() != 11 {
                return Err(HandlerError::InvalidData(message.mid));
            }

            let data_no_system = std::str::from_utf8(&message.data[..10])
                .ok()
                .and_then(|value| value.parse::<u32>().ok())
                .ok_or(HandlerError::InvalidData(message.mid))?;
            let send_only_new_data = match message.data[10] {
                b'0' => false,
                b'1' => true,
                _ => return Err(HandlerError::InvalidData(message.mid)),
            };

            Ok(MultiSpindleResultSubscribeRequest {
                data_no_system: Some(data_no_system),
                send_only_new_data,
            })
        }
        _ => Err(HandlerError::RevisionUnsupported {
            mid: message.mid,
            revision: message.revision,
        }),
    }
}

pub fn into_subscription(
    revision: u8,
    request: MultiSpindleResultSubscribeRequest,
    latest_result_id: Option<u32>,
) -> MultiSpindleResultSubscription {
    MultiSpindleResultSubscription::new(
        revision,
        request.data_no_system,
        request.send_only_new_data,
        latest_result_id,
    )
}

/// MID 0100 - Multi-spindle result subscribe
/// Client requests subscription to multi-spindle tightening results
/// Revision 1 contains no data
pub struct MultiSpindleResultSubscribeHandler;

impl MidHandler for MultiSpindleResultSubscribeHandler {
    fn handle(&self, message: &Message) -> Result<Response, HandlerError> {
        println!("MID 0100: Multi-spindle result subscription request");
        parse_subscribe_request(message)?;

        // Acknowledge subscription
        let ack_data = CommandAccepted::with_mid(100);
        Ok(Response::from_data(5, message.revision, ack_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_spindle_result_subscribe() {
        let handler = MultiSpindleResultSubscribeHandler;
        let message = Message {
            length: 20,
            mid: 100,
            revision: 1,
            data: vec![],
        };

        let response = handler.handle(&message).unwrap();
        assert_eq!(response.mid, 5); // Command accepted
    }

    #[test]
    fn test_parse_multi_spindle_result_subscribe_revision_2() {
        let message = Message {
            length: 30,
            mid: 100,
            revision: 2,
            data: b"0000000042".to_vec(),
        };

        let request = parse_subscribe_request(&message).unwrap();
        assert_eq!(request.data_no_system, Some(42));
        assert!(!request.send_only_new_data);
    }

    #[test]
    fn test_parse_multi_spindle_result_subscribe_revision_3() {
        let message = Message {
            length: 31,
            mid: 100,
            revision: 3,
            data: b"00000000011".to_vec(),
        };

        let request = parse_subscribe_request(&message).unwrap();
        assert_eq!(request.data_no_system, Some(1));
        assert!(request.send_only_new_data);
    }

    #[test]
    fn test_parse_multi_spindle_result_subscribe_invalid_data() {
        let message = Message {
            length: 30,
            mid: 100,
            revision: 2,
            data: b"notdigits!".to_vec(),
        };

        assert!(matches!(
            parse_subscribe_request(&message),
            Err(HandlerError::InvalidData(100))
        ));
    }
}
