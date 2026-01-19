//! Update cursor position during selection.

use crate::markdown_widget::foundation::types::SelectionPos;
use crate::markdown_widget::state::selection_state::SelectionState;

impl SelectionState {
    /// Update the cursor position during selection.
    ///
    /// # Arguments
    ///
    /// * `x` - New X coordinate
    /// * `y` - New Y coordinate
    pub fn update_cursor(&mut self, x: i32, y: i32) {
        if self.active {
            self.cursor = Some(SelectionPos::new(x, y));
        }
    }

    /// Set anchor at current cursor position (for keyboard selection toggle).
    pub fn set_anchor(&mut self) {
        if self.active {
            if let Some(cursor) = self.cursor {
                self.anchor = Some(cursor);
            }
        }
    }

    /// Clear anchor (deselect).
    pub fn clear_anchor(&mut self) {
        self.anchor = None;
    }

    /// Toggle anchor at current position.
    pub fn toggle_anchor(&mut self) {
        if self.anchor.is_some() {
            self.clear_anchor();
        } else {
            self.set_anchor();
        }
    }
}
