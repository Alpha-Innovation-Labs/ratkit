//! Method to toggle sidebar visibility.

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Toggles the sidebar visibility.
    ///
    /// When toggled off, the entire area is used for the diff display.
    /// When toggled on, the file tree sidebar appears on the left.
    ///
    /// This method only has an effect when `config.sidebar_enabled` is true.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    ///
    /// let mut diff = CodeDiff::new()
    ///     .with_config(DiffConfig::new().sidebar_enabled(true));
    ///
    /// assert!(diff.show_sidebar);
    /// diff.toggle_sidebar();
    /// assert!(!diff.show_sidebar);
    /// ```
    pub fn toggle_sidebar(&mut self) {
        if self.config.sidebar_enabled {
            self.show_sidebar = !self.show_sidebar;
        }
    }
}
