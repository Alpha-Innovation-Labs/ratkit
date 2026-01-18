//! Visible range method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Get range of currently visible lines (1-indexed, inclusive).
    ///
    /// # Returns
    ///
    /// A tuple of (start_line, end_line) for visible content.
    pub fn visible_range(&self) -> (usize, usize) {
        let start = self.scroll_offset + 1;
        let end = (self.scroll_offset + self.viewport_height).min(self.total_lines);
        (start, end)
    }
}
