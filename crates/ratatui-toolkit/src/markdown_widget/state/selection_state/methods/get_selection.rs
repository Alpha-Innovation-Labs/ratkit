//! Get the current selection bounds.

use crate::markdown_widget::foundation::types::SelectionPos;
use crate::markdown_widget::state::selection_state::SelectionState;

impl SelectionState {
    /// Get the normalized selection bounds (start, end) where start <= end.
    ///
    /// # Returns
    ///
    /// `Some((start, end))` if there's an active selection, `None` otherwise.
    pub fn get_selection(&self) -> Option<(SelectionPos, SelectionPos)> {
        if !self.active {
            return None;
        }

        let anchor = self.anchor?;
        let cursor = self.cursor?;

        // Normalize so start is before end
        Some(normalize_selection(anchor, cursor))
    }

    /// Check if there's an active selection (anchor and cursor set).
    pub fn has_selection(&self) -> bool {
        self.active && self.anchor.is_some() && self.cursor.is_some()
    }
}

/// Normalize selection bounds so start <= end.
fn normalize_selection(a: SelectionPos, b: SelectionPos) -> (SelectionPos, SelectionPos) {
    if a.y < b.y || (a.y == b.y && a.x <= b.x) {
        (a, b)
    } else {
        (b, a)
    }
}
