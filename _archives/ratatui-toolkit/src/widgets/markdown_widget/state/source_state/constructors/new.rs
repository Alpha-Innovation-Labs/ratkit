//! Constructor for SourceState.

use crate::widgets::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Create a new source state with no source.
    pub fn new() -> Self {
        Self {
            source: None,
            line_count: 0,
        }
    }
}
