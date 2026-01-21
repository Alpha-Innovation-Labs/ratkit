//! Is collapsed method for ExpandableState.

use crate::widgets::markdown_widget::state::expandable_state::ExpandableState;

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
