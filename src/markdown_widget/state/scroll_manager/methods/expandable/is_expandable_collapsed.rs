//! Is expandable collapsed method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Check if expandable content is collapsed.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    ///
    /// # Returns
    ///
    /// `true` if the content is collapsed (default state).
    pub fn is_expandable_collapsed(&self, content_id: &str) -> bool {
        self.expandable_content
            .get(content_id)
            .map(|state| state.collapsed)
            .unwrap_or(true)
    }
}
