use crate::protocol::field::FieldBuilder;
use crate::protocol::response_data::ResponseData;
use serde::{Deserialize, Serialize};

/// MID 0061 - Last tightening result data
///
/// Contains detailed information about a completed tightening operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TighteningResult {
    /// Cell ID (Parameter 01)
    pub cell_id: u32,

    /// Channel ID (Parameter 02)
    pub channel_id: u32,

    /// Torque Controller Name (Parameter 03)
    pub controller_name: String,

    /// VIN Number (Parameter 04) - Optional
    pub vin_number: Option<String>,

    /// Job ID (Parameter 05)
    pub job_id: u32,

    /// Parameter Set ID (Parameter 06)
    pub pset_id: u32,

    /// Batch Size (Parameter 07)
    pub batch_size: u32,

    /// Batch Counter (Parameter 08)
    pub batch_counter: u32,

    /// Tightening Status (Parameter 09) - OK=1, NOK=0
    pub tightening_status: bool,

    /// Torque Status (Parameter 10) - OK=1, NOK=0
    pub torque_status: bool,

    /// Angle Status (Parameter 11) - OK=1, NOK=0
    pub angle_status: bool,

    /// Torque Min Limit (Parameter 12) - in Nm
    pub torque_min: f64,

    /// Torque Max Limit (Parameter 13) - in Nm
    pub torque_max: f64,

    /// Torque Final Target (Parameter 14) - in Nm
    pub torque_target: f64,

    /// Torque (Parameter 15) - actual torque in Nm
    pub torque: f64,

    /// Angle Min (Parameter 16) - in degrees
    pub angle_min: f64,

    /// Angle Max (Parameter 17) - in degrees
    pub angle_max: f64,

    /// Angle Final Target (Parameter 18) - in degrees
    pub angle_target: f64,

    /// Angle (Parameter 19) - actual angle in degrees
    pub angle: f64,

    /// Timestamp (Parameter 20) - format: YYYY-MM-DD:HH:MM:SS
    pub timestamp: String,

    /// Last Change in Parameter Set (Parameter 21) - format: YYYY-MM-DD:HH:MM:SS
    pub last_pset_change: Option<String>,

    /// Batch Status (Parameter 22) - OK=1, NOK=0
    pub batch_status: Option<bool>,

    /// Tightening ID (Parameter 23)
    pub tightening_id: Option<u32>,

    /// Strategy (Revision 2 Parameter 07)
    pub strategy: Option<u8>,

    /// Strategy options bit field (Revision 2 Parameter 08)
    pub strategy_options: Option<u32>,

    /// Rundown angle status (Revision 2 Parameter 15)
    pub rundown_angle_status: Option<u8>,

    /// Current monitoring status (Revision 2 Parameter 16)
    pub current_monitoring_status: Option<u8>,

    /// Self-tap status (Revision 2 Parameter 17)
    pub self_tap_status: Option<u8>,

    /// Prevail torque monitoring status (Revision 2 Parameter 18)
    pub prevail_torque_monitoring_status: Option<u8>,

    /// Prevail torque compensate status (Revision 2 Parameter 19)
    pub prevail_torque_compensate_status: Option<u8>,

    /// Tightening error status bit field (Revision 2 Parameter 20)
    pub tightening_error_status: Option<u64>,

    /// Rundown angle min (Revision 2 Parameter 29)
    pub rundown_angle_min: Option<f64>,

    /// Rundown angle max (Revision 2 Parameter 30)
    pub rundown_angle_max: Option<f64>,

    /// Rundown angle actual (Revision 2 Parameter 31)
    pub rundown_angle: Option<f64>,

    /// Current monitoring min (Revision 2 Parameter 32)
    pub current_monitoring_min: Option<u16>,

    /// Current monitoring max (Revision 2 Parameter 33)
    pub current_monitoring_max: Option<u16>,

    /// Current monitoring value (Revision 2 Parameter 34)
    pub current_monitoring_value: Option<u16>,

    /// Self-tap min torque (Revision 2 Parameter 35)
    pub self_tap_min: Option<f64>,

    /// Self-tap max torque (Revision 2 Parameter 36)
    pub self_tap_max: Option<f64>,

    /// Self-tap torque (Revision 2 Parameter 37)
    pub self_tap_torque: Option<f64>,

    /// Prevail torque min (Revision 2 Parameter 38)
    pub prevail_torque_min: Option<f64>,

    /// Prevail torque max (Revision 2 Parameter 39)
    pub prevail_torque_max: Option<f64>,

    /// Prevail torque actual (Revision 2 Parameter 40)
    pub prevail_torque: Option<f64>,

    /// Job sequence number (Revision 2 Parameter 42)
    pub job_sequence_number: Option<u32>,

    /// Sync tightening ID (Revision 2 Parameter 43)
    pub sync_tightening_id: Option<u32>,

    /// Tool serial number (Revision 2 Parameter 44)
    pub tool_serial_number: Option<String>,

    /// Parameter set name (Revision 3 Parameter 47)
    pub pset_name: Option<String>,

    /// Torque values unit (Revision 3 Parameter 48)
    pub torque_unit: Option<u8>,

    /// Result type (Revision 3 Parameter 49)
    pub result_type: Option<u8>,
}

