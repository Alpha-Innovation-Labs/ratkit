//! Is current line visible method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Check if current line is visible in the viewport.
    ///
    /// # Returns
    ///
    /// `true` if the current line is within the visible viewport.
    pub fn is_current_line_visible(&self) -> bool {
        let first_visible = self.scroll_offset + 1;
        let last_visible = self.scroll_offset + self.viewport_height;
        self.current_line >= first_visible && self.current_line <= last_visible
    }
}
