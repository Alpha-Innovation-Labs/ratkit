//! Clear hierarchy method for CollapseState.

use crate::markdown_widget::state::collapse_state::CollapseState;

impl CollapseState {
    /// Clear section hierarchy (called when content changes).
    pub fn clear_hierarchy(&mut self) {
        self.hierarchy.clear();
    }
}
