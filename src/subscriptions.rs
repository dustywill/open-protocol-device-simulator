use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct MultiSpindleResultSubscription {
    pub revision: u8,
    pub data_no_system: Option<u32>,
    pub send_only_new_data: bool,
    #[serde(skip)]
    pub future_only_after_result_id: Option<u32>,
}

impl MultiSpindleResultSubscription {
    pub fn new(
        revision: u8,
        data_no_system: Option<u32>,
        send_only_new_data: bool,
        latest_result_id: Option<u32>,
    ) -> Self {
        Self {
            revision,
            data_no_system,
            send_only_new_data,
            future_only_after_result_id: if send_only_new_data {
                latest_result_id
            } else {
                None
            },
        }
    }

    pub fn should_send_live_result(&self, result_id: u32) -> bool {
        self.future_only_after_result_id
            .is_none_or(|latest| result_id > latest)
    }
}

/// Manages client subscription state for various event types
#[derive(Debug, Clone, Default, Serialize)]
pub struct Subscriptions {
    /// Subscribed to tightening result events (MID 0061)
    pub tightening_result: Option<u8>,

    /// Subscribed to parameter set selection events (MID 0015)
    pub pset_selection: bool,

    /// Subscribed to vehicle ID events (MID 0052)
    pub vehicle_id: bool,

    /// Subscribed to multi-spindle status events (MID 0091)
    pub multi_spindle_status: bool,

    /// Subscribed to multi-spindle result events (MID 0101)
    pub multi_spindle_result: Option<MultiSpindleResultSubscription>,

    /// Subscribed to alarm events (not yet implemented)
    pub alarm: bool,

    /// Subscribed to job info events (not yet implemented)
    pub job_info: bool,
}

impl Subscriptions {
    /// Create a new subscription manager with all subscriptions disabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Subscribe to tightening result events
    pub fn subscribe_tightening_result(&mut self, revision: u8) {
        self.tightening_result = Some(revision);
    }

    /// Unsubscribe from tightening result events
    pub fn unsubscribe_tightening_result(&mut self) {
        self.tightening_result = None;
    }

    /// Subscribe to parameter set selection events
    pub fn subscribe_pset_selection(&mut self) {
        self.pset_selection = true;
    }

    /// Unsubscribe from parameter set selection events
    pub fn unsubscribe_pset_selection(&mut self) {
        self.pset_selection = false;
    }

    /// Check if subscribed to tightening results
    pub fn is_subscribed_to_tightening_result(&self) -> bool {
        self.tightening_result.is_some()
    }

    /// Get subscribed tightening result revision
    pub fn tightening_result_revision(&self) -> Option<u8> {
        self.tightening_result
    }

    /// Check if subscribed to pset selection
    pub fn is_subscribed_to_pset_selection(&self) -> bool {
        self.pset_selection
    }

    /// Subscribe to vehicle ID events
    pub fn subscribe_vehicle_id(&mut self) {
        self.vehicle_id = true;
    }

    /// Unsubscribe from vehicle ID events
    pub fn unsubscribe_vehicle_id(&mut self) {
        self.vehicle_id = false;
    }

    /// Check if subscribed to vehicle ID
    pub fn is_subscribed_to_vehicle_id(&self) -> bool {
        self.vehicle_id
    }

    /// Subscribe to multi-spindle status events
    pub fn subscribe_multi_spindle_status(&mut self) {
        self.multi_spindle_status = true;
    }

    /// Unsubscribe from multi-spindle status events
    pub fn unsubscribe_multi_spindle_status(&mut self) {
        self.multi_spindle_status = false;
    }

    /// Check if subscribed to multi-spindle status
    pub fn is_subscribed_to_multi_spindle_status(&self) -> bool {
        self.multi_spindle_status
    }

    /// Subscribe to multi-spindle result events
    pub fn subscribe_multi_spindle_result(&mut self, subscription: MultiSpindleResultSubscription) {
        self.multi_spindle_result = Some(subscription);
    }

    /// Unsubscribe from multi-spindle result events
    pub fn unsubscribe_multi_spindle_result(&mut self) {
        self.multi_spindle_result = None;
    }

