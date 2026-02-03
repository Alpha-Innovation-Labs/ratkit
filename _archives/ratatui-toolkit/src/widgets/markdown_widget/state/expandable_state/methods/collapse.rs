//! Collapse expandable method for ExpandableState.

use crate::widgets::markdown_widget::state::expandable_state::{ExpandableEntry, ExpandableState};

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
