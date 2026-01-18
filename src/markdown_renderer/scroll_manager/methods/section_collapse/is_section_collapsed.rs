//! Is section collapsed method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Check if a section is collapsed (directly or via parent hierarchy).
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to check.
    ///
    /// # Returns
    ///
    /// `true` if the section or any of its parent sections is collapsed.
    pub fn is_section_collapsed(&self, section_id: usize) -> bool {
        // First check if this section is directly collapsed
        if self
            .collapsed_sections
            .get(&section_id)
            .copied()
            .unwrap_or(false)
        {
            return true;
        }

        // Check if any parent section is collapsed (hierarchical collapse)
        let mut current_id = section_id;
        while let Some(&(_level, parent_id)) = self.section_hierarchy.get(&current_id) {
            if let Some(parent) = parent_id {
                if self
                    .collapsed_sections
                    .get(&parent)
                    .copied()
                    .unwrap_or(false)
                {
                    return true;
                }
                current_id = parent;
            } else {
                break;
            }
        }

        false
    }
}
