//! Source path method for SourceState.

use std::path::Path;

use crate::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Get the file path if this is a file-based source.
    ///
    /// # Returns
    ///
    /// The file path, or `None` if this is a string source or no source is set.
    pub fn source_path(&self) -> Option<&Path> {
        self.source.as_ref().and_then(|s| s.path())
    }
}
