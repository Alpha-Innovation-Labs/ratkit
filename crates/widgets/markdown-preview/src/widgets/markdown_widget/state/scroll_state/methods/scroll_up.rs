//! Scroll up method for ScrollState.

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Scroll up by given number of lines.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of lines to scroll up.
    pub fn scroll_up(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = self.scroll_offset.saturating_sub(amount).min(max_offset);
    }
}
