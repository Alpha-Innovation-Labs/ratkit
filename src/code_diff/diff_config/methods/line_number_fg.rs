use ratatui::style::Color;

use crate::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the foreground color for line numbers.
    ///
    /// # Arguments
    ///
    /// * `color` - The foreground color to use
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_toolkit::code_diff::DiffConfig;
    ///
    /// let config = DiffConfig::new().line_number_fg(Color::Gray);
    /// ```
    pub fn line_number_fg(mut self, color: Color) -> Self {
        self.line_number_fg = color;
        self
    }
}
