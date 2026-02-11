//! Collapse state for markdown widget.
//!
//! Manages section collapse/expand state with hierarchical support.

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

/// Constructor for CollapseState.

impl CollapseState {
    /// Create a new collapse state with no collapsed sections.
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            hierarchy: HashMap::new(),
        }
    }
}

/// Clear hierarchy method for CollapseState.

impl CollapseState {
    /// Clear section hierarchy (called when content changes).
    pub fn clear_hierarchy(&mut self) {
        self.hierarchy.clear();
    }
}

/// Collapse all sections method for CollapseState.

impl CollapseState {
    /// Collapse all sections.
    pub fn collapse_all(&mut self) {
        let section_ids: Vec<usize> = self.sections.keys().copied().collect();
        for section_id in section_ids {
            self.sections.insert(section_id, true);
        }
    }
}

/// Collapse section method for CollapseState.

impl CollapseState {
    /// Collapse a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to collapse.
    pub fn collapse_section(&mut self, section_id: usize) {
        self.sections.insert(section_id, true);
    }
}

/// Expand all sections method for CollapseState.

impl CollapseState {
    /// Expand all sections.
    pub fn expand_all(&mut self) {
        let section_ids: Vec<usize> = self.sections.keys().copied().collect();
        for section_id in section_ids {
            self.sections.insert(section_id, false);
        }
    }
}

/// Expand section method for CollapseState.

impl CollapseState {
    /// Expand a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to expand.
    pub fn expand_section(&mut self, section_id: usize) {
        self.sections.insert(section_id, false);
    }
}

/// Get hierarchy method for CollapseState.

impl CollapseState {
    /// Get the hierarchy information for a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The section ID to look up
    ///
    /// # Returns
    ///
    /// `Some((level, parent_id))` if the section exists, `None` otherwise.
    pub fn get_hierarchy(&self, section_id: usize) -> Option<(u8, Option<usize>)> {
        self.hierarchy.get(&section_id).copied()
    }
}

/// Is section collapsed method for CollapseState.

impl CollapseState {
    /// Check if a section is collapsed (directly or via parent hierarchy).
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to check.
    ///
    /// # Returns
    ///
    /// `true` if the section or any of its parent sections is collapsed.
    pub fn is_section_collapsed(&self, section_id: usize) -> bool {
        // First check if this section is directly collapsed
        if self.sections.get(&section_id).copied().unwrap_or(false) {
            return true;
        }

        // Check if any parent section is collapsed (hierarchical collapse)
        let mut current_id = section_id;
        while let Some(&(_level, parent_id)) = self.hierarchy.get(&current_id) {
            if let Some(parent) = parent_id {
                if self.sections.get(&parent).copied().unwrap_or(false) {
                    return true;
                }
                current_id = parent;
            } else {
                break;
            }
        }

        false
    }
}

/// Register section method for CollapseState.

impl CollapseState {
    /// Register section hierarchy (called during parsing).
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section.
    /// * `level` - The heading level (1-6).
    /// * `parent_section_id` - The parent section's ID, if any.
    pub fn register_section(
        &mut self,
        section_id: usize,
        level: u8,
        parent_section_id: Option<usize>,
    ) {
        self.hierarchy
            .insert(section_id, (level, parent_section_id));
    }
}

/// Set section collapsed method for CollapseState.

impl CollapseState {
    /// Set the collapse state of a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section.
    /// * `collapsed` - Whether the section should be collapsed.
    pub fn set_section_collapsed(&mut self, section_id: usize, collapsed: bool) {
        self.sections.insert(section_id, collapsed);
    }
}

/// Toggle section collapse method for CollapseState.

impl CollapseState {
    /// Toggle the collapse state of a section.
    ///
    /// # Arguments
    ///
    /// * `section_id` - The ID of the section to toggle.
    pub fn toggle_section(&mut self, section_id: usize) {
        let is_collapsed = self.sections.entry(section_id).or_insert(false);
        *is_collapsed = !*is_collapsed;
    }
}

/// Default trait implementation for CollapseState.

impl Default for CollapseState {
    fn default() -> Self {
        Self::new()
    }
}
