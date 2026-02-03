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
