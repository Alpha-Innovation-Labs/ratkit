//! Syntax highlighting for code blocks using syntect.
//!
//! This module provides syntax highlighting functionality for code blocks
//! in markdown documents using the syntect library.

pub mod constructors;
pub mod methods;
mod traits;

use crate::widgets::markdown_widget::extensions::theme::SyntaxThemeVariant;

/// Highlighter for code blocks using syntect.
pub struct SyntaxHighlighter {
    #[cfg(feature = "markdown")]
    pub(crate) syntax_set: syntect::parsing::SyntaxSet,
    #[cfg(feature = "markdown")]
    pub(crate) theme: syntect::highlighting::Theme,
    #[cfg(feature = "markdown")]
    pub(crate) theme_variant: SyntaxThemeVariant,
}
