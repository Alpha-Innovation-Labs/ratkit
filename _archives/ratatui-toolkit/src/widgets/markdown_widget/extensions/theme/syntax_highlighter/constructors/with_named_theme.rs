//! Named theme constructor for SyntaxHighlighter.

use crate::widgets::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;
use crate::widgets::markdown_widget::extensions::theme::SyntaxThemeVariant;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with a specific theme name.
    ///
    /// # Arguments
    ///
    /// * `theme_name` - Name of the theme (e.g., "base16-ocean.dark", "github-dark", "github-light")
    pub fn with_named_theme(theme_name: &str) -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get(theme_name)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to dark theme
                theme_set.themes["base16-ocean.dark"].clone()
            });

        let theme_variant = if theme_name.contains("light") {
            SyntaxThemeVariant::Light
        } else {
            SyntaxThemeVariant::Dark
        };

        Self {
            syntax_set,
            theme,
            theme_variant,
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with a named theme (no-op when markdown feature is disabled).
    pub fn with_named_theme(_theme_name: &str) -> Self {
        Self
    }
}
