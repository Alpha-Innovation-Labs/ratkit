//! Set dark theme method for SyntaxHighlighter.

use crate::widgets::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;
use crate::widgets::markdown_widget::extensions::theme::SyntaxThemeVariant;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Set the theme to dark mode.
    pub fn set_dark_theme(&mut self) {
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        self.theme = theme_set.themes["base16-ocean.dark"].clone();
        self.theme_variant = SyntaxThemeVariant::Dark;
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Set dark theme (no-op when markdown feature is disabled).
    pub fn set_dark_theme(&mut self) {}
}
