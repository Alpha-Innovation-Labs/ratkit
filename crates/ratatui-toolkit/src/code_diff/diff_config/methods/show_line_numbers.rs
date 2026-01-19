use crate::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets whether to display line numbers.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show line numbers
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
    /// let config = DiffConfig::new().show_line_numbers(false);
    /// ```
    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }
}
