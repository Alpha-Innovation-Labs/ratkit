//! State for tracking double-click detection with deferred single-click handling.

use std::time::Instant;

/// State for tracking double-click detection with deferred single-click handling.
#[derive(Debug, Clone, Default)]
pub struct DoubleClickState {
    /// Time of the last click.
    pub(crate) last_click_time: Option<Instant>,
    /// Position of the last click.
    pub(crate) last_click_pos: Option<(u16, u16)>,
    /// Pending single-click that hasn't been processed yet.
    /// Stores: (x, y, timestamp, scroll_offset_at_click_time)
    pub(crate) pending_single_click: Option<(u16, u16, Instant, usize)>,
}

impl DoubleClickState {
    /// Double-click time threshold in milliseconds.
    pub(crate) const DOUBLE_CLICK_THRESHOLD_MS: u64 = 150;
}

/// Constructor for DoubleClickState.

impl DoubleClickState {
    /// Create a new double-click state tracker.
    ///
    /// # Returns
    ///
    /// A new `DoubleClickState` with no pending clicks.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Check pending timeout method for DoubleClickState.
use std::time::Duration;

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

/// Clear pending method for DoubleClickState.

impl DoubleClickState {
    /// Clear any pending click (e.g., when double-click is detected).
    pub fn clear_pending(&mut self) {
        self.pending_single_click = None;
    }
}

/// Process click method for DoubleClickState.

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
