//! Set section collapsed method for CollapseState.

use crate::markdown_widget::state::collapse_state::CollapseState;

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
