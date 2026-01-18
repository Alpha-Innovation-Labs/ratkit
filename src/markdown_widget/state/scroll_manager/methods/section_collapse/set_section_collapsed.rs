//! Set section collapsed method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Set the collapse state of a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section.
    /// * `collapsed` - Whether the section should be collapsed.
    pub fn set_section_collapsed(&mut self, section_id: usize, collapsed: bool) {
        self.collapsed_sections.insert(section_id, collapsed);
    }
}
