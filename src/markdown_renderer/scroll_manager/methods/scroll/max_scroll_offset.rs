//! Max scroll offset method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Get the maximum valid scroll offset.
    ///
    /// # Returns
    ///
    /// The maximum scroll offset that keeps content visible.
    pub fn max_scroll_offset(&self) -> usize {
        self.total_lines.saturating_sub(self.viewport_height)
    }
}
