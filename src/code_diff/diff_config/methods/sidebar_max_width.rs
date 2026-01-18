//! Builder method for sidebar maximum width.

use crate::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the maximum sidebar width as a percentage (0-100).
    ///
    /// The sidebar cannot be resized larger than this value.
    ///
    /// # Arguments
    ///
    /// * `width` - The maximum width as percentage
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
    /// let config = DiffConfig::new().sidebar_max_width(40);
    /// assert_eq!(config.sidebar_max_width, 40);
    /// ```
    #[must_use]
    pub fn sidebar_max_width(mut self, width: u16) -> Self {
        self.sidebar_max_width = width.min(100);
        self
    }
}
