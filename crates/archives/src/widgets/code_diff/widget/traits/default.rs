use crate::widgets::code_diff::code_diff::CodeDiff;

impl Default for CodeDiff {
    /// Creates a default empty diff widget.
    ///
    /// # Returns
    ///
    /// A new `CodeDiff` instance with no hunks and default configuration
    fn default() -> Self {
        Self::new()
    }
}
