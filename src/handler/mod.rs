pub mod batch_increment;
pub mod batch_reset;
pub mod batch_size;
pub mod communication_start;
pub mod communication_stop;
pub mod data;
pub mod job_select;
pub mod keep_alive;
pub mod multi_spindle_result_ack;
pub mod multi_spindle_result_subscribe;
pub mod multi_spindle_result_unsubscribe;
pub mod multi_spindle_status_ack;
pub mod multi_spindle_status_subscribe;
pub mod multi_spindle_status_unsubscribe;
pub mod pset_select;
pub mod pset_selected_ack;
pub mod pset_subscription;
pub mod pset_unsubscribe;
pub mod tightening_result_ack;
pub mod tightening_result_subscription;
pub mod tightening_result_unsubscribe;
pub mod tool_disable;
pub mod tool_enable;
pub mod vehicle_id_ack;
pub mod vehicle_id_download;
pub mod vehicle_id_subscription;
pub mod vehicle_id_unsubscribe;

use crate::observable_state::ObservableState;
use crate::protocol::{Message, Response};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Unknown MID: {0}")]
    UnknownMid(u16),

    #[error("Unsupported revision {revision} for MID {mid}")]
    RevisionUnsupported { mid: u16, revision: u8 },

    #[error("Invalid data for MID {0}")]
    InvalidData(u16),

    #[error("Handler error: {0}")]
    #[allow(dead_code)]
    Processing(String),
}

impl HandlerError {
    pub fn to_error_response(&self, request_revision: u8) -> Response {
        let error_data = match self {
            Self::UnknownMid(mid) => data::ErrorResponse::generic(*mid),
            Self::RevisionUnsupported { mid, .. } => {
                data::ErrorResponse::revision_unsupported(*mid)
            }
            Self::InvalidData(mid) => data::ErrorResponse::invalid_data(*mid),
            Self::Processing(_) => data::ErrorResponse::generic(0),
        };

        Response::from_data(4, request_revision, error_data)
    }
}

pub fn is_revision_supported(mid: u16, revision: u8) -> bool {
    match mid {
        38 => matches!(revision, 1 | 2),
        60 | 61 | 100 | 101 => matches!(revision, 1 | 2 | 3),
        _ => true,
    }
}

/// Trait for handling specific MID messages
pub trait MidHandler: Send + Sync {
    /// Process a message and generate a response
    fn handle(&self, message: &Message) -> Result<Response, HandlerError>;
}

/// Registry that routes MIDs to their handlers
pub struct HandlerRegistry {
    handlers: HashMap<u16, Box<dyn MidHandler>>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a handler for a specific MID
    pub fn register(&mut self, mid: u16, handler: Box<dyn MidHandler>) {
        self.handlers.insert(mid, handler);
    }

    /// Process a message using the appropriate handler
    pub fn handle_message(&self, message: &Message) -> Result<Response, HandlerError> {
        if !is_revision_supported(message.mid, message.revision) {
            return Err(HandlerError::RevisionUnsupported {
                mid: message.mid,
                revision: message.revision,
            });
        }

        let handler = self
            .handlers
            .get(&message.mid)
            .ok_or(HandlerError::UnknownMid(message.mid))?;

        handler.handle(message)
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a registry with all standard handlers registered
pub fn create_default_registry(observable_state: ObservableState) -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();
    let state = observable_state.state();

    // Register all MID handlers (sorted by MID number)
    registry.register(
        1,
        Box::new(communication_start::CommunicationStartHandler::new(
            Arc::clone(state),
        )),
    );
    registry.register(
        3,
        Box::new(communication_stop::CommunicationStopHandler::new()),
    );
    registry.register(14, Box::new(pset_subscription::PsetSubscriptionHandler));
    registry.register(16, Box::new(pset_selected_ack::PsetSelectedAckHandler));
    registry.register(17, Box::new(pset_unsubscribe::PsetUnsubscribeHandler));
    registry.register(
        18,
        Box::new(pset_select::PsetSelectHandler::new(
            observable_state.clone(),
        )),
    );
    registry.register(
        19,
        Box::new(batch_size::BatchSizeHandler::new(Arc::clone(state))),
    );
    registry.register(
        20,
        Box::new(batch_reset::BatchResetHandler::new(Arc::clone(state))),
    );
    registry.register(
        128,
        Box::new(batch_increment::BatchIncrementHandler::new(
            observable_state.clone(),
        )),
    );
    registry.register(
        38,
        Box::new(job_select::JobSelectHandler::new(Arc::clone(state))),
    );
    registry.register(
        42,
        Box::new(tool_disable::ToolDisableHandler::new(
            observable_state.clone(),
        )),
    );
    registry.register(
        43,
        Box::new(tool_enable::ToolEnableHandler::new(
            observable_state.clone(),
        )),
    );
    registry.register(
        50,
        Box::new(vehicle_id_download::VehicleIdDownloadHandler::new(
            observable_state.clone(),
        )),
    );
    registry.register(
        51,
        Box::new(vehicle_id_subscription::VehicleIdSubscriptionHandler),
    );
    registry.register(53, Box::new(vehicle_id_ack::VehicleIdAckHandler));
    registry.register(
        54,
        Box::new(vehicle_id_unsubscribe::VehicleIdUnsubscribeHandler),
    );
    registry.register(
        90,
        Box::new(multi_spindle_status_subscribe::MultiSpindleStatusSubscribeHandler),
    );
    registry.register(
        92,
        Box::new(multi_spindle_status_unsubscribe::MultiSpindleStatusUnsubscribeHandler),
    );
    registry.register(
        93,
        Box::new(multi_spindle_status_ack::MultiSpindleStatusAckHandler),
    );
    registry.register(
        100,
        Box::new(multi_spindle_result_subscribe::MultiSpindleResultSubscribeHandler),
    );
    registry.register(
        102,
        Box::new(multi_spindle_result_ack::MultiSpindleResultAckHandler),
    );
    registry.register(
        103,
        Box::new(multi_spindle_result_unsubscribe::MultiSpindleResultUnsubscribeHandler),
    );
    registry.register(
        60,
        Box::new(tightening_result_subscription::TighteningResultSubscriptionHandler),
    );
    registry.register(
        62,
        Box::new(tightening_result_ack::TighteningResultAckHandler),
    );
    registry.register(
        63,
        Box::new(tightening_result_unsubscribe::TighteningResultUnsubscribeHandler),
    );
    registry.register(9999, Box::new(keep_alive::KeepAliveHandler));

    registry
}

#[cfg(test)]
mod tests {
    use super::is_revision_supported;

    #[test]
    fn test_revision_support_policy() {
        assert!(is_revision_supported(38, 1));
        assert!(is_revision_supported(38, 2));
        assert!(!is_revision_supported(38, 3));
        assert!(is_revision_supported(60, 3));
        assert!(!is_revision_supported(60, 4));
        assert!(is_revision_supported(100, 2));
        assert!(!is_revision_supported(100, 4));
        assert!(is_revision_supported(42, 9));
    }
}
