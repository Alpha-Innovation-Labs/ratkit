//! Terminal grid with VecDeque-based scrollback
//!
//! Inspired by mprocs' implementation using VecDeque for efficient
//! circular buffer scrollback.

use super::cell::Cell;
use std::collections::VecDeque;

/// Terminal size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub rows: usize,
    pub cols: usize,
}

/// A single row in the grid
#[derive(Debug, Clone)]
pub struct Row {
    cells: Vec<Cell>,
    wrapped: bool, // Whether this line is wrapped from previous
}

impl Row {
    /// Create a new row with given width
    fn new(cols: usize) -> Self {
        Self {
            cells: vec![Cell::default(); cols],
            wrapped: false,
        }
    }

    /// Get number of columns
    pub fn cols(&self) -> usize {
        self.cells.len()
    }

    /// Check if row is wrapped
    pub fn wrapped(&self) -> bool {
        self.wrapped
    }

    /// Set wrapped status
    pub fn set_wrapped(&mut self, wrapped: bool) {
        self.wrapped = wrapped;
    }

    /// Get cell at column
    pub fn get(&self, col: usize) -> Option<&Cell> {
        self.cells.get(col)
    }

    /// Get mutable cell at column
    pub fn get_mut(&mut self, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(col)
    }

    /// Set cell at column
    pub fn set(&mut self, col: usize, cell: Cell) {
        if col < self.cells.len() {
            self.cells[col] = cell;
        }
    }

    /// Extract text content from this row
    pub fn text_content(&self, start: usize, width: usize) -> String {
        let end = (start + width).min(self.cells.len());
        self.cells[start..end]
            .iter()
            .map(|cell| cell.text.as_str())
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}

/// Grid with VecDeque-based scrollback
///
/// Layout: [scrollback_rows... | visible_rows...]
/// row0() calculates where visible area starts
#[derive(Debug, Clone)]
pub struct Grid {
    /// Current visible size
    size: Size,

    /// All rows (scrollback + visible)
    rows: VecDeque<Row>,

    /// Maximum scrollback lines
    scrollback_len: usize,

    /// Current scrollback offset (0 = at bottom)
    scrollback_offset: usize,
}

impl Grid {
    /// Create a new grid
    pub fn new(rows: usize, cols: usize, scrollback_len: usize) -> Self {
        let size = Size { rows, cols };
        let mut grid_rows = VecDeque::with_capacity(rows + scrollback_len);

        for _ in 0..rows {
            grid_rows.push_back(Row::new(cols));
        }

        Self {
            size,
            rows: grid_rows,
            scrollback_len,
            scrollback_offset: 0,
        }
    }

    /// Get grid size
    pub fn size(&self) -> Size {
        self.size
    }

    /// Get current scrollback offset
    pub fn scrollback(&self) -> usize {
        self.scrollback_offset
    }

    /// Get maximum scrollback length
    pub fn scrollback_len(&self) -> usize {
        self.scrollback_len
    }

    /// Set scrollback offset
    pub fn set_scrollback(&mut self, offset: usize) {
        self.scrollback_offset = offset.min(self.row0());
    }

    /// Scroll screen up (view older content)
    pub fn scroll_screen_up(&mut self, n: usize) {
        self.scrollback_offset = (self.scrollback_offset + n).min(self.row0());
    }

    /// Scroll screen down (view newer content)
    pub fn scroll_screen_down(&mut self, n: usize) {
        self.scrollback_offset = self.scrollback_offset.saturating_sub(n);
    }

    /// Calculate where visible area starts in the deque
    fn row0(&self) -> usize {
        self.rows.len().saturating_sub(self.size.rows)
    }

    /// Get visible rows iterator
    pub fn visible_rows(&self) -> impl Iterator<Item = &Row> {
        let start = self.row0().saturating_sub(self.scrollback_offset);
        self.rows.iter().skip(start).take(self.size.rows)
    }

    /// Get cell at position (relative to visible area)
    pub fn cell(&self, row: usize, col: usize) -> Option<&Cell> {
        let start = self.row0().saturating_sub(self.scrollback_offset);
        let actual_row = start + row;
        self.rows.get(actual_row)?.get(col)
    }

    /// Get mutable cell at position (relative to current visible bottom)
    pub fn cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        let row_index = self.row0() + row;
        self.rows.get_mut(row_index)?.get_mut(col)
    }

    /// Get row (relative to visible area)
    pub fn row(&self, row: usize) -> Option<&Row> {
        let start = self.row0();
        self.rows.get(start + row)
    }

    /// Scroll content up (add new line at bottom, old line goes to scrollback)
    pub fn scroll_up(&mut self, count: usize) {
        for _ in 0..count.min(self.size.rows) {
            let row0 = self.row0();

            // Add new empty row at bottom
            self.rows
                .insert(row0 + self.size.rows, Row::new(self.size.cols));

            // Remove top row and add to scrollback if enabled
            if self.scrollback_len > 0 {
                if let Some(removed) = self.rows.remove(row0) {
                    // Add to scrollback (front of deque)
                    self.rows.insert(row0, removed);
                }

                // Limit scrollback size
                while self.rows.len() > self.size.rows + self.scrollback_len {
                    self.rows.pop_front();
                }

                // Adjust scroll offset if user was scrolled up
                if self.scrollback_offset > 0 {
                    self.scrollback_offset = self.row0().min(self.scrollback_offset + 1);
                }
            } else {
                self.rows.remove(row0);
            }
        }
    }

    /// Resize the grid
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.size = Size { rows, cols };

        // Adjust row count
        while self.rows.len() < rows {
            self.rows.push_back(Row::new(cols));
        }

        // Resize existing rows to new column count
        for row in &mut self.rows {
            row.cells.resize(cols, Cell::default());
        }
    }

    /// Extract selected text
    pub fn get_selected_text(&self, low_x: i32, low_y: i32, high_x: i32, high_y: i32) -> String {
        let mut contents = String::new();
        let lines_len = high_y - low_y + 1;

        for i in 0..lines_len {
            let row_idx = (self.row0() as i32 + low_y + i) as usize;

            if let Some(row) = self.rows.get(row_idx) {
                let start = if i == 0 { low_x.max(0) as usize } else { 0 };
                let width = if i == lines_len - 1 {
                    (high_x + 1 - start as i32).max(0) as usize
                } else {
                    row.cols().saturating_sub(start)
                };

                let text = row.text_content(start, width);
                contents.push_str(&text);

                // Add newline unless it's the last line or row is wrapped
                if i != lines_len - 1 && !row.wrapped() {
                    contents.push('\n');
                }
            }
        }

        contents
    }

    /// Clear all cells
    pub fn clear(&mut self) {
        for row in &mut self.rows {
            for cell in &mut row.cells {
                *cell = Cell::default();
            }
        }
    }
}
