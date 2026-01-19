//! Adjust scroll for current line method for ScrollState.

use crate::markdown_widget::state::scroll_state::ScrollState;

impl ScrollState {
    /// Adjust scroll offset to ensure current_line is visible with a margin.
    /// Scrolls when the cursor gets within 3 lines of the top/bottom edge.
    pub fn adjust_scroll_for_current_line(&mut self) {
        const SCROLL_MARGIN: usize = 3;

        if self.viewport_height == 0 {
            return;
        }

        // Calculate the visible range
        let first_visible = self.scroll_offset + 1;
        let last_visible = self.scroll_offset + self.viewport_height;

        // If viewport is too small for margins, fall back to simple visibility
        if self.viewport_height <= SCROLL_MARGIN * 2 {
            if self.current_line < first_visible {
                self.scroll_offset = self.current_line.saturating_sub(1);
            } else if self.current_line > last_visible {
                self.scroll_offset = self.current_line.saturating_sub(self.viewport_height);
            }
            return;
        }

        // Scroll up if current line is within margin of the top
        let top_threshold = first_visible + SCROLL_MARGIN;
        if self.current_line < top_threshold && self.scroll_offset > 0 {
            // Scroll up to maintain margin
            let desired_offset = self.current_line.saturating_sub(SCROLL_MARGIN + 1);
            self.scroll_offset = desired_offset;
        }

        // Scroll down if current line is within margin of the bottom
        let bottom_threshold = last_visible.saturating_sub(SCROLL_MARGIN);
        if self.current_line > bottom_threshold {
            // Scroll down to maintain margin
            let desired_offset = self.current_line + SCROLL_MARGIN - self.viewport_height;
            self.scroll_offset = desired_offset.min(self.max_scroll_offset());
        }
    }
}
