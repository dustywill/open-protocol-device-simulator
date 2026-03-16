use crate::config::DeviceConfig;
use crate::device_fsm::DeviceFSMState;
use crate::failure_simulator::FailureConfig;
use crate::multi_spindle::{MultiSpindleConfig, MultiSpindleResultRecord};
use crate::tightening_tracker::TighteningTracker;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

const MULTI_SPINDLE_HISTORY_LIMIT: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ToolDirection {
    Cw,
    Ccw,
}

/// Represents the internal state of the simulated device
#[derive(Debug, Clone, Serialize)]
pub struct DeviceState {
    // Controller identification
    pub cell_id: u32,
    pub channel_id: u32,
    pub controller_name: String,
    pub supplier_code: String,

    // Parameter set (pset) state
    pub current_pset_id: Option<u32>,
    pub current_pset_name: Option<String>,
    #[serde(skip)]
    pub current_pset_last_change: String,

    // Tightening tracking (single mode or batch mode)
    pub tightening_tracker: TighteningTracker,

    // Device operational state
    pub device_fsm_state: DeviceFSMState,

    // Tool state
    pub tool_enabled: bool,
    pub tool_direction: ToolDirection,
    pub tool_start_switch_active: bool,

    // Vehicle/Job identification
    pub vehicle_id: Option<String>,
    pub current_job_id: Option<u32>,

    // Multi-spindle configuration
    pub multi_spindle_config: MultiSpindleConfig,

    // Recent multi-spindle results used for MID 0100 replay.
    #[serde(skip)]
    pub multi_spindle_result_history: VecDeque<MultiSpindleResultRecord>,

    #[serde(skip)]
    controller_time_offset_seconds: i64,

    // Communication failure injection configuration
    pub failure_config: FailureConfig,
}

impl DeviceState {
    /// Create a new device state with default values
    pub fn new() -> Self {
        Self {
            cell_id: 1,
            channel_id: 1,
            controller_name: "OpenProtocolSimulator".to_string(),
            supplier_code: "SIM".to_string(),
            current_pset_id: Some(1),
            current_pset_name: Some("Default".to_string()),
            current_pset_last_change: Self::current_timestamp_for_offset(0),
            tightening_tracker: TighteningTracker::new(),
            device_fsm_state: DeviceFSMState::idle(),
            tool_enabled: true,
            tool_direction: ToolDirection::Cw,
            tool_start_switch_active: false,
            vehicle_id: None,
            current_job_id: Some(1),
            multi_spindle_config: MultiSpindleConfig::default(),
            multi_spindle_result_history: VecDeque::new(),
            controller_time_offset_seconds: 0,
            failure_config: FailureConfig::default(),
        }
    }

    /// Create a new device state from configuration
    pub fn new_from_config(config: &DeviceConfig) -> Self {
        Self {
            cell_id: config.cell_id,
            channel_id: config.channel_id,
            controller_name: config.controller_name.clone(),
            supplier_code: config.supplier_code.clone(),
            current_pset_id: Some(1),
            current_pset_name: Some("Default".to_string()),
            current_pset_last_change: Self::current_timestamp_for_offset(0),
            tightening_tracker: TighteningTracker::new(),
            device_fsm_state: DeviceFSMState::idle(),
            tool_enabled: true,
            tool_direction: ToolDirection::Cw,
            tool_start_switch_active: false,
            vehicle_id: None,
            current_job_id: Some(1),
            multi_spindle_config: MultiSpindleConfig::default(),
            multi_spindle_result_history: VecDeque::new(),
            controller_time_offset_seconds: 0,
            failure_config: FailureConfig::default(),
        }
    }

