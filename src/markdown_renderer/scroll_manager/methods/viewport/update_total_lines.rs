//! Update total lines method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Update total line count.
    ///
    /// # Arguments
    ///
    /// * `total` - The total number of lines in the document.
    pub fn update_total_lines(&mut self, total: usize) {
        self.total_lines = total.max(1);
    }
}
