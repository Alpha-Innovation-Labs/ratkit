//! Widget trait implementation for Minimap.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::markdown_widget::extensions::minimap::Minimap;

impl Widget for Minimap<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Fill background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf[(x, y)].set_style(self.config.background_style);
            }
        }

        // Render minimap lines
        let lines = self.render_to_lines(area.height as usize);

        for (i, line) in lines.iter().enumerate() {
            if i >= area.height as usize {
                break;
            }

            let y = area.y + i as u16;
            let mut x = area.x;

            for span in line.spans.iter() {
                for ch in span.content.chars() {
                    if x >= area.x + area.width {
                        break;
                    }
                    buf[(x, y)].set_char(ch);
                    buf[(x, y)].set_style(span.style);
                    x += 1;
                }
            }
        }
    }
}
