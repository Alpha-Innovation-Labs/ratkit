//! Border rendering for the TOC widget in expanded mode.

use ratatui::{buffer::Buffer, layout::Rect};

use super::super::Toc;

/// Border characters for rounded corners.
const CORNER_TOP_LEFT: char = '╭';
const CORNER_TOP_RIGHT: char = '╮';
const CORNER_BOTTOM_LEFT: char = '╰';
const CORNER_BOTTOM_RIGHT: char = '╯';
const HORIZONTAL: char = '─';
const VERTICAL: char = '│';

impl<'a> Toc<'a> {
    /// Render the border around the TOC.
    ///
    /// Draws a rounded border with a title header and decorative separator.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the border in.
    /// * `buf` - The buffer to render to.
    ///
    /// # Returns
    ///
    /// The inner area available for content after the border.
    pub(crate) fn render_border(&self, area: Rect, buf: &mut Buffer) -> Rect {
        if area.width < 4 || area.height < 3 {
            return area;
        }

        let border_style = self.config.border_style;
        let title_style = self.config.title_style;
        let bg_style = self.config.background_style;

        // Top-left corner
        buf.cell_mut((area.x, area.y))
            .map(|cell| cell.set_char(CORNER_TOP_LEFT).set_style(border_style));

        // In expanded mode, show title; in compact mode, just draw the border line
        if self.expanded {
            // Title and top border
            let title = &self.config.title;
            let title_start = area.x + 2;
            let title_end = title_start + title.len() as u16;

            // Space before title
            buf.cell_mut((area.x + 1, area.y))
                .map(|cell| cell.set_char(' ').set_style(bg_style));

            // Render title
            for (i, ch) in title.chars().enumerate() {
                let x = title_start + i as u16;
                if x < area.x + area.width - 1 {
                    buf.cell_mut((x, area.y))
                        .map(|cell| cell.set_char(ch).set_style(title_style));
                }
            }

            // Space after title
            if title_end < area.x + area.width - 1 {
                buf.cell_mut((title_end, area.y))
                    .map(|cell| cell.set_char(' ').set_style(bg_style));
            }

            // Horizontal line after title
            let line_start = title_end + 1;
            for x in line_start..(area.x + area.width - 1) {
                buf.cell_mut((x, area.y))
                    .map(|cell| cell.set_char(HORIZONTAL).set_style(border_style));
            }
        } else {
            // Compact mode: just horizontal line, no title
            for x in (area.x + 1)..(area.x + area.width - 1) {
                buf.cell_mut((x, area.y))
                    .map(|cell| cell.set_char(HORIZONTAL).set_style(border_style));
            }
        }

        // Top-right corner
        buf.cell_mut((area.x + area.width - 1, area.y))
            .map(|cell| cell.set_char(CORNER_TOP_RIGHT).set_style(border_style));

        // Left and right borders (vertical lines)
        for y in (area.y + 1)..(area.y + area.height - 1) {
            buf.cell_mut((area.x, y))
                .map(|cell| cell.set_char(VERTICAL).set_style(border_style));
            buf.cell_mut((area.x + area.width - 1, y))
                .map(|cell| cell.set_char(VERTICAL).set_style(border_style));
        }

        // Bottom-left corner
        buf.cell_mut((area.x, area.y + area.height - 1))
            .map(|cell| cell.set_char(CORNER_BOTTOM_LEFT).set_style(border_style));

        // Bottom border
        for x in (area.x + 1)..(area.x + area.width - 1) {
            buf.cell_mut((x, area.y + area.height - 1))
                .map(|cell| cell.set_char(HORIZONTAL).set_style(border_style));
        }

        // Bottom-right corner
        buf.cell_mut((area.x + area.width - 1, area.y + area.height - 1))
            .map(|cell| cell.set_char(CORNER_BOTTOM_RIGHT).set_style(border_style));

        // Return inner area (excluding border)
        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        }
    }
}
