use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::fuzzy_finder::FuzzyFinder;
use crate::termtui::Color as TermColor;

impl Widget for FuzzyFinder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(ref terminal) = self.terminal {
            let parser = terminal.parser.lock().unwrap();
            let screen = parser.screen();
            let size = screen.size();
            let rows = size.rows as usize;
            let cols = size.cols as usize;

            for row_idx in 0..area.height as usize {
                if row_idx >= rows {
                    break;
                }
                if let Some(screen_row) = screen.primary_grid().visible_row(row_idx as u16) {
                    for col_idx in 0..area.width as usize {
                        if col_idx >= cols {
                            break;
                        }
                        if let Some(cell) = screen_row.get(col_idx as u16) {
                            if let Some(buf_cell) =
                                buf.cell_mut((area.x + col_idx as u16, area.y + row_idx as u16))
                            {
                                let mut style = cell.attrs().to_ratatui();
                                if cell.attrs().fg == TermColor::Default
                                    && cell.attrs().bg == TermColor::Default
                                {
                                    style = style.fg(ratatui::style::Color::Reset);
                                }
                                buf_cell
                                    .set_char(cell.text().chars().next().unwrap_or(' '))
                                    .set_style(style);
                            }
                        }
                    }
                }
            }
        } else {
            use ratatui::style::{Color, Style};

            let center_x = area.x + area.width.saturating_sub(14) / 2;
            let center_y = area.y + area.height / 2;

            if center_y < area.y + area.height {
                buf.set_string(
                    center_x,
                    center_y,
                    &self.loading_message,
                    Style::default().fg(Color::Yellow),
                );
            }
        }
    }
}
