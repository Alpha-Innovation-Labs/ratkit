//! Syntax highlighting for code blocks using syntect.
//!
//! This module provides syntax highlighting functionality for code blocks
//! in markdown documents using the syntect library.

mod syntax_theme_variant;

mod constructors;
mod methods;
mod traits;

pub use syntax_theme_variant::SyntaxThemeVariant;

/// Highlighter for code blocks using syntect.
pub struct SyntaxHighlighter {
    #[cfg(feature = "markdown")]
    pub(crate) syntax_set: syntect::parsing::SyntaxSet,
    #[cfg(feature = "markdown")]
    pub(crate) theme: syntect::highlighting::Theme,
    #[cfg(feature = "markdown")]
    pub(crate) theme_variant: SyntaxThemeVariant,
}
