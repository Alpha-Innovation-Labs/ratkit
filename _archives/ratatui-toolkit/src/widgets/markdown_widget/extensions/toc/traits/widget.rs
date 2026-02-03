//! Widget trait implementation for Toc.

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::widgets::markdown_widget::extensions::toc::Toc;

impl<'a> Widget for Toc<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if self.expanded {
            // Expanded mode (hovered): show border + text entries
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_char(' ').set_style(self.config.background_style);
                    }
                }
            }

            let content_area = if self.config.show_border {
                self.render_border(area, buf)
            } else {
                area
            };
            self.render_expanded(content_area, buf);
        } else {
            // Compact mode (not hovered): show horizontal lines
            self.render_compact(area, buf);
        }
    }
}
