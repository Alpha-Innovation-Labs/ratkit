//! TreeViewState::collapse_all method.

use crate::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Collapses all nodes in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.expand(vec![0]);
    /// state.expand(vec![0, 1]);
    /// state.collapse_all();
    /// assert!(state.expanded.is_empty());
    /// ```
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }
}
