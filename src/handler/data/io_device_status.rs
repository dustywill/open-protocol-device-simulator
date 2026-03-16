use crate::protocol::field::FieldBuilder;
use crate::protocol::response_data::ResponseData;

pub struct IoDeviceStatus {
    pub device_number: u8,
    pub relays: Vec<(u16, bool)>,
    pub digital_inputs: Vec<(u16, bool)>,
}

impl IoDeviceStatus {
    pub fn new(device_number: u8, relays: Vec<(u16, bool)>, digital_inputs: Vec<(u16, bool)>) -> Self {
        Self {
            device_number,
            relays,
            digital_inputs,
        }
    }
}

impl ResponseData for IoDeviceStatus {
    fn serialize(&self) -> Vec<u8> {
        let relay_list = self
            .relays
            .iter()
            .map(|(number, active)| format!("{number:03}{}", if *active { 1 } else { 0 }))
            .collect::<String>();
        let digital_input_list = self
            .digital_inputs
            .iter()
            .map(|(number, active)| format!("{number:03}{}", if *active { 1 } else { 0 }))
            .collect::<String>();

        FieldBuilder::new()
            .add_int(Some(1), self.device_number as i32, 2)
            .add_str(Some(2), relay_list, 32)
            .add_str(Some(3), digital_input_list, 32)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_device_status_serialization() {
        let data = IoDeviceStatus::new(
            0,
            vec![(20, false), (22, true), (0, false), (0, false), (0, false), (0, false), (0, false), (0, false)],
            vec![(0, false); 8],
        )
        .serialize();

        let text = String::from_utf8(data).unwrap();
        assert!(text.starts_with("010002"));
        assert!(text.contains("0200"));
        assert!(text.contains("0221"));
        assert_eq!(text.len(), 72);
    }
}
