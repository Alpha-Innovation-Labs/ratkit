//! Method to toggle focus between sidebar and diff view.

use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Toggles focus between the sidebar and diff view.
    ///
    /// When sidebar is focused, navigation keys move through files.
    /// When diff view is focused, navigation keys scroll the diff.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    ///
    /// let mut diff = CodeDiff::new()
    ///     .with_config(DiffConfig::new().sidebar_enabled(true));
    ///
    /// assert!(diff.sidebar_focused);
    /// diff.toggle_focus();
    /// assert!(!diff.sidebar_focused);
    /// ```
    pub fn toggle_focus(&mut self) {
        self.sidebar_focused = !self.sidebar_focused;
        self.file_tree.focused = self.sidebar_focused;
    }
}
