//! TreeViewState::select method.

use crate::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Sets the selected node path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the node to select (indices from root).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.select(vec![0, 1, 2]);
    /// assert_eq!(state.selected_path, Some(vec![0, 1, 2]));
    /// ```
    pub fn select(&mut self, path: Vec<usize>) {
        self.selected_path = Some(path);
    }
}
