//! Expand section method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Expand a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to expand.
    pub fn expand_section(&mut self, section_id: usize) {
        self.collapsed_sections.insert(section_id, false);
    }
}
