//! Enter selection mode.

use crate::markdown_widget::foundation::types::SelectionPos;
use crate::markdown_widget::state::selection_state::SelectionState;
use ratatui::text::Line;

impl SelectionState {
    /// Enter selection mode at the given position.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (column)
    /// * `y` - Y coordinate (document row)
    /// * `lines` - Current rendered lines to freeze
    /// * `width` - Current render width
    pub fn enter(&mut self, x: i32, y: i32, lines: Vec<Line<'static>>, width: usize) {
        self.active = true;
        self.anchor = Some(SelectionPos::new(x, y));
        self.cursor = Some(SelectionPos::new(x, y));
        self.frozen_lines = Some(lines);
        self.frozen_width = width;
    }

    /// Check if selection mode is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}