    /// Create a thread-safe shared state
    pub fn new_shared() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new()))
    }

    /// Create a thread-safe shared state from configuration
    pub fn new_shared_from_config(config: &DeviceConfig) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new_from_config(config)))
    }

    /// Set the parameter set
    pub fn set_pset(&mut self, pset_id: u32, pset_name: Option<String>) {
        self.current_pset_id = Some(pset_id);
        self.current_pset_name = pset_name;
        self.current_pset_last_change = self.current_timestamp();
    }

    /// Set the active job ID
    pub fn set_job_id(&mut self, job_id: u32) {
        self.current_job_id = Some(job_id);
    }
    /// Set batch size (enables batch mode)
    pub fn set_batch_size(&mut self, size: u32) {
        self.tightening_tracker.enable_batch(size);
    }

    /// Increment batch counter without tightening (MID 0128 - skip bolt)
    pub fn increment_batch(&mut self) -> u32 {
        self.tightening_tracker.increment_batch()
    }

    /// Reset batch counter (MID 0020)
    /// Returns true if in batch mode, false otherwise
    pub fn reset_batch(&mut self) -> bool {
        self.tightening_tracker.reset_batch()
    }

    /// Enable the tool
    pub fn enable_tool(&mut self) {
        self.tool_enabled = true;
    }

    /// Disable the tool
    pub fn disable_tool(&mut self) {
        self.tool_enabled = false;
    }

    /// Set tool direction (maps to the relay state used by external clients).
    pub fn set_tool_direction(&mut self, direction: ToolDirection) -> bool {
        if self.tool_direction == direction {
            return false;
        }
        self.tool_direction = direction;
        true
    }

    /// Relay 22 is active only when the tool direction is CCW.
    pub fn direction_ccw_relay_active(&self) -> bool {
        matches!(self.tool_direction, ToolDirection::Ccw)
    }

    pub fn set_tool_start_switch_active(&mut self, active: bool) -> bool {
        if self.tool_start_switch_active == active {
            return false;
        }
        self.tool_start_switch_active = active;
        true
    }

    pub fn set_controller_time(&mut self, timestamp: NaiveDateTime) {
        self.controller_time_offset_seconds =
            timestamp
                .signed_duration_since(Local::now().naive_local())
                .num_seconds();
        self.current_pset_last_change = self.current_timestamp();
    }

    pub fn current_protocol_timestamp(&self) -> String {
        self.current_timestamp()
    }

    /// Set vehicle ID
    pub fn set_vehicle_id(&mut self, vin: String) {
        self.vehicle_id = Some(vin);
    }

    /// Clear vehicle ID
    #[allow(dead_code)]
    pub fn clear_vehicle_id(&mut self) {
        self.vehicle_id = None;
    }

    /// Enable multi-spindle mode
    pub fn enable_multi_spindle(&mut self, spindle_count: u8, sync_id: u32) -> Result<(), String> {
        let config = MultiSpindleConfig::new(spindle_count, sync_id);
        if !config.is_valid() {
            return Err(format!(
                "Invalid multi-spindle configuration: spindle_count must be 2-16, got {}",
                spindle_count
            ));
        }
        self.multi_spindle_config = config;
        Ok(())
    }

    /// Disable multi-spindle mode (revert to single-spindle)
    pub fn disable_multi_spindle(&mut self) {
        self.multi_spindle_config = MultiSpindleConfig::disable();
    }

    pub fn record_multi_spindle_result(&mut self, record: MultiSpindleResultRecord) {
        self.multi_spindle_result_history.push_back(record);
        while self.multi_spindle_result_history.len() > MULTI_SPINDLE_HISTORY_LIMIT {
            self.multi_spindle_result_history.pop_front();
        }
    }

    pub fn latest_multi_spindle_result_id(&self) -> Option<u32> {
        self.multi_spindle_result_history
            .back()
            .map(MultiSpindleResultRecord::result_id)
    }

    pub fn replay_multi_spindle_results(
        &self,
        data_no_system: Option<u32>,
    ) -> Vec<MultiSpindleResultRecord> {
        let Some(after_result_id) = data_no_system.filter(|value| *value != 0) else {
            return self.multi_spindle_result_history.iter().cloned().collect();
        };

        let found = self
            .multi_spindle_result_history
            .iter()
            .any(|record| record.result_id() == after_result_id);
        if !found {
            return self.multi_spindle_result_history.iter().cloned().collect();
        }

        self.multi_spindle_result_history
            .iter()
            .filter(|record| record.result_id() > after_result_id)
            .cloned()
            .collect()
    }

    /// Check if multi-spindle mode is enabled
    ///
    /// Query method for checking multi-spindle state.
    /// Used by webUI dashboard to display mode and by HTTP API endpoints
    /// for status reporting and configuration validation.
    #[allow(dead_code)]
    pub fn is_multi_spindle_enabled(&self) -> bool {
        self.multi_spindle_config.enabled
    }

    /// Get multi-spindle configuration
    ///
    /// Query method for accessing multi-spindle settings.
    /// Used by webUI configuration panel to display and edit spindle
    /// count and sync ID settings, and by HTTP API for configuration export.
    #[allow(dead_code)]
    pub fn get_multi_spindle_config(&self) -> &MultiSpindleConfig {
        &self.multi_spindle_config
    }

    fn current_timestamp(&self) -> String {
        Self::current_timestamp_for_offset(self.controller_time_offset_seconds)
    }

    fn current_timestamp_for_offset(offset_seconds: i64) -> String {
        (Local::now() + chrono::Duration::seconds(offset_seconds))
            .format("%Y-%m-%d:%H:%M:%S")
            .to_string()
    }
}

