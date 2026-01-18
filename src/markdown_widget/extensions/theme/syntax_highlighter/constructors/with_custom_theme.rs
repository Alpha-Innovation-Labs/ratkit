//! Custom theme constructor for SyntaxHighlighter.

use crate::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;
use crate::markdown_widget::extensions::theme::SyntaxThemeVariant;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with custom theme.
    pub fn with_custom_theme(theme: syntect::highlighting::Theme) -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_variant = if theme.name.as_deref().unwrap_or("").contains("light") {
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
