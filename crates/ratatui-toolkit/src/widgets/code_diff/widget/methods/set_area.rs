use ratatui::layout::Rect as RatatuiRect;

use crate::widgets::code_diff::CodeDiff;

impl CodeDiff {
    /// Set the area for mouse event handling.
    ///
    /// This is called during render so mouse events can use the cached area.
    pub fn set_area(&mut self, area: RatatuiRect) {
        self.area = Some(area);
    }
}
