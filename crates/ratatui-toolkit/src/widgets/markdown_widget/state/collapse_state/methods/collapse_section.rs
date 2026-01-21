//! Collapse section method for CollapseState.

use crate::widgets::markdown_widget::state::collapse_state::CollapseState;

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
