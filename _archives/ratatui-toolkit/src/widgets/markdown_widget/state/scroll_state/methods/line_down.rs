//! Line down method for ScrollState.

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Move current line down (for keyboard navigation).
    pub fn line_down(&mut self) {
        if self.current_line < self.total_lines {
            self.current_line += 1;
        }
        self.adjust_scroll_for_current_line();
    }
}
