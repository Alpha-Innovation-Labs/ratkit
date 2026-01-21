use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Scrolls up by the specified number of lines.
    ///
    /// # Arguments
    ///
    /// * `lines` - Number of lines to scroll up
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let mut diff = CodeDiff::new();
    /// diff.scroll_offset = 10;
    /// diff.scroll_up(3);
    /// assert_eq!(diff.scroll_offset, 7);
    /// ```
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }
}
