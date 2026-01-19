//! Clear pending method for DoubleClickState.

use crate::markdown_widget::state::double_click_state::DoubleClickState;

impl DoubleClickState {
    /// Clear any pending click (e.g., when double-click is detected).
    pub fn clear_pending(&mut self) {
        self.pending_single_click = None;
    }
}
