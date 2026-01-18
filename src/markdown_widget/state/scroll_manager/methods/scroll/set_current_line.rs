//! Set current line method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Set current line and adjust scroll to keep it visible.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number to set as current (1-indexed).
    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line.clamp(1, self.total_lines);
        self.adjust_scroll_for_current_line();
    }
}
