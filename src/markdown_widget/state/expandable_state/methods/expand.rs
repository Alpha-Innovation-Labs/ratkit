//! Expand expandable method for ExpandableState.

use crate::markdown_widget::state::expandable_state::{ExpandableEntry, ExpandableState};

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
