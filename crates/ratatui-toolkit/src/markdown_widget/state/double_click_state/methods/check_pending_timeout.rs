//! Check pending timeout method for DoubleClickState.

use std::time::Duration;

use crate::markdown_widget::state::double_click_state::DoubleClickState;

impl DoubleClickState {
    /// Check if there's a pending single-click that has timed out (no double-click came).
    ///
    /// Call this periodically (e.g., each frame) to get pending clicks.
    ///
    /// # Returns
    ///
    /// The position and scroll_offset of the pending click if it should be processed.
    /// Returns `(x, y, scroll_offset_at_click_time)`.
    pub fn check_pending_timeout(&mut self) -> Option<(u16, u16, usize)> {
        if let Some((x, y, time, scroll_offset)) = self.pending_single_click {
            let elapsed = time.elapsed();
            if elapsed >= Duration::from_millis(Self::DOUBLE_CLICK_THRESHOLD_MS) {
                // Timeout expired, process the pending single click
                self.pending_single_click = None;
                return Some((x, y, scroll_offset));
            }
        }
        None
    }
}
