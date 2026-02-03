use ratatui::style::Color;

use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the background color for added lines.
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
    /// let config = DiffConfig::new().added_bg(Color::Green);
    /// ```
    pub fn added_bg(mut self, color: Color) -> Self {
        self.added_bg = color;
        self
    }
}
