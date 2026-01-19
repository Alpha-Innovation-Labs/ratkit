//! Render the scrollbar thumb (scrollable indicator).

use ratatui::{buffer::Buffer, layout::Rect};

use crate::markdown_widget::extensions::scrollbar::CustomScrollbar;

use super::thumb_bounds::thumb_bounds;

impl<'a> CustomScrollbar<'a> {
    /// Render the thumb (scrollable indicator) of the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the thumb in (same as track area).
    /// * `buf` - The buffer to render to.
    pub(crate) fn render_thumb(&self, area: Rect, buf: &mut Buffer) {
        let (thumb_y, thumb_height) =
            thumb_bounds(self.scroll_state, area.height, self.config.min_thumb_height);

        let thumb_start = area.y + thumb_y;
        let thumb_end = thumb_start + thumb_height;

        for y in thumb_start..thumb_end.min(area.y + area.height) {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(self.config.thumb_char)
                        .set_style(self.config.thumb_style);
                }
            }
        }
    }
}
