use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the number of context lines to show around changes.
    ///
    /// # Arguments
    ///
    /// * `lines` - Number of context lines
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
    /// let config = DiffConfig::new().context_lines(5);
    /// ```
    pub fn context_lines(mut self, lines: usize) -> Self {
        self.context_lines = lines;
        self
    }
}
