use crate::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Returns a formatted stats string (e.g., "+5 -3").
    ///
    /// # Returns
    ///
    /// A string showing the added and removed line counts
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffHunk, DiffLine};
    ///
    /// let mut diff = CodeDiff::new();
    /// let mut hunk = DiffHunk::new(1, 1, 1, 2);
    /// hunk.add_line(DiffLine::removed("old", 1));
    /// hunk.add_line(DiffLine::added("new1", 1));
    /// hunk.add_line(DiffLine::added("new2", 2));
    /// diff.add_hunk(hunk);
    /// assert_eq!(diff.stats_text(), "+2 -1");
    /// ```
    pub fn stats_text(&self) -> String {
        format!("+{} -{}", self.added_count(), self.removed_count())
    }
}
