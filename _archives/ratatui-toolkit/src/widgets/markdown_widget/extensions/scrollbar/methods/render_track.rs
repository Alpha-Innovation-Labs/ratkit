//! Render the scrollbar track (background).

use ratatui::{buffer::Buffer, layout::Rect};

use crate::widgets::markdown_widget::extensions::scrollbar::CustomScrollbar;

impl<'a> CustomScrollbar<'a> {
    /// Render the track (background) of the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the track in.
    /// * `buf` - The buffer to render to.
    pub(crate) fn render_track(&self, area: Rect, buf: &mut Buffer) {
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(self.config.track_char)
                        .set_style(self.config.track_style);
                }
            }
        }
    }
}
