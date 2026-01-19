//! Markdown panel component (simplified wrapper)

/// Simple markdown panel that wraps markdown content
pub struct MarkdownPanel<'a> {
    content: &'a str,
}

impl<'a> MarkdownPanel<'a> {
    /// Create a new markdown panel
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }

    /// Get content
    pub fn content(&self) -> &'a str {
        self.content
    }
}
