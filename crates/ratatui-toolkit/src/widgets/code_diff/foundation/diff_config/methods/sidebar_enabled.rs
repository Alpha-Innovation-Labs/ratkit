//! Builder method for enabling sidebar.

use crate::widgets::code_diff::diff_config::DiffConfig;

impl DiffConfig {
    /// Sets whether the sidebar file tree is enabled.
    ///
    /// When enabled, the `CodeDiff` widget will display a file tree sidebar
    /// showing all files in a multi-file diff. The `[` key toggles visibility.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable the sidebar
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
    /// let config = DiffConfig::new().sidebar_enabled(true);
    /// assert!(config.sidebar_enabled);
    /// ```
    #[must_use]
    pub fn sidebar_enabled(mut self, enabled: bool) -> Self {
        self.sidebar_enabled = enabled;
        self
    }
}
