//! TreeViewState::filter_text method.

use crate::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Gets the current filter text.
    ///
    /// # Returns
    ///
    /// The current filter text, or `None` if no filter is set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// assert!(state.filter_text().is_none());
    /// state.set_filter("test".to_string());
    /// assert_eq!(state.filter_text(), Some("test"));
    /// ```
    pub fn filter_text(&self) -> Option<&str> {
        self.filter.as_deref()
    }
}
