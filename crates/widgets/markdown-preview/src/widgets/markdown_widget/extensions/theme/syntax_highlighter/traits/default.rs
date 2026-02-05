//! Default trait implementation for SyntaxHighlighter.

use crate::widgets::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;

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
