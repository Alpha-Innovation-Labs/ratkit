//! Constructor for SelectionState.

use crate::markdown_widget::state::selection_state::SelectionState;

impl SelectionState {
    /// Create a new inactive selection state.
    pub fn new() -> Self {
        Self::default()
    }
}
