//! Default trait implementation for VimState.

use crate::markdown_widget::state::vim_state::VimState;

impl Default for VimState {
    fn default() -> Self {
        Self::new()
    }
}
