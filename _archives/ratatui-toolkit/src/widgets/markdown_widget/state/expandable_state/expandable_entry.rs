//! State for a single expandable content entry.

/// State for a single expandable content entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandableEntry {
    /// Whether the content is collapsed (showing limited lines).
    pub collapsed: bool,
    /// Maximum number of visible lines when collapsed.
    pub max_lines: usize,
}

impl ExpandableEntry {
    /// Create a new expandable entry.
    pub fn new(collapsed: bool, max_lines: usize) -> Self {
        Self {
            collapsed,
            max_lines: max_lines.max(1),
        }
    }
}
