//! Builder method for sidebar default width.

use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the default sidebar width as a percentage (0-100).
    ///
    /// This is the initial width when the widget is created.
    /// The width can be adjusted at runtime with resize methods.
    ///
    /// # Arguments
    ///
    /// * `width` - The default width as percentage (clamped to min/max)
    ///
    /// # Returns
    ///
    /// The modified configuration for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffConfig;
    ///
    /// let config = DiffConfig::new().sidebar_default_width(30);
    /// assert_eq!(config.sidebar_default_width, 30);
    /// ```
    #[must_use]
    pub fn sidebar_default_width(mut self, width: u16) -> Self {
        self.sidebar_default_width = width.clamp(self.sidebar_min_width, self.sidebar_max_width);
        self
    }
}
