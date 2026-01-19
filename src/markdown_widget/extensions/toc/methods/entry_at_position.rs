//! Position-based entry lookup for hover and click detection.

use ratatui::layout::Rect;

use super::super::Toc;
use super::find_entry_at_position_compact::find_entry_at_position_compact;
use super::find_entry_at_position_expanded::find_entry_at_position_expanded;
use super::get_expanded_content_area::get_expanded_content_area;

impl<'a> Toc<'a> {
    /// Find the entry index at a given screen position.
    ///
    /// # Arguments
    ///
    /// * `x` - Screen X coordinate.
    /// * `y` - Screen Y coordinate.
    /// * `area` - The area the TOC is rendered in.
    ///
    /// # Returns
    ///
    /// The entry index at that position, or None if no entry is there.
    pub fn entry_at_position(&self, x: u16, y: u16, area: Rect) -> Option<usize> {
        // Check horizontal bounds - must be within the TOC width
        if x < area.x || x >= area.x + area.width {
            return None;
        }

        // Check if above the TOC area
        if y < area.y {
            return None;
        }

        let entries = self.toc_state.entries();
        if entries.is_empty() {
            return None;
        }

        if self.expanded {
            // Calculate content area based on actual entries rather than passed area height
            let content_area = get_expanded_content_area(area, &self.config, entries.len());
            find_entry_at_position_expanded(
                x,
                y,
                content_area,
                entries,
                self.toc_state.scroll_offset,
            )
        } else {
            find_entry_at_position_compact(y, area, &self.config, entries)
        }
    }
}