    /// Check if subscribed to multi-spindle result
    pub fn is_subscribed_to_multi_spindle_result(&self) -> bool {
        self.multi_spindle_result.is_some()
    }

    /// Get subscribed multi-spindle result revision
    pub fn multi_spindle_result_revision(&self) -> Option<u8> {
        self.multi_spindle_result.map(|subscription| subscription.revision)
    }

    /// Get the full multi-spindle result subscription state
    pub fn multi_spindle_result_subscription(&self) -> Option<MultiSpindleResultSubscription> {
        self.multi_spindle_result
    }

    /// Get count of active subscriptions
    ///
    /// Diagnostic method for subscription statistics.
    /// Used by webUI connection dashboard to display per-client
    /// subscription counts and by monitoring/metrics endpoints.
    #[allow(dead_code)]
    pub fn active_count(&self) -> usize {
        let mut count = 0;
        if self.tightening_result.is_some() {
            count += 1;
        }
        if self.pset_selection {
            count += 1;
        }
        if self.vehicle_id {
            count += 1;
        }
        if self.multi_spindle_status {
            count += 1;
        }
        if self.multi_spindle_result.is_some() {
            count += 1;
        }
        if self.alarm {
            count += 1;
        }
        if self.job_info {
            count += 1;
        }
        count
    }

    /// Check if any subscriptions are active
    ///
    /// Convenience method for subscription status checks.
    /// Used by connection lifecycle management to determine whether to
    /// keep idle connections alive, and by webUI for client status display.
    #[allow(dead_code)]
    pub fn has_any_subscription(&self) -> bool {
        self.active_count() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_no_subscriptions() {
        let subs = Subscriptions::new();
        assert!(!subs.is_subscribed_to_tightening_result());
        assert_eq!(subs.tightening_result_revision(), None);
        assert!(!subs.is_subscribed_to_pset_selection());
        assert_eq!(subs.active_count(), 0);
        assert!(!subs.has_any_subscription());
    }

    #[test]
    fn test_subscribe_tightening_result() {
        let mut subs = Subscriptions::new();
        subs.subscribe_tightening_result(3);

        assert!(subs.is_subscribed_to_tightening_result());
        assert_eq!(subs.tightening_result_revision(), Some(3));
        assert_eq!(subs.active_count(), 1);
        assert!(subs.has_any_subscription());
    }

    #[test]
    fn test_unsubscribe_tightening_result() {
        let mut subs = Subscriptions::new();
        subs.subscribe_tightening_result(1);
        subs.unsubscribe_tightening_result();

        assert!(!subs.is_subscribed_to_tightening_result());
        assert_eq!(subs.tightening_result_revision(), None);
        assert_eq!(subs.active_count(), 0);
    }

    #[test]
    fn test_multiple_subscriptions() {
        let mut subs = Subscriptions::new();
        subs.subscribe_tightening_result(2);
        subs.subscribe_pset_selection();

        assert!(subs.is_subscribed_to_tightening_result());
        assert!(subs.is_subscribed_to_pset_selection());
        assert_eq!(subs.active_count(), 2);
    }

    #[test]
    fn test_subscribe_idempotent() {
        let mut subs = Subscriptions::new();
        subs.subscribe_tightening_result(1);
        subs.subscribe_tightening_result(3);

        assert!(subs.is_subscribed_to_tightening_result());
        assert_eq!(subs.tightening_result_revision(), Some(3));
        assert_eq!(subs.active_count(), 1);
    }

    #[test]
    fn test_multi_spindle_result_subscription_tracks_replay_options() {
        let mut subs = Subscriptions::new();
        let subscription = MultiSpindleResultSubscription::new(3, Some(42), true, Some(100));
        subs.subscribe_multi_spindle_result(subscription);

        let stored = subs.multi_spindle_result_subscription().unwrap();
        assert_eq!(stored.revision, 3);
        assert_eq!(stored.data_no_system, Some(42));
        assert!(stored.send_only_new_data);
        assert_eq!(stored.future_only_after_result_id, Some(100));
        assert_eq!(subs.multi_spindle_result_revision(), Some(3));
    }
}
