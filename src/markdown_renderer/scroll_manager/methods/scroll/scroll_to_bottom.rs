//! Scroll to bottom method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Move to bottom of document.
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.max_scroll_offset();
        self.current_line = self.total_lines;
    }
}
