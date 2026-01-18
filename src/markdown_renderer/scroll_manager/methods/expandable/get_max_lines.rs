//! Get max lines method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Get max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    ///
    /// # Returns
    ///
    /// The maximum visible lines for this content, or the default if not set.
    pub fn get_max_lines(&self, content_id: &str) -> usize {
        self.expandable_content
            .get(content_id)
            .map(|state| state.max_lines)
            .unwrap_or(self.default_max_lines)
    }
}