impl TighteningResult {
    /// Create a new tightening result with example values
    #[allow(dead_code)]
    pub fn example() -> Self {
        Self {
            cell_id: 1,
            channel_id: 1,
            controller_name: "Simulator".to_string(),
            vin_number: Some("TEST123456789".to_string()),
            job_id: 1,
            pset_id: 1,
            batch_size: 10,
            batch_counter: 5,
            tightening_status: true,
            torque_status: true,
            angle_status: true,
            torque_min: 10.0,
            torque_max: 15.0,
            torque_target: 12.5,
            torque: 12.3,
            angle_min: 30.0,
            angle_max: 50.0,
            angle_target: 40.0,
            angle: 39.5,
            timestamp: "2025-01-15:10:30:45".to_string(),
            last_pset_change: Some("2025-01-15:09:00:00".to_string()),
            batch_status: Some(true),
            tightening_id: Some(12345),
            strategy: Some(1),
            strategy_options: Some(0),
            rundown_angle_status: Some(1),
            current_monitoring_status: Some(1),
            self_tap_status: Some(1),
            prevail_torque_monitoring_status: Some(1),
            prevail_torque_compensate_status: Some(1),
            tightening_error_status: Some(0),
            rundown_angle_min: Some(10.0),
            rundown_angle_max: Some(20.0),
            rundown_angle: Some(15.0),
            current_monitoring_min: Some(10),
            current_monitoring_max: Some(90),
            current_monitoring_value: Some(50),
            self_tap_min: Some(1.0),
            self_tap_max: Some(2.0),
            self_tap_torque: Some(1.5),
            prevail_torque_min: Some(2.0),
            prevail_torque_max: Some(3.0),
            prevail_torque: Some(2.5),
            job_sequence_number: Some(12),
            sync_tightening_id: Some(7),
            tool_serial_number: Some("SIM-TOOL-0001".to_string()),
            pset_name: Some("Default".to_string()),
            torque_unit: Some(1),
            result_type: Some(1),
        }
    }

    pub fn serialize_for_revision(&self, revision: u8) -> Vec<u8> {
        match revision {
            2 => self.serialize_revision_2(),
            3 => self.serialize_revision_3(),
            _ => self.serialize_revision_1(),
        }
    }

    fn status_code(status: bool) -> i32 {
        if status { 1 } else { 0 }
    }

    fn batch_status_value(&self) -> i32 {
        match self.batch_status {
            Some(true) => 1,
            Some(false) => 0,
            None => 2,
        }
    }

