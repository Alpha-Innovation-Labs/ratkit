//! TreeViewState::clear_filter method.

use crate::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Clears the current filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.set_filter("test".to_string());
    /// state.clear_filter();
    /// assert!(state.filter.is_none());
    /// ```
    pub fn clear_filter(&mut self) {
        self.filter = None;
    }
}
