//! Default trait implementation for CollapseState.

use crate::markdown_widget::state::collapse_state::CollapseState;

impl Default for CollapseState {
    fn default() -> Self {
        Self::new()
    }
}
