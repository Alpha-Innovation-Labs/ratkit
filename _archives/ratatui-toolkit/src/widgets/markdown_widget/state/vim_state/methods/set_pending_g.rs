//! Set pending g method for VimState.

use std::time::Instant;

use crate::widgets::markdown_widget::state::vim_state::VimState;

impl VimState {
    /// Set a pending 'g' keypress for potential 'gg' command.
    pub fn set_pending_g(&mut self) {
        self.pending_g_time = Some(Instant::now());
    }
}
