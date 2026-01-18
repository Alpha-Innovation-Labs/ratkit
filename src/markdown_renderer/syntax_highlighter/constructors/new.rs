//! Default constructor for SyntaxHighlighter.

use crate::markdown_renderer::syntax_highlighter::{SyntaxHighlighter, SyntaxThemeVariant};

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default dark theme.
    pub fn new() -> Self {
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
    /// Create a new syntax highlighter (no-op when markdown feature is disabled).
    pub fn new() -> Self {
        Self
    }
}
