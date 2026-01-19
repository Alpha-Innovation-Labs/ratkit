//! Update viewport method for ScrollState.

use ratatui::layout::Rect;

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Update viewport dimensions.
    ///
    /// # Arguments
    ///
    /// * `area` - The new viewport area.
    pub fn update_viewport(&mut self, area: Rect) {
        self.viewport_height = area.height as usize;
    }
}
