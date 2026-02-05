//! Display settings for markdown widget.
//!
//! Manages display-related configuration like line numbers and themes.

pub mod constructors;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use traits::*;

use crate::widgets::markdown_widget::foundation::elements::CodeBlockTheme;

/// Display settings for markdown rendering.
///
/// Controls visual options like line numbers, themes, and collapse indicators.
#[derive(Debug, Clone)]
pub struct DisplaySettings {
    /// Whether to show line numbers in code blocks.
    pub show_line_numbers: bool,
    /// Whether to show line numbers for the entire document.
    pub show_document_line_numbers: bool,
    /// Color theme for code blocks.
    pub code_block_theme: CodeBlockTheme,
    /// Whether to show collapse indicators on headings.
    pub show_heading_collapse: bool,
    /// Scroll multiplier (lines per scroll tick).
    pub scroll_multiplier: usize,
}
