//! TreeViewState::is_expanded method.

use crate::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Checks if a node is expanded.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the node to check.
    ///
    /// # Returns
    ///
    /// `true` if the node is expanded, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// assert!(!state.is_expanded(&[0]));
    /// state.expand(vec![0]);
    /// assert!(state.is_expanded(&[0]));
    /// ```
    pub fn is_expanded(&self, path: &[usize]) -> bool {
        self.expanded.contains(path)
    }
}
