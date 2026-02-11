//! TreeViewState::backspace_filter method.

use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Removes the last character from the filter text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.set_filter("test".to_string());
    /// state.backspace_filter();
    /// assert_eq!(state.filter, Some("tes".to_string()));
    /// ```
    pub fn backspace_filter(&mut self) {
        if let Some(ref mut filter) = self.filter {
            filter.pop();
        }
    }
}
