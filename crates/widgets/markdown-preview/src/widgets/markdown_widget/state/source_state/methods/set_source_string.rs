//! Set source string method for SourceState.

use crate::widgets::markdown_widget::foundation::source::MarkdownSource;
use crate::widgets::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Set a string-based markdown source.
    ///
    /// **Note:** Caller should invalidate any caches after calling this.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content string.
    pub fn set_source_string(&mut self, content: impl Into<String>) {
        let content_str: String = content.into();
        self.line_count = content_str.lines().count();
        self.source = Some(MarkdownSource::from_string(content_str));
        self.watcher = None;
        self.watch_path = None;
    }
}
