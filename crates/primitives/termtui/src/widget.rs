//! Ratatui widget for rendering the terminal

use crate::copy_mode::{CopyMode, CopyPos};
use crate::screen::Screen;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;

/// Widget for rendering a terminal screen
pub struct TermTuiWidget<'a> {
    /// The screen to render
    screen: &'a Screen,
    /// Scroll offset (0 = latest)
    scroll_offset: usize,
    /// Copy mode state (for rendering selection)
    copy_mode: Option<&'a CopyMode>,
}

impl<'a> TermTuiWidget<'a> {
    /// Create a new widget
    pub fn new(screen: &'a Screen) -> Self {
        Self {
            screen,
            scroll_offset: 0,
            copy_mode: None,
        }
    }

    /// Set scroll offset
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Set copy mode for selection rendering
    pub fn copy_mode(mut self, mode: &'a CopyMode) -> Self {
        self.copy_mode = Some(mode);
        self
    }
}

impl Widget for TermTuiWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let size = self.screen.size();
        let _screen_rows = size.rows as usize;
        let screen_cols = size.cols as usize;

        // Get selection bounds if in copy mode
        let selection = self.copy_mode.and_then(|m| m.get_selection());
        let copy_cursor = self.copy_mode.and_then(|m| m.cursor());

        // Render each visible row
        for (row_idx, row) in self.screen.visible_rows().enumerate() {
            if row_idx >= area.height as usize {
                break;
            }

            let y = area.y + row_idx as u16;

            // Render each cell in the row
            for (col_idx, cell) in row.cells().enumerate() {
                if col_idx >= area.width as usize || col_idx >= screen_cols {
                    break;
                }

                let x = area.x + col_idx as u16;

                // Skip wide character continuations
                if cell.is_wide_continuation() {
                    continue;
                }

                // Get cell style
                let mut style = cell.attrs().to_ratatui();

                // Check if this cell is in selection
                if let Some((start, end)) = &selection {
                    let cell_y = row_idx as i32 - self.scroll_offset as i32;
                    let cell_x = col_idx as i32;

                    if is_in_selection(cell_x, cell_y, start, end) {
                        style = Style::default()
                            .bg(Color::Rgb(70, 130, 180))
                            .fg(Color::White);
                    }
                }

                // Get the character to render
                let ch = cell.text().chars().next().unwrap_or(' ');

                if let Some(buf_cell) = buf.cell_mut((x, y)) {
                    buf_cell.set_char(ch).set_style(style);
                }
            }
        }

        // Render cursor
        let cursor_pos = self.screen.cursor_pos();

        // In copy mode, render copy mode cursor instead
        if let Some(copy_cursor) = copy_cursor {
            let cursor_row = (copy_cursor.y + self.scroll_offset as i32) as u16;
            let cursor_col = copy_cursor.x as u16;

            if cursor_row < area.height && cursor_col < area.width {
                let x = area.x + cursor_col;
                let y = area.y + cursor_row;

                if let Some(cell) = buf.cell_mut((x, y)) {
                    let cursor_style = Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD);
                    cell.set_style(cursor_style);
                }
            }
        } else if self.screen.cursor_visible() && self.scroll_offset == 0 {
            // Normal cursor (only when not scrolled back)
            let cursor_row = cursor_pos.row;
            let cursor_col = cursor_pos.col;

            if cursor_row < area.height && cursor_col < area.width {
                let x = area.x + cursor_col;
                let y = area.y + cursor_row;

                if let Some(cell) = buf.cell_mut((x, y)) {
                    let cursor_style = Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::REVERSED);

                    // Use block cursor if cell is empty
                    if cell.symbol() == " " {
                        cell.set_char('█');
                    }
                    cell.set_style(cursor_style);
                }
            }
        }
    }
}

/// Check if a cell position is within the selection range
fn is_in_selection(x: i32, y: i32, start: &CopyPos, end: &CopyPos) -> bool {
    let (low, high) = CopyPos::to_low_high(start, end);

    if y < low.y || y > high.y {
        return false;
    }

    if y == low.y && y == high.y {
        // Single line selection
        x >= low.x && x <= high.x
    } else if y == low.y {
        // First line of multi-line selection
        x >= low.x
    } else if y == high.y {
        // Last line of multi-line selection
        x <= high.x
    } else {
        // Middle lines - entire line selected
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_selection_single_line() {
        let start = CopyPos::new(5, 10);
        let end = CopyPos::new(15, 10);

        assert!(is_in_selection(5, 10, &start, &end));
        assert!(is_in_selection(10, 10, &start, &end));
        assert!(is_in_selection(15, 10, &start, &end));
        assert!(!is_in_selection(4, 10, &start, &end));
        assert!(!is_in_selection(16, 10, &start, &end));
        assert!(!is_in_selection(10, 9, &start, &end));
    }

    #[test]
    fn test_is_in_selection_multi_line() {
        let start = CopyPos::new(5, 10);
        let end = CopyPos::new(15, 12);

        // First line
        assert!(is_in_selection(5, 10, &start, &end));
        assert!(is_in_selection(50, 10, &start, &end)); // All of first line from start
        assert!(!is_in_selection(4, 10, &start, &end));

        // Middle line
        assert!(is_in_selection(0, 11, &start, &end));
        assert!(is_in_selection(50, 11, &start, &end));

        // Last line
        assert!(is_in_selection(0, 12, &start, &end));
        assert!(is_in_selection(15, 12, &start, &end));
        assert!(!is_in_selection(16, 12, &start, &end));
    }

    #[test]
    fn test_is_in_selection_reversed() {
        // Selection from bottom-right to top-left
        let start = CopyPos::new(15, 12);
        let end = CopyPos::new(5, 10);

        // Should work the same due to to_low_high normalization
        assert!(is_in_selection(10, 11, &start, &end));
    }
}
