//! Set light theme method for SyntaxHighlighter.

use crate::widgets::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;
use crate::widgets::markdown_widget::extensions::theme::SyntaxThemeVariant;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Set the theme to light mode (GitHub Light).
    pub fn set_light_theme(&mut self) {
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        self.theme = theme_set
            .themes
            .get("github-light")
            .cloned()
            .unwrap_or_else(|| theme_set.themes["base16-ocean.light"].clone());
        self.theme_variant = SyntaxThemeVariant::Light;
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Set light theme (no-op when markdown feature is disabled).
    pub fn set_light_theme(&mut self) {}
}
