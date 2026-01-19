//! Terminal cell representation

use crate::termtui::attrs::Attrs;
use ratatui::buffer::Cell as RatatuiCell;
use unicode_width::UnicodeWidthChar;

/// A single terminal cell
#[derive(Clone, Debug, Default)]
pub struct Cell {
    /// Cell text (may contain combining characters)
    text: String,
    /// Cell attributes
    attrs: Attrs,
}

impl Cell {
    /// Create a new empty cell
    pub fn new() -> Self {
        Self {
            text: " ".to_string(),
            attrs: Attrs::default(),
        }
    }

    /// Create a cell with specific attributes
    pub fn with_attrs(attrs: Attrs) -> Self {
        Self {
            text: " ".to_string(),
            attrs,
        }
    }

    /// Get the cell text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the cell text
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// Get the cell attributes
    pub fn attrs(&self) -> &Attrs {
        &self.attrs
    }

    /// Get mutable reference to attributes
    pub fn attrs_mut(&mut self) -> &mut Attrs {
        &mut self.attrs
    }

    /// Set the cell attributes
    pub fn set_attrs(&mut self, attrs: Attrs) {
        self.attrs = attrs;
    }

    /// Get the display width of this cell
    pub fn width(&self) -> usize {
        self.text
            .chars()
            .next()
            .and_then(|c| c.width())
            .unwrap_or(1)
    }

    /// Check if this cell is a wide character continuation
    /// (placeholder for second column of wide char)
    pub fn is_wide_continuation(&self) -> bool {
        self.text.is_empty()
    }

    /// Set this cell as a wide character continuation
    pub fn set_wide_continuation(&mut self) {
        self.text.clear();
    }

    /// Clear the cell (reset to space with default attrs)
    pub fn clear(&mut self) {
        self.text = " ".to_string();
        self.attrs = Attrs::default();
    }

    /// Clear the cell but keep attributes
    pub fn clear_keep_attrs(&mut self) {
        self.text = " ".to_string();
    }

    /// Convert to ratatui cell
    pub fn to_ratatui(&self) -> RatatuiCell {
        let mut cell = RatatuiCell::default();

        // Get the first character or space
        let ch = self.text.chars().next().unwrap_or(' ');
        cell.set_char(ch);
        cell.set_style(self.attrs.to_ratatui());

        cell
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.attrs == other.attrs
    }
}

impl Eq for Cell {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_new() {
        let cell = Cell::new();
        assert_eq!(cell.text(), " ");
        assert_eq!(cell.width(), 1);
    }

    #[test]
    fn test_cell_set_text() {
        let mut cell = Cell::new();
        cell.set_text("A");
        assert_eq!(cell.text(), "A");
    }

    #[test]
    fn test_cell_wide_char() {
        let mut cell = Cell::new();
        cell.set_text("你");
        assert_eq!(cell.width(), 2);
    }

    #[test]
    fn test_cell_clear() {
        let mut cell = Cell::new();
        cell.set_text("X");
        cell.attrs_mut().set_bold(true);

        cell.clear();
        assert_eq!(cell.text(), " ");
        assert!(!cell.attrs().bold());
    }

    #[test]
    fn test_cell_to_ratatui() {
        let mut cell = Cell::new();
        cell.set_text("A");
        cell.attrs_mut().set_bold(true);

        let ratatui_cell = cell.to_ratatui();
        assert_eq!(ratatui_cell.symbol(), "A");
    }
}
