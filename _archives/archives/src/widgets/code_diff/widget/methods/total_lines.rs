use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Returns the total number of lines to display.
    ///
    /// This includes all lines from all hunks plus hunk header lines.
    ///
    /// # Returns
    ///
    /// The total line count for rendering
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let diff = CodeDiff::new();
    /// assert_eq!(diff.total_lines(), 0);
    /// ```
    pub fn total_lines(&self) -> usize {
        self.hunks
            .iter()
            .map(|h| h.lines.len() + 1) // +1 for hunk header
            .sum()
    }
}
