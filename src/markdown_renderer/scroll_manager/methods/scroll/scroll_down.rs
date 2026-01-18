//! Scroll down method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
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