impl Default for DeviceState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_state_creation() {
        let state = DeviceState::new();
        assert_eq!(state.cell_id, 1);
        assert!(state.tool_enabled);
        assert_eq!(state.tightening_tracker.counter(), 0);
    }

    #[test]
    fn test_tightening_tracker() {
        let mut state = DeviceState::new();
        // In single mode, counter stays 0
        let info = state.tightening_tracker.add_tightening(true);
        assert_eq!(info.counter, 0);

        // Enable batch mode
        state.set_batch_size(2);
        let info = state.tightening_tracker.add_tightening(true);
        assert_eq!(info.counter, 1);
    }

    #[test]
    fn test_tool_state() {
        let mut state = DeviceState::new();
        state.disable_tool();
        assert!(!state.tool_enabled);
        state.enable_tool();
        assert!(state.tool_enabled);
        assert_eq!(state.tool_direction, ToolDirection::Cw);
    }

    #[test]
    fn test_tool_direction_state() {
        let mut state = DeviceState::new();
        assert!(!state.direction_ccw_relay_active());

        assert!(state.set_tool_direction(ToolDirection::Ccw));
        assert_eq!(state.tool_direction, ToolDirection::Ccw);
        assert!(state.direction_ccw_relay_active());
        assert!(!state.set_tool_direction(ToolDirection::Ccw));
    }

    #[test]
    fn test_shared_state() {
        let state = DeviceState::new_shared();
        {
            let mut s = state.write().unwrap();
            s.set_pset(5, Some("Test".to_string()));
        }
        {
            let s = state.read().unwrap();
            assert_eq!(s.current_pset_id, Some(5));
        }
    }

    #[test]
    fn test_multi_spindle_result_history_replay() {
        use crate::multi_spindle::{MultiSpindleResult, MultiSpindleResultRecord, SpindleResult};

        let mut state = DeviceState::new();
        for result_id in 1..=3 {
            state.record_multi_spindle_result(MultiSpindleResultRecord {
                result: MultiSpindleResult::new(
                    result_id,
                    100,
                    vec![SpindleResult::ok(1, 5000, 1800), SpindleResult::ok(2, 5000, 1800)],
                ),
                vin_number: "VIN".to_string(),
                job_id: 1,
                pset_id: 1,
                batch_size: 0,
                batch_counter: result_id,
                batch_status: 2,
                torque_min: 4500,
                torque_max: 5500,
                torque_target: 5000,
                angle_min: 170,
                angle_max: 190,
                angle_target: 180,
                last_change_timestamp: "2026-01-01:00:00:00".to_string(),
            });
        }

        let replay = state.replay_multi_spindle_results(Some(2));
        assert_eq!(replay.len(), 1);
        assert_eq!(replay[0].result_id(), 3);

        let missing_anchor_replay = state.replay_multi_spindle_results(Some(99));
        assert_eq!(missing_anchor_replay.len(), 3);
    }
}
