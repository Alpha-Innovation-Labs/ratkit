//! Column alignment for table cells.

/// Represents the alignment of a table column.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColumnAlignment {
    /// Default alignment (left).
    #[default]
    None,
    /// Left-aligned.
    Left,
    /// Center-aligned.
    Center,
    /// Right-aligned.
    Right,
}
