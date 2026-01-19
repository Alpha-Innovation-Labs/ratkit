//! Default trait implementation for SourceState.

use crate::markdown_widget::state::source_state::SourceState;

impl Default for SourceState {
    fn default() -> Self {
        Self::new()
    }
}
