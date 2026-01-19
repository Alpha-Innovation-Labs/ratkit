//! TreeViewState::clear_selection method.

use crate::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Clears the current selection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.select(vec![0]);
    /// state.clear_selection();
    /// assert!(state.selected_path.is_none());
    /// ```
    pub fn clear_selection(&mut self) {
        self.selected_path = None;
    }
}
