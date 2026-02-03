use ratatui::style::Color;

use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the background color for hunk header lines.
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
    /// let config = DiffConfig::new().hunk_header_bg(Color::DarkGray);
    /// ```
    pub fn hunk_header_bg(mut self, color: Color) -> Self {
        self.hunk_header_bg = color;
        self
    }
}
