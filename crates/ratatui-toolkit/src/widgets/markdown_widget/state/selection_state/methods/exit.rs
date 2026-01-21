//! Exit selection mode.

use crate::widgets::markdown_widget::state::selection_state::SelectionState;

impl SelectionState {
    /// Exit selection mode and clear state.
    pub fn exit(&mut self) {
        self.active = false;
        self.anchor = None;
        self.cursor = None;
        self.frozen_lines = None;
        self.frozen_width = 0;
    }
}
