use ratatui::style::Color;

use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the foreground color for hunk header lines.
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
    /// let config = DiffConfig::new().hunk_header_fg(Color::Cyan);
    /// ```
    pub fn hunk_header_fg(mut self, color: Color) -> Self {
        self.hunk_header_fg = color;
        self
    }
}
