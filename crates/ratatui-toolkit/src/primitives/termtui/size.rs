//! Terminal size representation

/// Terminal dimensions
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct Size {
    /// Number of columns (width)
    pub cols: u16,
    /// Number of rows (height)
    pub rows: u16,
}

impl Size {
    /// Create a new size
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}
