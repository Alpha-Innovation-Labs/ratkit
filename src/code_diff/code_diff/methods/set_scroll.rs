use crate::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Sets the scroll offset to a specific value.
    ///
    /// The offset is clamped to valid bounds.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset to set
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let mut diff = CodeDiff::new();
    /// diff.set_scroll(10);
    /// ```
    pub fn set_scroll(&mut self, offset: usize) {
        let max_scroll = self.total_lines();
        self.scroll_offset = offset.min(max_scroll);
    }
}
