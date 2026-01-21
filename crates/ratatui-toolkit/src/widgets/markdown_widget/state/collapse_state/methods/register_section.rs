//! Register section method for CollapseState.

use crate::widgets::markdown_widget::state::collapse_state::CollapseState;

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
