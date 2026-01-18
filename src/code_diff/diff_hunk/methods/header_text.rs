use crate::code_diff::diff_hunk::DiffHunk;

impl DiffHunk {
    /// Returns the formatted header text for this hunk.
    ///
    /// Generates a unified diff style header like `@@ -1,4 +1,5 @@ context`.
    ///
    /// # Returns
    ///
    /// The formatted header string
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffHunk;
    ///
    /// let mut hunk = DiffHunk::new(1, 4, 1, 5);
    /// assert_eq!(hunk.header_text(), "@@ -1,4 +1,5 @@");
    /// ```
    pub fn header_text(&self) -> String {
        let base = format!(
            "@@ -{},{} +{},{} @@",
            self.old_start, self.old_count, self.new_start, self.new_count
        );

        if let Some(ref ctx) = self.context {
            format!("{} {}", base, ctx)
        } else {
            base
        }
    }
}
