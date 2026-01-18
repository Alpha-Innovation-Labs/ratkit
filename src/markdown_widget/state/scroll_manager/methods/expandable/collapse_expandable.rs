//! Collapse expandable method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::{ExpandableState, MarkdownScrollManager};

impl MarkdownScrollManager {
    /// Collapse expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    pub fn collapse_expandable(&mut self, content_id: &str) {
        let state = self
            .expandable_content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableState {
                collapsed: true,
                max_lines: self.default_max_lines,
            });
        state.collapsed = true;
    }
}
