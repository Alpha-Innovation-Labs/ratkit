use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Scrolls down by the specified number of lines.
    ///
    /// The scroll is clamped to the maximum scrollable position.
    ///
    /// # Arguments
    ///
    /// * `lines` - Number of lines to scroll down
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let mut diff = CodeDiff::new();
    /// diff.scroll_down(5);
    /// // scroll_offset is now 5 (or less if content is shorter)
    /// ```
    pub fn scroll_down(&mut self, lines: usize) {
        let max_scroll = self.total_lines();
        self.scroll_offset = (self.scroll_offset + lines).min(max_scroll);
    }
}
