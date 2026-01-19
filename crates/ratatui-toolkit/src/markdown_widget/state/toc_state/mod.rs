//! Table of Contents state for markdown widget.
//!
//! Single source of truth for TOC state including scroll offset, hover state, and entries.

pub mod constructors;
pub mod enums;
pub mod methods;

pub use constructors::*;
pub use enums::*;
pub use methods::*;

/// State for the Table of Contents sidebar.
///
/// Manages scroll position, hover state, and TOC entries.
#[derive(Debug, Clone, Default)]
pub struct TocState {
    /// Current scroll offset within the TOC.
    pub scroll_offset: usize,
    /// Index of the currently hovered entry, if any.
    pub hovered_entry: Option<usize>,
    /// Whether the TOC itself is hovered.
    pub hovered: bool,
    /// List of TOC entries extracted from the document.
    pub entries: Vec<TocEntry>,
}
