//! TreeViewState::append_to_filter method.

use crate::tree_view::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Appends a character to the filter text.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to append.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let mut state = TreeViewState::new();
    /// state.enter_filter_mode();
    /// state.append_to_filter('t');
    /// state.append_to_filter('e');
    /// assert_eq!(state.filter, Some("te".to_string()));
    /// ```
    pub fn append_to_filter(&mut self, c: char) {
        if let Some(ref mut filter) = self.filter {
            filter.push(c);
        } else {
            self.filter = Some(c.to_string());
        }
    }
}
