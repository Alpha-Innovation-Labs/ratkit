//! TreeViewState::exit_filter_mode method.

use crate::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Exits filter mode.
    ///
    /// Sets `filter_mode` to `false` but preserves the current filter text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.enter_filter_mode();
    /// state.set_filter("test".to_string());
    /// state.exit_filter_mode();
    /// assert!(!state.filter_mode);
    /// assert_eq!(state.filter, Some("test".to_string()));
    /// ```
    pub fn exit_filter_mode(&mut self) {
        self.filter_mode = false;
    }
}
