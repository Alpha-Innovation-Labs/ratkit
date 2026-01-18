//! Method to set content directly on a `MarkdownSource`.

use crate::markdown_renderer::MarkdownSource;

impl MarkdownSource {
    /// Set the content directly (for string sources).
    ///
    /// This is useful for updating string-based sources programmatically.
    /// For file sources, this updates the cached content but does not write to disk.
    ///
    /// Returns `true` if the content was changed.
    pub fn set_content(&mut self, new_content: impl Into<String>) -> bool {
        let new_content = new_content.into();
        match self {
            Self::String(content) => {
                if *content != new_content {
                    *content = new_content;
                    true
                } else {
                    false
                }
            }
            Self::File { content, .. } => {
                if *content != new_content {
                    *content = new_content;
                    true
                } else {
                    false
                }
            }
        }
    }
}
