use crate::protocol::field::FieldBuilder;
use crate::protocol::response_data::ResponseData;

/// MID 0217 - Relay function status
pub struct RelayFunction {
    pub relay_number: u16,
    pub active: bool,
}

impl RelayFunction {
    pub fn new(relay_number: u16, active: bool) -> Self {
        Self {
            relay_number,
            active,
        }
    }
}

impl ResponseData for RelayFunction {
    fn serialize(&self) -> Vec<u8> {
        FieldBuilder::new()
            .add_int(Some(1), self.relay_number as i32, 3)
            .add_int(Some(2), i32::from(self.active), 1)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_function_serialization() {
        let data = RelayFunction::new(22, true).serialize();
        assert_eq!(&data, b"01022021");
    }
}
