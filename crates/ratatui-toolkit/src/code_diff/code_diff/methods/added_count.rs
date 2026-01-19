use crate::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Returns the total number of added lines across all hunks.
    ///
    /// # Returns
    ///
    /// The count of all added lines
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let diff = CodeDiff::from_unified_diff("@@ -1 +1,2 @@\n+added");
    /// assert_eq!(diff.added_count(), 1);
    /// ```
    pub fn added_count(&self) -> usize {
        self.hunks.iter().map(|h| h.added_count()).sum()
    }
}
