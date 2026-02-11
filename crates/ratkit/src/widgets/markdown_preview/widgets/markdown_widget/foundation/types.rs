//! Common types for the markdown widget.

//! Git statistics for display in the markdown widget statusline.

/// Git statistics for display in statusline.
#[derive(Debug, Clone, Copy, Default)]
pub struct GitStats {
    /// Lines added.
    pub additions: usize,
    /// Files modified (or lines modified depending on context).
    pub modified: usize,
    /// Lines deleted.
    pub deletions: usize,
}

/// Position in the rendered text (visual coordinates).

/// Position in the rendered text (visual coordinates).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectionPos {
    /// X coordinate (column).
    pub x: i32,
    /// Y coordinate (row, relative to document start, not screen).
    pub y: i32,
}

impl SelectionPos {
    /// Create a new position.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
