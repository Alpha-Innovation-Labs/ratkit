//! Builder method for sidebar minimum width.

use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets the minimum sidebar width as a percentage (0-100).
    ///
    /// The sidebar cannot be resized smaller than this value.
    ///
    /// # Arguments
    ///
    /// * `width` - The minimum width as percentage
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
    /// let config = DiffConfig::new().sidebar_min_width(15);
    /// assert_eq!(config.sidebar_min_width, 15);
    /// ```
    #[must_use]
    pub fn sidebar_min_width(mut self, width: u16) -> Self {
        self.sidebar_min_width = width.min(100);
        self
    }
}
