//! Content method for SourceState.

use crate::widgets::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Get the current content from the source.
    ///
    /// # Returns
    ///
    /// The markdown content, or `None` if no source is set.
    pub fn content(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.content())
    }
}
