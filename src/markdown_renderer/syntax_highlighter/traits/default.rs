//! Default trait implementation for SyntaxHighlighter.

use crate::markdown_renderer::syntax_highlighter::SyntaxHighlighter;

#[cfg(feature = "markdown")]
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "markdown"))]
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
