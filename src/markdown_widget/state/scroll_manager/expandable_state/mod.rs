//! State for expandable content.
//!
//! Tracks collapse state and max visible lines for expandable content sections.

pub mod constructors;

/// State for expandable content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandableState {
    /// Whether the content is collapsed (showing limited lines).
    pub collapsed: bool,
    /// Maximum number of visible lines when collapsed.
    pub max_lines: usize,
}
