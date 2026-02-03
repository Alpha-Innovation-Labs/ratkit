//! Method to check if a `MarkdownSource` is string-based.

use super::super::MarkdownSource;

impl MarkdownSource {
    /// Check if this source is string-based.
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
}
