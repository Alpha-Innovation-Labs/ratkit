use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the width of the gutter column.
    ///
    /// The gutter displays the +/- markers for each line.
    ///
    /// # Arguments
    ///
    /// * `width` - The gutter width in characters
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
    /// let config = DiffConfig::new().gutter_width(3);
    /// ```
    pub fn gutter_width(mut self, width: u16) -> Self {
        self.gutter_width = width;
        self
    }
}
