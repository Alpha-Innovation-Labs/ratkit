//! Constructor for creating a `MarkdownSource` from a string.

use super::super::MarkdownSource;

impl MarkdownSource {
    /// Create a new `MarkdownSource` from a string.
    ///
    /// # Arguments
    /// * `s` - The markdown string content.
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }
}
