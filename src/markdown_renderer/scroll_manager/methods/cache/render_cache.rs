//! Render cache getter for MarkdownScrollManager.

use super::super::super::{MarkdownScrollManager, RenderCache};

impl MarkdownScrollManager {
    /// Get a reference to the render cache if it exists.
    ///
    /// The render cache contains the rendered lines from the last render operation.
    /// This is useful for extracting text for copy operations.
    ///
    /// # Returns
    ///
    /// A reference to the render cache, or None if no rendering has occurred.
    pub fn render_cache(&self) -> Option<&RenderCache> {
        self.render_cache.as_ref()
    }
}
