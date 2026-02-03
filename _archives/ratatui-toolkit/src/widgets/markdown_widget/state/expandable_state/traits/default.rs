//! Default trait implementation for ExpandableState.

use crate::widgets::markdown_widget::state::expandable_state::ExpandableState;

impl Default for ExpandableState {
    fn default() -> Self {
        Self::new()
    }
}
