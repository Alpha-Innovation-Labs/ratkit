//! Set source string method for MarkdownScrollManager.

use crate::markdown_widget::foundation::source::MarkdownSource;
use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Set a string-based markdown source.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content string.
    pub fn set_source_string(&mut self, content: impl Into<String>) {
        self.source = Some(MarkdownSource::from_string(content));
        self.invalidate_cache();
    }
}
