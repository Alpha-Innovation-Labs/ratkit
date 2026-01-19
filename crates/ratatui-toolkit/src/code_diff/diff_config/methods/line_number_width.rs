use crate::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the width of line number columns.
    ///
    /// # Arguments
    ///
    /// * `width` - The line number column width in characters
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffConfig;
    ///
    /// let config = DiffConfig::new().line_number_width(6);
    /// ```
    pub fn line_number_width(mut self, width: u16) -> Self {
        self.line_number_width = width;
        self
    }
}
