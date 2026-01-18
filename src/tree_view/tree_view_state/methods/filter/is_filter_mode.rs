//! TreeViewState::is_filter_mode method.

use crate::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Checks if filter mode is active.
    ///
    /// # Returns
    ///
    /// `true` if filter mode is active, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// assert!(!state.is_filter_mode());
    /// state.enter_filter_mode();
    /// assert!(state.is_filter_mode());
    /// ```
    pub fn is_filter_mode(&self) -> bool {
        self.filter_mode
    }
}
