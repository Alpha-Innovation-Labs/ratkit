//! Default trait implementation for DoubleClickState.

use crate::markdown_widget::state::double_click_state::DoubleClickState;

impl Default for DoubleClickState {
    fn default() -> Self {
        Self {
            last_click_time: None,
            last_click_pos: None,
            pending_single_click: None,
        }
    }
}
