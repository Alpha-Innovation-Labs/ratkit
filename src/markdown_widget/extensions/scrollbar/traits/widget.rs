//! Widget trait implementation for CustomScrollbar.

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::markdown_widget::extensions::scrollbar::CustomScrollbar;

impl<'a> Widget for CustomScrollbar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Don't render if content fits in viewport
        if self.scroll_state.total_lines <= self.scroll_state.viewport_height {
            return;
        }

        // Render track (background)
        self.render_track(area, buf);

        // Render thumb (scrollable indicator)
        self.render_thumb(area, buf);

        // Render percentage if enabled
        if self.show_percentage {
            self.render_percentage(area, buf);
        }
    }
}
