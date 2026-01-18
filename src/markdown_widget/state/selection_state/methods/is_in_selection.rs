//! Check if a position is within the selection.

use crate::markdown_widget::foundation::types::SelectionPos;
use crate::markdown_widget::state::selection_state::SelectionState;

impl SelectionState {
    /// Check if a cell at (x, y) is within the current selection.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (column)
    /// * `y` - Y coordinate (row)
    ///
    /// # Returns
    ///
    /// `true` if the position is within the selection.
    pub fn is_in_selection(&self, x: i32, y: i32) -> bool {
        let Some((start, end)) = self.get_selection() else {
            return false;
        };

        is_pos_in_selection(x, y, &start, &end)
    }
}

/// Check if position (x, y) is within the selection bounds.
pub fn is_pos_in_selection(x: i32, y: i32, start: &SelectionPos, end: &SelectionPos) -> bool {
    // Outside y range
    if y < start.y || y > end.y {
        return false;
    }

    // Single line selection
    if start.y == end.y {
        return x >= start.x && x <= end.x;
    }

    // Multi-line selection
    if y == start.y {
        // First line: from start.x to end of line
        return x >= start.x;
    } else if y == end.y {
        // Last line: from start of line to end.x
        return x <= end.x;
    } else {
        // Middle lines: entire line is selected
        true
    }
}
