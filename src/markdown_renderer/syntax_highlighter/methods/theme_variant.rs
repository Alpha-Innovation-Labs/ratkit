//! Theme variant getter method for SyntaxHighlighter.

use crate::markdown_renderer::syntax_highlighter::{SyntaxHighlighter, SyntaxThemeVariant};

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Get the current theme variant.
    pub fn theme_variant(&self) -> SyntaxThemeVariant {
        self.theme_variant
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Get current theme variant (always returns Dark when markdown feature is disabled).
    pub fn theme_variant(&self) -> SyntaxThemeVariant {
        SyntaxThemeVariant::Dark
    }
}
