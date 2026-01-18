use crate::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Sets the file path for this diff.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to display in the header
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    ///
    /// let diff = CodeDiff::new().with_file_path("src/main.rs");
    /// ```
    pub fn with_file_path(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }
}
