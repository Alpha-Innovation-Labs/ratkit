use ratatui::style::Color;

use crate::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the foreground color for added lines.
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
    /// let config = DiffConfig::new().added_fg(Color::LightGreen);
    /// ```
    pub fn added_fg(mut self, color: Color) -> Self {
        self.added_fg = color;
        self
    }
}
