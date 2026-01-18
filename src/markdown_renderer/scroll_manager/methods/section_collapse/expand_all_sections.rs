//! Expand all sections method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Expand all sections.
    pub fn expand_all_sections(&mut self) {
        let section_ids: Vec<usize> = self.collapsed_sections.keys().copied().collect();
        for section_id in section_ids {
            self.collapsed_sections.insert(section_id, false);
        }
    }
}
