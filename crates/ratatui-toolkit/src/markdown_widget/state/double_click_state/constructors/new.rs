//! Constructor for DoubleClickState.

use crate::markdown_widget::state::double_click_state::DoubleClickState;

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
