//! Update total lines method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Update total line count.
    ///
    /// # Arguments
    ///
    /// * `total` - The total number of lines in the document.
    pub fn update_total_lines(&mut self, total: usize) {
        self.total_lines = total.max(1);
    }
}
