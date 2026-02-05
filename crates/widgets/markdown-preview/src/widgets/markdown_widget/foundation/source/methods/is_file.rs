//! Method to check if a `MarkdownSource` is file-based.

use super::super::MarkdownSource;

impl MarkdownSource {
    /// Check if this source is file-based.
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }
}
