//! MID-specific data structures with friendly names
//!
//! Each module defines typed data structures for specific MID responses,
//! implementing the ResponseData trait for automatic serialization.

pub mod command_accepted;
pub mod communication_start;
pub mod error_response;
pub mod io_device_status;
pub mod multi_spindle_result_broadcast;
pub mod multi_spindle_status_broadcast;
pub mod pset_selected;
pub mod relay_function;
pub mod tightening_result;
pub mod vehicle_id_broadcast;

pub use command_accepted::CommandAccepted;
pub use communication_start::CommunicationStartAck;
#[allow(unused_imports)]
pub use error_response::ErrorCode;
pub use error_response::ErrorResponse;
pub use io_device_status::IoDeviceStatus;
pub use multi_spindle_result_broadcast::MultiSpindleResultBroadcast;
pub use multi_spindle_status_broadcast::MultiSpindleStatusBroadcast;
#[allow(unused_imports)]
pub use pset_selected::PsetSelected;
pub use relay_function::RelayFunction;
pub use tightening_result::TighteningResult;
pub use vehicle_id_broadcast::VehicleIdBroadcast;
