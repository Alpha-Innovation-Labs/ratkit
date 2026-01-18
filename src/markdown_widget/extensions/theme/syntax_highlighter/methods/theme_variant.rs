//! Theme variant getter method for SyntaxHighlighter.

use crate::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;
use crate::markdown_widget::extensions::theme::SyntaxThemeVariant;

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