    fn serialize_revision_1(&self) -> Vec<u8> {
        let vin = self.vin_number.as_deref().unwrap_or("");
        let pset_change = self.last_pset_change.as_deref().unwrap_or("");
        let tightening_id = self.tightening_id.unwrap_or(0);

        FieldBuilder::new()
            .add_int(Some(1), self.cell_id as i32, 4)
            .add_int(Some(2), self.channel_id as i32, 2)
            .add_str(Some(3), &self.controller_name, 25)
            .add_str(Some(4), vin, 25)
            .add_int(Some(5), self.job_id as i32, 2)
            .add_int(Some(6), self.pset_id as i32, 3)
            .add_int(Some(7), self.batch_size as i32, 4)
            .add_int(Some(8), self.batch_counter as i32, 4)
            .add_int(Some(9), Self::status_code(self.tightening_status), 1)
            .add_int(Some(10), Self::status_code(self.torque_status), 1)
            .add_int(Some(11), Self::status_code(self.angle_status), 1)
            .add_int(Some(12), (self.torque_min * 100.0) as i32, 6)
            .add_int(Some(13), (self.torque_max * 100.0) as i32, 6)
            .add_int(Some(14), (self.torque_target * 100.0) as i32, 6)
            .add_int(Some(15), (self.torque * 100.0) as i32, 6)
            .add_int(Some(16), self.angle_min as i32, 5)
            .add_int(Some(17), self.angle_max as i32, 5)
            .add_int(Some(18), self.angle_target as i32, 5)
            .add_int(Some(19), self.angle as i32, 5)
            .add_str(Some(20), &self.timestamp, 19)
            .add_str(Some(21), pset_change, 19)
            .add_int(Some(22), self.batch_status_value(), 1)
            .add_int(Some(23), tightening_id as i32, 10)
            .build()
    }

