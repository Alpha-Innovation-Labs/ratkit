//! Default trait implementation for SourceState.

use crate::widgets::markdown_widget::state::source_state::SourceState;

impl Default for SourceState {
    fn default() -> Self {
        Self::new()
    }
}
