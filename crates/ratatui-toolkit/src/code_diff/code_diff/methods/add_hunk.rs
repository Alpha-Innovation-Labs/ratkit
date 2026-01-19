use crate::code_diff::code_diff::CodeDiff;
use crate::code_diff::diff_hunk::DiffHunk;

impl CodeDiff {
    /// Adds a diff hunk to this widget.
    ///
    /// # Arguments
    ///
    /// * `hunk` - The diff hunk to add
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffHunk, DiffLine};
    ///
    /// let mut diff = CodeDiff::new();
    /// let mut hunk = DiffHunk::new(1, 2, 1, 3);
    /// hunk.add_line(DiffLine::context("unchanged", 1, 1));
    /// diff.add_hunk(hunk);
    /// ```
    pub fn add_hunk(&mut self, hunk: DiffHunk) {
        self.hunks.push(hunk);
    }
}
