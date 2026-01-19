//! Scroll down method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Scroll down by given number of lines.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of lines to scroll down.
    pub fn scroll_down(&mut self, amount: usize) {
        let max_offset = self.max_scroll_offset();
        self.scroll_offset = (self.scroll_offset + amount).min(max_offset);
    }
}
