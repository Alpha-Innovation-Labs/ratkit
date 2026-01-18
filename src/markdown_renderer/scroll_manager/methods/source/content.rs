//! Content method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Get the current content from the source.
    ///
    /// # Returns
    ///
    /// The markdown content, or `None` if no source is set.
    pub fn content(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.content())
    }
}
