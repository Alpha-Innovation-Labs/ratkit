//! Dark theme constructor for SyntaxHighlighter.

use crate::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;
use crate::markdown_widget::extensions::theme::SyntaxThemeVariant;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with dark theme.
    pub fn with_dark_theme() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Dark,
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with dark theme (no-op when markdown feature is disabled).
    pub fn with_dark_theme() -> Self {
        Self
    }
}
