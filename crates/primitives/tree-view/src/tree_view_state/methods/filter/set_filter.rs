//! TreeViewState::set_filter method.

use crate::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Sets the filter text.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter text to set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.set_filter("test".to_string());
    /// assert_eq!(state.filter, Some("test".to_string()));
    /// ```
    pub fn set_filter(&mut self, filter: String) {
        self.filter = Some(filter);
    }
}
