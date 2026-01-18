//! Method to get the content of a `MarkdownSource`.

use super::super::MarkdownSource;

impl MarkdownSource {
    /// Get the current content of the markdown source.
    ///
    /// For string sources, this returns the original string.
    /// For file sources, this returns the cached content.
    pub fn content(&self) -> &str {
        match self {
            Self::String(s) => s,
            Self::File { content, .. } => content,
        }
    }
}
