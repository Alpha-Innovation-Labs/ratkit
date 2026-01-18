//! Position in the rendered text (visual coordinates).

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
