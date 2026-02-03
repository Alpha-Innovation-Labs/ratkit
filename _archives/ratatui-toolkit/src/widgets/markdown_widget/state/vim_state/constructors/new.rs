//! Constructor for VimState.

use crate::widgets::markdown_widget::state::vim_state::VimState;

impl VimState {
    /// Create a new vim state with defaults.
    pub fn new() -> Self {
        Self {
            pending_g_time: None,
        }
    }
}
