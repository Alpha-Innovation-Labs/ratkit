//! Scroll percentage method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Calculate percentage scrolled (0.0 to 1.0).
    ///
    /// # Returns
    ///
    /// The scroll position as a percentage of total scrollable content.
    pub fn scroll_percentage(&self) -> f64 {
        let max_offset = self.max_scroll_offset();
        if max_offset == 0 {
            0.0
        } else {
            self.scroll_offset as f64 / max_offset as f64
        }
    }
}
