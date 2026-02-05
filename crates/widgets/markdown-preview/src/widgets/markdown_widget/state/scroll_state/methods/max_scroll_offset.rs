//! Max scroll offset method for ScrollState.

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Get the maximum valid scroll offset.
    ///
    /// # Returns
    ///
    /// The maximum scroll offset that keeps content visible.
    pub fn max_scroll_offset(&self) -> usize {
        self.total_lines.saturating_sub(self.viewport_height)
    }
}
