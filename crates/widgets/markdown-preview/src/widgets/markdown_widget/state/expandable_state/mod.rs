//! Expandable state for markdown widget.
//!
//! Manages expandable content sections that can be collapsed/expanded.

pub mod constructors;
pub mod expandable_entry;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use expandable_entry::ExpandableEntry;
pub use methods::*;
pub use traits::*;

use std::collections::HashMap;

/// Expandable state for markdown content.
///
/// Tracks which content blocks are expanded/collapsed and their max line settings.
#[derive(Debug, Clone)]
pub struct ExpandableState {
    /// Expandable content state: content_id -> entry state.
    content: HashMap<String, ExpandableEntry>,
    /// Default max lines for expandable content.
    default_max_lines: usize,
}
