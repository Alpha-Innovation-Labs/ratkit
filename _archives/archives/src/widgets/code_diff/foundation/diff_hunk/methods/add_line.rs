use crate::widgets::code_diff::diff_hunk::DiffHunk;
use crate::widgets::code_diff::diff_line::DiffLine;

impl DiffHunk {
    /// Adds a diff line to this hunk.
    ///
    /// # Arguments
    ///
    /// * `line` - The diff line to add
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffHunk, DiffLine};
    ///
    /// let mut hunk = DiffHunk::new(1, 2, 1, 3);
    /// hunk.add_line(DiffLine::context("unchanged", 1, 1));
    /// hunk.add_line(DiffLine::added("new line", 2));
    /// ```
    pub fn add_line(&mut self, line: DiffLine) {
        self.lines.push(line);
    }
}
