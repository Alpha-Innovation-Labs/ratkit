use crate::widgets::code_diff::diff_hunk::DiffHunk;

impl DiffHunk {
    /// Returns the number of added lines in this hunk.
    ///
    /// # Returns
    ///
    /// The count of lines with `DiffLineKind::Added`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffHunk, DiffLine};
    ///
    /// let mut hunk = DiffHunk::new(1, 2, 1, 3);
    /// hunk.add_line(DiffLine::added("new line 1", 1));
    /// hunk.add_line(DiffLine::added("new line 2", 2));
    /// assert_eq!(hunk.added_count(), 2);
    /// ```
    pub fn added_count(&self) -> usize {
        self.lines.iter().filter(|l| l.is_added()).count()
    }
}
