//! Scroll to top method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Move to top of document.
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.current_line = 1;
    }
}
