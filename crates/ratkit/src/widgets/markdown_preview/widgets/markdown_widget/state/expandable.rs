//! Expandable state for markdown widget.
//!
//! Manages expandable content sections that can be collapsed/expanded.

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

/// Constructor for ExpandableState.

impl ExpandableState {
    /// Create a new expandable state with defaults.
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            default_max_lines: 3,
        }
    }
}

/// State for a single expandable content entry.

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

/// Collapse expandable method for ExpandableState.

impl ExpandableState {
    /// Collapse expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    pub fn collapse(&mut self, content_id: &str) {
        let state = self
            .content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableEntry::new(true, self.default_max_lines));
        state.collapsed = true;
    }
}

/// Expand expandable method for ExpandableState.

impl ExpandableState {
    /// Expand expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    pub fn expand(&mut self, content_id: &str) {
        let state = self
            .content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableEntry::new(false, self.default_max_lines));
        state.collapsed = false;
    }
}

/// Get max lines method for ExpandableState.

impl ExpandableState {
    /// Get max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    ///
    /// # Returns
    ///
    /// The maximum visible lines for this content, or the default if not set.
    pub fn get_max_lines(&self, content_id: &str) -> usize {
        self.content
            .get(content_id)
            .map(|state| state.max_lines)
            .unwrap_or(self.default_max_lines)
    }
}

/// Is collapsed method for ExpandableState.

impl ExpandableState {
    /// Check if expandable content is collapsed.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    ///
    /// # Returns
    ///
    /// `true` if the content is collapsed (default state).
    pub fn is_collapsed(&self, content_id: &str) -> bool {
        self.content
            .get(content_id)
            .map(|state| state.collapsed)
            .unwrap_or(true)
    }
}

/// Set default max lines method for ExpandableState.

impl ExpandableState {
    /// Set the default max lines for new expandable content.
    ///
    /// # Arguments
    ///
    /// * `max_lines` - Default maximum visible lines when collapsed (minimum 1).
    pub fn set_default_max_lines(&mut self, max_lines: usize) {
        self.default_max_lines = max_lines.max(1);
    }
}

/// Set max lines method for ExpandableState.

impl ExpandableState {
    /// Set max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    /// * `max_lines` - Maximum visible lines when collapsed (minimum 1).
    pub fn set_max_lines(&mut self, content_id: &str, max_lines: usize) {
        let state = self
            .content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableEntry::new(true, self.default_max_lines));
        state.max_lines = max_lines.max(1);
    }
}

/// Toggle expandable method for ExpandableState.

impl ExpandableState {
    /// Toggle expandable content collapsed state.
    ///
    /// # Arguments
    ///
    /// * `content_id` - The ID of the expandable content.
    pub fn toggle(&mut self, content_id: &str) {
        let state = self
            .content
            .entry(content_id.to_string())
            .or_insert_with(|| ExpandableEntry::new(true, self.default_max_lines));
        state.collapsed = !state.collapsed;
    }
}

/// Default trait implementation for ExpandableState.

impl Default for ExpandableState {
    fn default() -> Self {
        Self::new()
    }
}
