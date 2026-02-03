//! Terminal row representation

use crate::primitives::termtui::attrs::Attrs;
use crate::primitives::termtui::cell::Cell;

/// A row of terminal cells
#[derive(Clone, Debug)]
pub struct Row {
    /// Cells in this row
    cells: Vec<Cell>,
    /// Number of cells that have been written to
    size: u16,
    /// Whether this row wraps to the next line
    wrapped: bool,
}

impl Row {
    /// Create a new row with the given width
    pub fn new(width: u16) -> Self {
        Self {
            cells: (0..width).map(|_| Cell::new()).collect(),
            size: 0,
            wrapped: false,
        }
    }

    /// Create a new row with specific attributes
    pub fn new_with_attrs(width: u16, attrs: Attrs) -> Self {
        Self {
            cells: (0..width).map(|_| Cell::with_attrs(attrs)).collect(),
            size: 0,
            wrapped: false,
        }
    }

    /// Get the width of this row
    pub fn width(&self) -> u16 {
        self.cells.len() as u16
    }

    /// Get a cell at the given column
    pub fn get(&self, col: u16) -> Option<&Cell> {
        self.cells.get(col as usize)
    }

    /// Get a mutable cell at the given column
    pub fn get_mut(&mut self, col: u16) -> Option<&mut Cell> {
        // Track size (rightmost written cell)
        if col >= self.size {
            self.size = col + 1;
        }
        self.cells.get_mut(col as usize)
    }

    /// Insert a cell at the given column, shifting others right
    pub fn insert(&mut self, col: u16, cell: Cell) {
        let col = col as usize;
        if col < self.cells.len() {
            self.cells.insert(col, cell);
            self.cells.pop(); // Keep same width
        }
    }

    /// Remove a cell at the given column, shifting others left
    pub fn remove(&mut self, col: u16) {
        let col = col as usize;
        if col < self.cells.len() {
            self.cells.remove(col);
            self.cells.push(Cell::new()); // Keep same width
        }
    }

    /// Clear the row
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
        self.size = 0;
        self.wrapped = false;
    }

    /// Clear cells from start to end (exclusive)
    pub fn erase(&mut self, start: u16, end: u16) {
        let start = start as usize;
        let end = (end as usize).min(self.cells.len());

        for cell in &mut self.cells[start..end] {
            cell.clear();
        }
    }

    /// Resize the row
    pub fn resize(&mut self, new_width: u16) {
        let new_width = new_width as usize;
        if new_width > self.cells.len() {
            self.cells.resize_with(new_width, Cell::new);
        } else {
            self.cells.truncate(new_width);
        }
    }

    /// Check if this row wraps to the next line
    pub fn wrapped(&self) -> bool {
        self.wrapped
    }

    /// Set whether this row wraps
    pub fn set_wrapped(&mut self, wrapped: bool) {
        self.wrapped = wrapped;
    }

    /// Check if a column is a wide character continuation
    pub fn is_wide_continuation(&self, col: u16) -> bool {
        self.cells
            .get(col as usize)
            .map(|c| c.is_wide_continuation())
            .unwrap_or(false)
    }

    /// Clear wide character at position (both cells)
    pub fn clear_wide(&mut self, col: u16) {
        let col = col as usize;
        if col < self.cells.len() {
            // Check if this is a wide char
            if self.cells[col].width() == 2 && col + 1 < self.cells.len() {
                self.cells[col].clear();
                self.cells[col + 1].clear();
            } else if col > 0 && self.cells[col].is_wide_continuation() {
                // This is continuation, clear the previous cell too
                self.cells[col - 1].clear();
                self.cells[col].clear();
            } else {
                self.cells[col].clear();
            }
        }
    }

    /// Write cell contents to a string (for text extraction)
    pub fn write_contents(&self, output: &mut String, start: u16, end: u16) {
        let start = start as usize;
        let end = (end as usize).min(self.cells.len());

        for cell in &self.cells[start..end] {
            if !cell.is_wide_continuation() {
                output.push_str(cell.text());
            }
        }
    }

    /// Get trimmed contents (no trailing spaces)
    pub fn contents_trimmed(&self) -> String {
        let mut output = String::new();
        self.write_contents(&mut output, 0, self.width());
        output.trim_end().to_string()
    }

    /// Iterate over cells
    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    /// Get the number of cells actually written to
    pub fn used_width(&self) -> u16 {
        self.size
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new(80)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_new() {
        let row = Row::new(80);
        assert_eq!(row.width(), 80);
        assert!(!row.wrapped());
    }

    #[test]
    fn test_row_get_set() {
        let mut row = Row::new(80);

        if let Some(cell) = row.get_mut(5) {
            cell.set_text("X");
        }

        assert_eq!(row.get(5).map(|c| c.text()), Some("X"));
    }

    #[test]
    fn test_row_clear() {
        let mut row = Row::new(80);

        if let Some(cell) = row.get_mut(5) {
            cell.set_text("X");
        }

        row.clear();
        assert_eq!(row.get(5).map(|c| c.text()), Some(" "));
    }

    #[test]
    fn test_row_erase() {
        let mut row = Row::new(80);

        for i in 0..10 {
            if let Some(cell) = row.get_mut(i) {
                cell.set_text("X");
            }
        }

        row.erase(3, 7);
        assert_eq!(row.get(2).map(|c| c.text()), Some("X"));
        assert_eq!(row.get(3).map(|c| c.text()), Some(" "));
        assert_eq!(row.get(6).map(|c| c.text()), Some(" "));
        assert_eq!(row.get(7).map(|c| c.text()), Some("X"));
    }

    #[test]
    fn test_row_contents() {
        let mut row = Row::new(80);

        for (i, c) in "Hello".chars().enumerate() {
            if let Some(cell) = row.get_mut(i as u16) {
                cell.set_text(c.to_string());
            }
        }

        let contents = row.contents_trimmed();
        assert_eq!(contents, "Hello");
    }

    #[test]
    fn test_row_wrapped() {
        let mut row = Row::new(80);
        assert!(!row.wrapped());

        row.set_wrapped(true);
        assert!(row.wrapped());
    }
}
