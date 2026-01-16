//! Private helper methods for [`ClickableScrollbarState`] mouse handling.
//!
//! This module contains internal helper methods used by the mouse event
//! handling implementation. These are private methods that calculate
//! scroll increments, map positions to offsets, and check orientation.

use ratatui::widgets::ScrollbarOrientation;

use crate::clickable_scrollbar::ClickableScrollbarState;

impl ClickableScrollbarState {
    /// Calculates the scroll increment amount.
    ///
    /// Returns the configured `scroll_by` value if set, otherwise
    /// auto-calculates as `page_len / 10` (minimum 1).
    ///
    /// # Returns
    ///
    /// The number of units to scroll per scroll event
    pub(crate) fn calculate_scroll_increment(&self) -> usize {
        self.scroll_by
            .unwrap_or_else(|| (self.page_len / 10).max(1))
    }

    /// Maps a mouse position to a scroll offset.
    ///
    /// Converts a column/row position within the scrollbar area to the
    /// corresponding scroll offset in the content.
    ///
    /// # Arguments
    ///
    /// * `col` - Mouse column position
    /// * `row` - Mouse row position
    ///
    /// # Returns
    ///
    /// The scroll offset corresponding to the mouse position
    pub(crate) fn map_position_to_offset(&self, col: u16, row: u16) -> usize {
        if self.is_vertical() {
            let pos = row.saturating_sub(self.area.y).saturating_sub(1) as usize;
            let span = self.area.height.saturating_sub(2) as usize;

            if span > 0 {
                (self.max_offset * pos) / span
            } else {
                0
            }
        } else {
            let pos = col.saturating_sub(self.area.x).saturating_sub(1) as usize;
            let span = self.area.width.saturating_sub(2) as usize;

            if span > 0 {
                (self.max_offset * pos) / span
            } else {
                0
            }
        }
    }

    /// Checks if the scrollbar orientation is vertical.
    ///
    /// # Returns
    ///
    /// true if the scrollbar is vertical (left or right), false if horizontal
    pub(crate) fn is_vertical(&self) -> bool {
        matches!(
            self.orientation,
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft
        )
    }
}
