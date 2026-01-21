//! Line count method for SourceState.

use crate::widgets::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Get the line count of the source content.
    ///
    /// # Returns
    ///
    /// The number of lines in the source content.
    pub fn line_count(&self) -> usize {
        self.line_count
    }
}
