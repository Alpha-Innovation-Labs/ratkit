//! Get hierarchy method for CollapseState.

use crate::widgets::markdown_widget::state::CollapseState;

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
