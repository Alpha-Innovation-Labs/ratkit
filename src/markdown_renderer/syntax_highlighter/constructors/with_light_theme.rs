//! Light theme constructor for SyntaxHighlighter.

use crate::markdown_renderer::syntax_highlighter::{SyntaxHighlighter, SyntaxThemeVariant};

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with GitHub Light theme.
    pub fn with_light_theme() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get("github-light")
            .cloned()
            .unwrap_or_else(|| theme_set.themes["base16-ocean.light"].clone());

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Light,
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with light theme (no-op when markdown feature is disabled).
    pub fn with_light_theme() -> Self {
        Self
    }
}