    fn serialize_revision_2(&self) -> Vec<u8> {
        let vin = self.vin_number.as_deref().unwrap_or("");
        let pset_change = self.last_pset_change.as_deref().unwrap_or("");

        FieldBuilder::new()
            .add_int(Some(1), self.cell_id as i32, 4)
            .add_int(Some(2), self.channel_id as i32, 2)
            .add_str(Some(3), &self.controller_name, 25)
            .add_str(Some(4), vin, 25)
            .add_int(Some(5), self.job_id as i32, 4)
            .add_int(Some(6), self.pset_id as i32, 3)
            .add_int(Some(7), self.strategy.unwrap_or(1) as i32, 2)
            .add_int(Some(8), self.strategy_options.unwrap_or(0) as i32, 5)
            .add_int(Some(9), self.batch_size as i32, 4)
            .add_int(Some(10), self.batch_counter as i32, 4)
            .add_int(Some(11), Self::status_code(self.tightening_status), 1)
            .add_int(Some(12), self.batch_status_value(), 1)
            .add_int(Some(13), Self::status_code(self.torque_status), 1)
            .add_int(Some(14), Self::status_code(self.angle_status), 1)
            .add_int(Some(15), self.rundown_angle_status.unwrap_or(1) as i32, 1)
            .add_int(
                Some(16),
                self.current_monitoring_status.unwrap_or(1) as i32,
                1,
            )
            .add_int(Some(17), self.self_tap_status.unwrap_or(1) as i32, 1)
            .add_int(
                Some(18),
                self.prevail_torque_monitoring_status.unwrap_or(1) as i32,
                1,
            )
            .add_int(
                Some(19),
                self.prevail_torque_compensate_status.unwrap_or(1) as i32,
                1,
            )
            .add_int(
                Some(20),
                self.tightening_error_status.unwrap_or(0) as i32,
                10,
            )
            .add_int(Some(21), (self.torque_min * 100.0) as i32, 6)
            .add_int(Some(22), (self.torque_max * 100.0) as i32, 6)
            .add_int(Some(23), (self.torque_target * 100.0) as i32, 6)
            .add_int(Some(24), (self.torque * 100.0) as i32, 6)
            .add_int(Some(25), self.angle_min as i32, 5)
            .add_int(Some(26), self.angle_max as i32, 5)
            .add_int(Some(27), self.angle_target as i32, 5)
            .add_int(Some(28), self.angle as i32, 5)
            .add_int(
                Some(29),
                self.rundown_angle_min.unwrap_or(0.0) as i32,
                5,
            )
            .add_int(
                Some(30),
                self.rundown_angle_max.unwrap_or(0.0) as i32,
                5,
            )
            .add_int(Some(31), self.rundown_angle.unwrap_or(0.0) as i32, 5)
            .add_int(
                Some(32),
                self.current_monitoring_min.unwrap_or(0) as i32,
                3,
            )
            .add_int(
                Some(33),
                self.current_monitoring_max.unwrap_or(0) as i32,
                3,
            )
            .add_int(
                Some(34),
                self.current_monitoring_value.unwrap_or(0) as i32,
                3,
            )
            .add_int(Some(35), (self.self_tap_min.unwrap_or(0.0) * 100.0) as i32, 6)
            .add_int(Some(36), (self.self_tap_max.unwrap_or(0.0) * 100.0) as i32, 6)
            .add_int(
                Some(37),
                (self.self_tap_torque.unwrap_or(0.0) * 100.0) as i32,
                6,
            )
            .add_int(
                Some(38),
                (self.prevail_torque_min.unwrap_or(0.0) * 100.0) as i32,
                6,
            )
            .add_int(
                Some(39),
                (self.prevail_torque_max.unwrap_or(0.0) * 100.0) as i32,
                6,
            )
            .add_int(
                Some(40),
                (self.prevail_torque.unwrap_or(0.0) * 100.0) as i32,
                6,
            )
            .add_int(Some(41), self.tightening_id.unwrap_or(0) as i32, 10)
            .add_int(Some(42), self.job_sequence_number.unwrap_or(0) as i32, 5)
            .add_int(Some(43), self.sync_tightening_id.unwrap_or(0) as i32, 5)
            .add_str(
                Some(44),
                self.tool_serial_number.as_deref().unwrap_or(""),
                14,
            )
            .add_str(Some(45), &self.timestamp, 19)
            .add_str(Some(46), pset_change, 19)
            .build()
    }

    fn serialize_revision_3(&self) -> Vec<u8> {
        let mut data = self.serialize_revision_2();
        data.extend_from_slice(
            &FieldBuilder::new()
                .add_str(Some(47), self.pset_name.as_deref().unwrap_or(""), 25)
                .add_int(Some(48), self.torque_unit.unwrap_or(1) as i32, 1)
                .add_int(Some(49), self.result_type.unwrap_or(1) as i32, 2)
                .build(),
        );
        data
    }
}

impl ResponseData for TighteningResult {
    fn serialize(&self) -> Vec<u8> {
        self.serialize_for_revision(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tightening_result_serialization() {
        let result = TighteningResult::example();
        let data = ResponseData::serialize(&result);

        // Should contain multiple parameters
        assert!(!data.is_empty());
        assert_eq!(data.len(), 211);
    }

    #[test]
    fn test_tightening_result_revision_2_serialization() {
        let result = TighteningResult::example();
        let data = result.serialize_for_revision(2);

        assert_eq!(data.len(), 365);
        let data_str = String::from_utf8_lossy(&data);
        assert!(data_str.contains("050001"));
        assert!(!data_str.contains("47Default"));
    }

    #[test]
    fn test_tightening_result_revision_3_serialization() {
        let result = TighteningResult::example();
        let data = result.serialize_for_revision(3);

        assert_eq!(data.len(), 399);
        let data_str = String::from_utf8_lossy(&data);
        assert!(data_str.contains("47Default"));
        assert!(data_str.contains("481"));
        assert!(data_str.contains("4901"));
    }
}
