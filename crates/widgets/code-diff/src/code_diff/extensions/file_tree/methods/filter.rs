//! Filter-related methods for DiffFileTree.

use super::super::super::diff_file_tree::DiffFileTree;

impl DiffFileTree {
    /// Enters filter mode, initializing an empty filter.
    ///
    /// When in filter mode, key presses are handled by `handle_filter_key`
    /// instead of normal navigation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::DiffFileTree;
    ///
    /// let mut tree = DiffFileTree::new();
    /// tree.enter_filter_mode();
    /// assert!(tree.is_filter_mode());
    /// ```
    pub fn enter_filter_mode(&mut self) {
        self.state.enter_filter_mode();
    }

    /// Returns whether filter input mode is currently active.
    ///
    /// When `true`, key presses should be delegated to `handle_filter_key`.
    ///
    /// # Returns
    ///
    /// `true` if filter mode is active, `false` otherwise.
    #[must_use]
    pub fn is_filter_mode(&self) -> bool {
        self.state.is_filter_mode()
    }

    /// Returns the current filter text.
    ///
    /// # Returns
    ///
    /// The current filter text, or `None` if no filter is active.
    #[must_use]
    pub fn filter_text(&self) -> Option<&str> {
        self.state.filter_text()
    }

    /// Clears the filter and exits filter mode.
    ///
    /// This removes any active filter, showing all items again.
    pub fn clear_filter(&mut self) {
        self.state.clear_filter();
    }
}
