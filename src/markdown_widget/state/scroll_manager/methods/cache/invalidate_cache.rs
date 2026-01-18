//! Invalidate cache method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Invalidate both parsed and render caches.
    ///
    /// Call this when content changes.
    pub fn invalidate_cache(&mut self) {
        self.parsed_cache = None;
        self.render_cache = None;
        self.section_hierarchy.clear();
    }
}
