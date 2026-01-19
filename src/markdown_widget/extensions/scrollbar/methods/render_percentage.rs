//! Render the percentage indicator.

use ratatui::{buffer::Buffer, layout::Rect};

use crate::markdown_widget::extensions::scrollbar::CustomScrollbar;

impl<'a> CustomScrollbar<'a> {
    /// Render the percentage indicator next to the scrollbar.
    ///
    /// Shows the current scroll percentage (0-100%) to the left of the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `area` - The scrollbar area (percentage renders to the left).
    /// * `buf` - The buffer to render to.
    pub(crate) fn render_percentage(&self, area: Rect, buf: &mut Buffer) {
        let total = self.scroll_state.total_lines;
        let viewport = self.scroll_state.viewport_height;
        let offset = self.scroll_state.scroll_offset;

        // Calculate percentage
        let max_scroll = total.saturating_sub(viewport);
        let percentage = if max_scroll > 0 {
            ((offset as f64 / max_scroll as f64) * 100.0).round() as u8
        } else {
            0
        };

        // Format percentage string (right-aligned, 4 chars: "100%", " 50%", "  0%")
        let pct_str = format!("{:>3}%", percentage);

        // Render to the left of the scrollbar, vertically centered
        let text_x = area.x.saturating_sub(pct_str.len() as u16 + 1);
        let text_y = area.y + area.height / 2;

        if text_y < area.y + area.height {
            for (i, ch) in pct_str.chars().enumerate() {
                let x = text_x + i as u16;
                if let Some(cell) = buf.cell_mut((x, text_y)) {
                    cell.set_char(ch).set_style(self.config.percentage_style);
                }
            }
        }
    }
}
