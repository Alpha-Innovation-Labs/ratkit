//! Check pending gg method for VimState.

use std::time::Duration;

use crate::widgets::markdown_widget::state::vim_state::VimState;

/// Timeout for 'gg' command (in milliseconds).
const GG_TIMEOUT_MS: u64 = 500;

impl VimState {
    /// Check if there's a pending 'g' that would complete a 'gg' command.
    ///
    /// This checks if 'g' was pressed recently enough to form a 'gg' command.
    /// If valid, clears the pending state and returns true.
    ///
    /// # Returns
    ///
    /// `true` if 'gg' command should be executed, `false` otherwise.
    pub fn check_pending_gg(&mut self) -> bool {
        if let Some(pending_time) = self.pending_g_time {
            if pending_time.elapsed() < Duration::from_millis(GG_TIMEOUT_MS) {
                self.pending_g_time = None;
                return true;
            }
        }
        false
    }
}
