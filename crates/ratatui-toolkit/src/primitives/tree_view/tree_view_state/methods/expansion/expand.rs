//! TreeViewState::expand method.

use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Expands a node at the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the node to expand.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.expand(vec![0]);
    /// assert!(state.is_expanded(&[0]));
    /// ```
    pub fn expand(&mut self, path: Vec<usize>) {
        self.expanded.insert(path);
    }
}
