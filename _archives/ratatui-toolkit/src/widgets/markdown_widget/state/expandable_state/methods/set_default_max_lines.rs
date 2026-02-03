//! Set default max lines method for ExpandableState.

use crate::widgets::markdown_widget::state::expandable_state::ExpandableState;

impl ExpandableState {
    /// Set the default max lines for new expandable content.
    ///
    /// # Arguments
    ///
    /// * `max_lines` - Default maximum visible lines when collapsed (minimum 1).
    pub fn set_default_max_lines(&mut self, max_lines: usize) {
        self.default_max_lines = max_lines.max(1);
    }
}
