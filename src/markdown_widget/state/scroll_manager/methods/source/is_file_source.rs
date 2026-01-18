//! Is file source method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Check if the source is file-based.
    ///
    /// # Returns
    ///
    /// `true` if the source is loaded from a file, `false` otherwise.
    pub fn is_file_source(&self) -> bool {
        self.source.as_ref().map(|s| s.is_file()).unwrap_or(false)
    }
}
