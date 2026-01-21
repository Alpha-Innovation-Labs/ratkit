//! Source state for markdown widget.
//!
//! Manages markdown content source - either from a string or a file.

pub mod constructors;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use traits::*;

use crate::widgets::markdown_widget::foundation::source::MarkdownSource;

/// Source state for markdown content management.
///
/// Manages the markdown source (string or file) and tracks line count.
#[derive(Debug, Clone)]
pub struct SourceState {
    /// Optional markdown source (string or file-based).
    source: Option<MarkdownSource>,
    /// Source file line count (for accurate status bar display).
    pub line_count: usize,
}
