//! Terminal grid with VecDeque-based scrollback (mprocs architecture)

use crate::row::Row;
use crate::size::Size;
use std::collections::VecDeque;

/// Cursor position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pos {
    pub col: u16,
    pub row: u16,
}

impl Pos {
    pub fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

/// Terminal grid with scrollback buffer
///
/// Uses VecDeque to efficiently manage scrollback history.
/// The visible rows are at the end of the deque, with scrollback
/// history at the front.
#[derive(Clone, Debug)]
pub struct Grid {
    /// All rows (scrollback + visible)
    rows: VecDeque<Row>,
    /// Current size
    size: Size,
    /// Cursor position
    pos: Pos,
    /// Maximum scrollback lines
    scrollback_len: usize,
    /// Current scrollback offset (0 = showing latest)
    scrollback_offset: usize,
    /// Number of rows that have been used
    used_rows: usize,
    /// Scroll region top (0-indexed)
    scroll_top: u16,
    /// Scroll region bottom (0-indexed, exclusive)
    scroll_bottom: u16,
    /// Saved cursor position
    saved_pos: Option<Pos>,
}

impl Grid {
    /// Create a new grid
    pub fn new(size: Size, scrollback_len: usize) -> Self {
        let rows = (0..size.rows)
            .map(|_| Row::new(size.cols))
            .collect::<VecDeque<_>>();

        Self {
            rows,
            size,
            pos: Pos::default(),
            scrollback_len,
            scrollback_offset: 0,
            used_rows: 0,
            scroll_top: 0,
            scroll_bottom: size.rows,
            saved_pos: None,
        }
    }

    /// Get the grid size
    pub fn size(&self) -> Size {
        self.size
    }

    /// Get cursor position
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Set cursor position
    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = Pos {
            col: pos.col.min(self.size.cols.saturating_sub(1)),
            row: pos.row.min(self.size.rows.saturating_sub(1)),
        };
    }

    /// Move cursor to column
    pub fn set_col(&mut self, col: u16) {
        self.pos.col = col.min(self.size.cols.saturating_sub(1));
    }

    /// Move cursor to row
    pub fn set_row(&mut self, row: u16) {
        self.pos.row = row.min(self.size.rows.saturating_sub(1));
    }

    /// Save cursor position
    pub fn save_pos(&mut self) {
        self.saved_pos = Some(self.pos);
    }

    /// Restore cursor position
    pub fn restore_pos(&mut self) {
        if let Some(pos) = self.saved_pos {
            self.pos = pos;
        }
    }

    /// Get the index where visible rows begin in the deque
    fn row0(&self) -> usize {
        self.rows.len().saturating_sub(self.size.rows as usize)
    }

    /// Get current scrollback offset
    pub fn scrollback(&self) -> usize {
        self.scrollback_offset
    }

    /// Set scrollback offset
    pub fn set_scrollback(&mut self, offset: usize) {
        let max_offset = self.row0();
        self.scrollback_offset = offset.min(max_offset);
    }

    /// Get available scrollback lines
    pub fn scrollback_available(&self) -> usize {
        self.row0()
    }

    /// Set scroll region
    pub fn set_scroll_region(&mut self, top: u16, bottom: u16) {
        self.scroll_top = top.min(self.size.rows.saturating_sub(1));
        self.scroll_bottom = bottom.min(self.size.rows).max(self.scroll_top + 1);
    }

    /// Reset scroll region to full screen
    pub fn reset_scroll_region(&mut self) {
        self.scroll_top = 0;
        self.scroll_bottom = self.size.rows;
    }

    /// Get a visible row (accounting for scrollback offset)
    pub fn visible_row(&self, row: u16) -> Option<&Row> {
        let idx = self.row0() + row as usize;
        let idx = idx.saturating_sub(self.scrollback_offset);
        self.rows.get(idx)
    }

    /// Get a drawing row (for writing, ignores scrollback offset)
    pub fn drawing_row(&self, row: u16) -> Option<&Row> {
        let idx = self.row0() + row as usize;
        self.rows.get(idx)
    }

