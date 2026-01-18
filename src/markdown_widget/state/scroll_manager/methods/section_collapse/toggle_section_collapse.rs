//! Toggle section collapse method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Toggle the collapse state of a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to toggle.
    pub fn toggle_section_collapse(&mut self, section_id: usize) {
        let is_collapsed = self.collapsed_sections.entry(section_id).or_insert(false);
        *is_collapsed = !*is_collapsed;
    }
}
