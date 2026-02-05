//! Constructor for ScrollState.

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Create a new scroll state with default settings.
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            viewport_height: 20,
            total_lines: 0,
            current_line: 1,
            filter: None,
            filter_mode: false,
        }
    }
}
