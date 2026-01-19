use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

use crate::statusbar::StatusBar;

impl Default for StatusBar<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        for x in area.x..area.x + area.width {
            buf[(x, area.y)].set_style(self.style);
        }

        let left_spans = self.build_spans(&self.left_items);
        if !left_spans.is_empty() {
            let left_line = Line::from(left_spans);
            buf.set_line(area.x + 1, area.y, &left_line, area.width);
        }

        let center_spans = self.build_spans(&self.center_items);
        if !center_spans.is_empty() {
            let center_line = Line::from(center_spans);
            let center_width = center_line.width() as u16;
            let center_x = area.x + (area.width.saturating_sub(center_width)) / 2;
            buf.set_line(center_x, area.y, &center_line, area.width);
        }

        let right_spans = self.build_spans(&self.right_items);
        if !right_spans.is_empty() {
            let right_line = Line::from(right_spans);
            let right_width = right_line.width() as u16;
            let right_x = area.x + area.width.saturating_sub(right_width + 1);
            buf.set_line(right_x, area.y, &right_line, area.width);
        }
    }
}
