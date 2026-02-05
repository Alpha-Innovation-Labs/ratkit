//! Process click method for DoubleClickState.

use std::time::{Duration, Instant};

use super::super::DoubleClickState;

impl DoubleClickState {
    /// Check if this click is a double-click and update state.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the click
    /// * `y` - Y coordinate of the click
    /// * `scroll_offset` - The scroll offset at the time of the click (for accurate line calculation later)
    ///
    /// # Returns
    ///
    /// `(is_double_click, should_process_pending_single)`.
    pub fn process_click(&mut self, x: u16, y: u16, scroll_offset: usize) -> (bool, bool) {
        let now = Instant::now();
        let double_click_threshold = Duration::from_millis(Self::DOUBLE_CLICK_THRESHOLD_MS);
        let position_threshold = 3; // Allow small movement between clicks

        // Check if this is a double-click (follows a recent click at same position)
        let is_double = if let (Some(last_time), Some((last_x, last_y))) =
            (self.last_click_time, self.last_click_pos)
        {
            let time_ok = now.duration_since(last_time) < double_click_threshold;
            let pos_ok = x.abs_diff(last_x) <= position_threshold
                && y.abs_diff(last_y) <= position_threshold;
            time_ok && pos_ok
        } else {
            false
        };

        if is_double {
            // Double-click detected - clear state, don't process pending single
            self.last_click_time = None;
            self.last_click_pos = None;
            self.pending_single_click = None;
            (true, false)
        } else {
            // Check if there's a pending single click that should now be processed
            // (because this new click is NOT at the same position, so the old one wasn't part of a double-click)
            let should_process_pending = self.pending_single_click.take().is_some();

            // Record this click with scroll_offset for accurate line calculation later
            self.last_click_time = Some(now);
            self.last_click_pos = Some((x, y));
            self.pending_single_click = Some((x, y, now, scroll_offset));

            (false, should_process_pending)
        }
    }
}
