//! Scroll to bottom method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Move to bottom of document.
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.max_scroll_offset();
        self.current_line = self.total_lines;
    }
}
