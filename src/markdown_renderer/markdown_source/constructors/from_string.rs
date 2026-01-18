//! Constructor for creating a `MarkdownSource` from a string.

use crate::markdown_renderer::MarkdownSource;

impl MarkdownSource {
    /// Create a new `MarkdownSource` from a string.
    ///
    /// # Arguments
    /// * `s` - The markdown string content.
    ///
    /// # Example
    /// ```
    /// use ratatui_toolkit::markdown_renderer::MarkdownSource;
    ///
    /// let source = MarkdownSource::from_string("# Hello World");
    /// assert_eq!(source.content(), "# Hello World");
    /// ```
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }
}
