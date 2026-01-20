//! TreeViewState::new constructor.

use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Creates a new tree view state with default values.
    ///
    /// # Returns
    ///
    /// A new `TreeViewState` with no selection, no expanded nodes,
    /// and no active filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let state = TreeViewState::new();
    /// assert!(state.selected_path.is_none());
    /// assert!(state.expanded.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}
