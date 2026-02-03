//! TreeViewState::collapse method.

use crate::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Collapses a node at the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the node to collapse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.expand(vec![0]);
    /// state.collapse(vec![0]);
    /// assert!(!state.is_expanded(&[0]));
    /// ```
    pub fn collapse(&mut self, path: Vec<usize>) {
        self.expanded.remove(&path);
    }
}
