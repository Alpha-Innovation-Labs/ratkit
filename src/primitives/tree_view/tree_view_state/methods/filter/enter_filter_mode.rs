//! TreeViewState::enter_filter_mode method.

use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Enters filter mode.
    ///
    /// Sets `filter_mode` to `true` and initializes the filter
    /// to an empty string if not already set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.enter_filter_mode();
    /// assert!(state.filter_mode);
    /// assert_eq!(state.filter, Some(String::new()));
    /// ```
    pub fn enter_filter_mode(&mut self) {
        self.filter_mode = true;
        if self.filter.is_none() {
            self.filter = Some(String::new());
        }
    }
}
