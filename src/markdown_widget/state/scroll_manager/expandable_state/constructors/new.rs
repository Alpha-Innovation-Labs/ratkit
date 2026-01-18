//! Constructor for ExpandableState.

use crate::markdown_widget::state::scroll_manager::expandable_state::ExpandableState;

impl ExpandableState {
    /// Create a new expandable state.
    ///
    /// # Arguments
    ///
    /// * `collapsed` - Whether the content is initially collapsed.
    /// * `max_lines` - Maximum visible lines when collapsed (minimum 1).
    pub fn new(collapsed: bool, max_lines: usize) -> Self {
        Self {
            collapsed,
            max_lines: max_lines.max(1),
        }
    }
}
