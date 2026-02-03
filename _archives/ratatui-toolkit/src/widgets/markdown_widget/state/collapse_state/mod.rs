//! Collapse state for markdown widget.
//!
//! Manages section collapse/expand state with hierarchical support.

pub mod constructors;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use traits::*;

use std::collections::HashMap;

/// Collapse state for markdown sections.
///
/// Tracks which sections are collapsed and their hierarchy.
#[derive(Debug, Clone)]
pub struct CollapseState {
    /// Section collapse state: section_id -> is_collapsed.
    sections: HashMap<usize, bool>,
    /// Section hierarchy: section_id -> (level, parent_section_id).
    hierarchy: HashMap<usize, (u8, Option<usize>)>,
}
