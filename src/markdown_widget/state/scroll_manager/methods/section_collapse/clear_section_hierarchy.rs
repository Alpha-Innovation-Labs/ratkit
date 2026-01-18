//! Clear section hierarchy method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Clear section hierarchy (called when content changes).
    pub fn clear_section_hierarchy(&mut self) {
        self.section_hierarchy.clear();
    }
}
