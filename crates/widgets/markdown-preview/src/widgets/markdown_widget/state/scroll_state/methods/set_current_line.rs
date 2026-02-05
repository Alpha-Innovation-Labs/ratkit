//! Set current line method for ScrollState.

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Set current line and adjust scroll to keep it visible.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number to set as current (1-indexed).
    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line.clamp(1, self.total_lines.max(1));
        self.adjust_scroll_for_current_line();
    }
}
