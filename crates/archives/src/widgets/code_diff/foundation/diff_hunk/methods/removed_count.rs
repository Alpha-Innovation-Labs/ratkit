use crate::widgets::code_diff::diff_hunk::DiffHunk;

impl DiffHunk {
    /// Returns the number of removed lines in this hunk.
    ///
    /// # Returns
    ///
    /// The count of lines with `DiffLineKind::Removed`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffHunk, DiffLine};
    ///
    /// let mut hunk = DiffHunk::new(1, 3, 1, 2);
    /// hunk.add_line(DiffLine::removed("old line 1", 1));
    /// hunk.add_line(DiffLine::removed("old line 2", 2));
    /// assert_eq!(hunk.removed_count(), 2);
    /// ```
    pub fn removed_count(&self) -> usize {
        self.lines.iter().filter(|l| l.is_removed()).count()
    }
}
