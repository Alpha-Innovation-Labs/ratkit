//! Register section method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Register section hierarchy (called during parsing).
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section.
    /// * `level` - The heading level (1-6).
    /// * `parent_section_id` - The parent section's ID, if any.
    pub fn register_section(
        &mut self,
        section_id: usize,
        level: u8,
        parent_section_id: Option<usize>,
    ) {
        self.section_hierarchy
            .insert(section_id, (level, parent_section_id));
    }
}