    /// Get a mutable drawing row
    pub fn drawing_row_mut(&mut self, row: u16) -> Option<&mut Row> {
        let idx = self.row0() + row as usize;
        if row as usize >= self.used_rows {
            self.used_rows = row as usize + 1;
        }
        self.rows.get_mut(idx)
    }

    /// Get current row (at cursor position)
    pub fn current_row(&self) -> Option<&Row> {
        self.drawing_row(self.pos.row)
    }

    /// Get mutable current row
    pub fn current_row_mut(&mut self) -> Option<&mut Row> {
        let row = self.pos.row;
        self.drawing_row_mut(row)
    }

    /// Scroll up within scroll region
    pub fn scroll_up(&mut self, count: usize) {
        for _ in 0..count {
            // If scroll region is full screen, add to scrollback
            if self.scroll_top == 0 && self.scroll_bottom == self.size.rows {
                // Add new row at the end
                self.rows.push_back(Row::new(self.size.cols));

                // Trim scrollback if needed
                while self.rows.len() > self.size.rows as usize + self.scrollback_len {
                    self.rows.pop_front();
                }
            } else {
                // Scroll within region only
                let top_idx = self.row0() + self.scroll_top as usize;
                let bottom_idx = self.row0() + self.scroll_bottom as usize - 1;

                if top_idx < self.rows.len() && bottom_idx < self.rows.len() {
                    // Remove top row of region
                    self.rows.remove(top_idx);
                    // Insert new row at bottom of region
                    self.rows.insert(bottom_idx, Row::new(self.size.cols));
                }
            }
        }
    }

    /// Scroll down within scroll region
    pub fn scroll_down(&mut self, count: usize) {
        for _ in 0..count {
            let top_idx = self.row0() + self.scroll_top as usize;
            let bottom_idx = self.row0() + self.scroll_bottom as usize - 1;

            if top_idx < self.rows.len() && bottom_idx < self.rows.len() {
                // Remove bottom row of region
                self.rows.remove(bottom_idx);
                // Insert new row at top of region
                self.rows.insert(top_idx, Row::new(self.size.cols));
            }
        }
    }

    /// Clear all rows
    pub fn clear(&mut self) {
        for row in self.rows.iter_mut() {
            row.clear();
        }
        self.used_rows = 0;
    }

    /// Clear from cursor to end of screen
    pub fn clear_below(&mut self) {
        let pos_row = self.pos.row;
        let pos_col = self.pos.col;
        let cols = self.size.cols;
        let rows = self.size.rows;

        // Clear current row from cursor
        if let Some(row) = self.drawing_row_mut(pos_row) {
            row.erase(pos_col, cols);
        }

        // Clear all rows below
        for r in (pos_row + 1)..rows {
            if let Some(row) = self.drawing_row_mut(r) {
                row.clear();
            }
        }
    }

    /// Clear from start of screen to cursor
    pub fn clear_above(&mut self) {
        let pos_row = self.pos.row;
        let pos_col = self.pos.col;

        // Clear all rows above
        for r in 0..pos_row {
            if let Some(row) = self.drawing_row_mut(r) {
                row.clear();
            }
        }

        // Clear current row up to cursor
        if let Some(row) = self.drawing_row_mut(pos_row) {
            row.erase(0, pos_col + 1);
        }
    }

    /// Resize the grid
    pub fn resize(&mut self, new_size: Size) {
        // Resize existing rows
        for row in self.rows.iter_mut() {
            row.resize(new_size.cols);
        }

        // Add or remove rows as needed
        while self.rows.len() < new_size.rows as usize {
            self.rows.push_back(Row::new(new_size.cols));
        }

        // Update size
        self.size = new_size;

        // Clamp cursor and scroll region
        self.pos.col = self.pos.col.min(new_size.cols.saturating_sub(1));
        self.pos.row = self.pos.row.min(new_size.rows.saturating_sub(1));
        self.scroll_bottom = new_size.rows;
    }

