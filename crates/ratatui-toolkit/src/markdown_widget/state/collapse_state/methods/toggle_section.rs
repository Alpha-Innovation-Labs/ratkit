//! Toggle section collapse method for CollapseState.

use crate::markdown_widget::state::collapse_state::CollapseState;

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
