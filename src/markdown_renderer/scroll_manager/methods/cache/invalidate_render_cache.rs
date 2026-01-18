//! Invalidate render cache method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Invalidate only the render cache.
    ///
    /// Call this when width changes but content is the same.
    pub fn invalidate_render_cache(&mut self) {
        self.render_cache = None;
    }
}
