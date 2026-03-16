use crate::protocol::field::FieldBuilder;
use crate::protocol::response_data::ResponseData;

/// MID 0015 - Parameter Set Selected
///
/// Notification sent when a parameter set is selected
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PsetSelected {
    /// Parameter Set ID that was selected
    pub pset_id: u32,
    /// Date/time of last change in parameter set settings.
    pub last_change_timestamp: String,
}

impl PsetSelected {
    #[allow(dead_code)]
    pub fn new(pset_id: u32, last_change_timestamp: String) -> Self {
        Self {
            pset_id,
            last_change_timestamp,
        }
    }
}

impl ResponseData for PsetSelected {
    fn serialize(&self) -> Vec<u8> {
        // Revision 1 format:
        // Parameter set ID (3 bytes) + last change timestamp (19 bytes)
        let builder = FieldBuilder::new()
            .add_int(None, self.pset_id as i32, 3)
            .add_str(None, &self.last_change_timestamp, 19);
        builder.build()
    }
}

impl Default for PsetSelected {
    fn default() -> Self {
        Self::new(1, "1970-01-01:00:00:00".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pset_selected_serialization() {
        let pset = PsetSelected::new(5, "2026-03-16:13:34:11".to_string());
        let data = pset.serialize();
        assert_eq!(&data[..], b"0052026-03-16:13:34:11");
        assert_eq!(data.len(), 22);
    }

    #[test]
    fn test_pset_selected_large_id() {
        let pset = PsetSelected::new(123, "2026-03-16:13:34:11".to_string());
        let data = pset.serialize();
        assert_eq!(&data[..], b"1232026-03-16:13:34:11");
    }
}
