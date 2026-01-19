//! Set max lines method for ExpandableState.

use crate::markdown_widget::state::expandable_state::{ExpandableEntry, ExpandableState};

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
