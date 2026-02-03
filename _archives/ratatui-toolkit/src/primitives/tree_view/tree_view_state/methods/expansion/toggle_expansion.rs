//! TreeViewState::toggle_expansion method.

use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Toggles expansion of a node at the given path.
    ///
    /// If the node is expanded, it will be collapsed.
    /// If the node is collapsed, it will be expanded.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the node to toggle.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.toggle_expansion(vec![0]);
    /// assert!(state.is_expanded(&[0]));
    /// state.toggle_expansion(vec![0]);
    /// assert!(!state.is_expanded(&[0]));
    /// ```
    pub fn toggle_expansion(&mut self, path: Vec<usize>) {
        if self.expanded.contains(&path) {
            self.expanded.remove(&path);
        } else {
            self.expanded.insert(path);
        }
    }
}
