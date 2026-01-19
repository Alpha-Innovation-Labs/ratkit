//! Default trait implementation for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl Default for ScrollState {
    fn default() -> Self {
        Self::new()
    }
}
