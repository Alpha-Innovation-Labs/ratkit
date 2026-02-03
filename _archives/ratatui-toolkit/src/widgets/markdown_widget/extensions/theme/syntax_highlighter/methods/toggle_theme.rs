//! Toggle theme method for SyntaxHighlighter.

use crate::widgets::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;
use crate::widgets::markdown_widget::extensions::theme::SyntaxThemeVariant;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Switch between light and dark themes.
    pub fn toggle_theme(&mut self) {
        match self.theme_variant {
            SyntaxThemeVariant::Dark => self.set_light_theme(),
            SyntaxThemeVariant::Light => self.set_dark_theme(),
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Toggle theme (no-op when markdown feature is disabled).
    pub fn toggle_theme(&mut self) {}
}
