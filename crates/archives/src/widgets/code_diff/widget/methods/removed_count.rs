use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Returns the total number of removed lines across all hunks.
    ///
    /// # Returns
    ///
    /// The count of all removed lines
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let diff = CodeDiff::from_unified_diff("@@ -1,2 +1 @@\n-removed");
    /// assert_eq!(diff.removed_count(), 1);
    /// ```
    pub fn removed_count(&self) -> usize {
        self.hunks.iter().map(|h| h.removed_count()).sum()
    }
}
