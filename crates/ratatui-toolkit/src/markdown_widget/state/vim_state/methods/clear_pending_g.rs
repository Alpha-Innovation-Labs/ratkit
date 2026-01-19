//! Clear pending g method for VimState.

use crate::markdown_widget::state::vim_state::VimState;

impl VimState {
    /// Clear any pending 'g' keypress.
    pub fn clear_pending_g(&mut self) {
        self.pending_g_time = None;
    }
}
