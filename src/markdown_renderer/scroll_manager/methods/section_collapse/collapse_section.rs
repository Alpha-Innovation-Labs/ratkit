//! Collapse section method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Collapse a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to collapse.
    pub fn collapse_section(&mut self, section_id: usize) {
        self.collapsed_sections.insert(section_id, true);
    }
}
