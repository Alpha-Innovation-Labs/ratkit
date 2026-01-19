use ratatui::style::Color;

use crate::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the background color for removed lines.
    ///
    /// # Arguments
    ///
    /// * `color` - The background color to use
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
    /// let config = DiffConfig::new().removed_bg(Color::Red);
    /// ```
    pub fn removed_bg(mut self, color: Color) -> Self {
        self.removed_bg = color;
        self
    }
}
