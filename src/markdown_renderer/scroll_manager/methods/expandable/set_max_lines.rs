//! Set max lines method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::{ExpandableState, MarkdownScrollManager};

impl MarkdownScrollManager {
    /// Set max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    /// * `max_lines` - Maximum visible lines when collapsed (minimum 1).
    pub fn set_max_lines(&mut self, content_id: &str, max_lines: usize) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.max_lines = max_lines.max(1);
    }
}
