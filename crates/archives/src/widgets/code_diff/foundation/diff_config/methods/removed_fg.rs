use ratatui::style::Color;

use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the foreground color for removed lines.
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
    /// let config = DiffConfig::new().removed_fg(Color::LightRed);
    /// ```
    pub fn removed_fg(mut self, color: Color) -> Self {
        self.removed_fg = color;
        self
    }
}
