use crate::widgets::code_diff::diff_hunk::DiffHunk;

impl DiffHunk {
    /// Creates a new diff hunk with the specified line ranges.
    ///
    /// # Arguments
    ///
    /// * `old_start` - Starting line number in the old file
    /// * `old_count` - Number of lines from the old file
    /// * `new_start` - Starting line number in the new file
    /// * `new_count` - Number of lines from the new file
    ///
    /// # Returns
    ///
    /// A new `DiffHunk` with empty lines and no context
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffHunk;
    ///
    /// let hunk = DiffHunk::new(1, 4, 1, 5);
    /// assert_eq!(hunk.old_start, 1);
    /// assert_eq!(hunk.old_count, 4);
    /// ```
    pub fn new(old_start: usize, old_count: usize, new_start: usize, new_count: usize) -> Self {
        Self {
            old_start,
            old_count,
            new_start,
            new_count,
            context: None,
            lines: Vec::new(),
        }
    }
}