    /// Get selected text from coordinates
    ///
    /// Handles wrapped lines correctly (no newline for soft-wrapped rows)
    pub fn get_selected_text(&self, low_x: i32, low_y: i32, high_x: i32, high_y: i32) -> String {
        let mut contents = String::new();

        let row0 = self.row0() as i32;
        let start_row = (row0 + low_y).max(0) as usize;
        let end_row = (row0 + high_y).max(0) as usize;

        for (i, row) in self.rows.iter().enumerate() {
            if i < start_row || i > end_row {
                continue;
            }

            let width = row.width();

            // Determine start and end columns for this row
            let start_col = if i == start_row {
                (low_x.max(0) as u16).min(width)
            } else {
                0
            };

            let end_col = if i == end_row {
                (high_x.max(0) as u16).min(width)
            } else {
                width
            };

            // Extract text from this row
            row.write_contents(&mut contents, start_col, end_col);

            // Add newline unless this row wraps (soft wrap)
            if i != end_row && !row.wrapped() {
                contents.push('\n');
            }
        }

        // Trim trailing whitespace from each line
        contents
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Iterate over visible rows
    pub fn visible_rows(&self) -> impl Iterator<Item = &Row> {
        let start = self.row0().saturating_sub(self.scrollback_offset);
        let end = start + self.size.rows as usize;
        self.rows.iter().skip(start).take(end - start)
    }

    /// Iterate over drawing rows (ignoring scrollback offset)
    pub fn drawing_rows(&self) -> impl Iterator<Item = &Row> {
        let start = self.row0();
        self.rows.iter().skip(start).take(self.size.rows as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(Size::new(80, 24), 1000);
        assert_eq!(grid.size().cols, 80);
        assert_eq!(grid.size().rows, 24);
        assert_eq!(grid.pos().col, 0);
        assert_eq!(grid.pos().row, 0);
    }

    #[test]
    fn test_grid_cursor() {
        let mut grid = Grid::new(Size::new(80, 24), 1000);

        grid.set_pos(Pos::new(10, 5));
        assert_eq!(grid.pos(), Pos::new(10, 5));

        // Should clamp to bounds
        grid.set_pos(Pos::new(100, 50));
        assert_eq!(grid.pos(), Pos::new(79, 23));
    }

    #[test]
    fn test_grid_scroll_up() {
        let mut grid = Grid::new(Size::new(80, 24), 100);

        // Write something to first row
        if let Some(row) = grid.drawing_row_mut(0) {
            if let Some(cell) = row.get_mut(0) {
                cell.set_text("A");
            }
        }

        // Scroll up
        grid.scroll_up(1);

        // Check scrollback available
        assert_eq!(grid.scrollback_available(), 1);
    }

    #[test]
    fn test_grid_scrollback() {
        let mut grid = Grid::new(Size::new(80, 24), 100);

        // Fill with some content and scroll
        for _ in 0..10 {
            grid.scroll_up(1);
        }

        assert_eq!(grid.scrollback_available(), 10);

        // Set scrollback offset
        grid.set_scrollback(5);
        assert_eq!(grid.scrollback(), 5);

        // Should clamp to available
        grid.set_scrollback(1000);
        assert_eq!(grid.scrollback(), 10);
    }

    #[test]
    fn test_grid_get_selected_text() {
        let mut grid = Grid::new(Size::new(80, 24), 100);

        // Write "Hello World" on first row
        if let Some(row) = grid.drawing_row_mut(0) {
            for (i, c) in "Hello World".chars().enumerate() {
                if let Some(cell) = row.get_mut(i as u16) {
                    cell.set_text(c.to_string());
                }
            }
        }

        let text = grid.get_selected_text(0, 0, 5, 0);
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_grid_resize() {
        let mut grid = Grid::new(Size::new(80, 24), 100);

        grid.resize(Size::new(120, 40));
        assert_eq!(grid.size().cols, 120);
        assert_eq!(grid.size().rows, 40);
    }
}
