//! Filter-related methods for FileSystemTree.

use crate::file_system_tree::FileSystemTree;
use crate::primitives::tree_view::TreeViewState;

impl<'a> FileSystemTree<'a> {
    /// Enters filter mode, initializing an empty filter.
    ///
    /// When in filter mode, key presses are handled by `handle_filter_key`
    /// instead of normal navigation.
    ///
    /// # Arguments
    ///
    /// * `state` - The tree view state to modify
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::file_system_tree::FileSystemTree;
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let tree = FileSystemTree::new(".");
    /// let mut state = TreeViewState::new();
    /// tree.enter_filter_mode(&mut state);
    /// assert!(tree.is_filter_mode(&state));
    /// ```
    pub fn enter_filter_mode(&self, state: &mut TreeViewState) {
        state.enter_filter_mode();
    }

    /// Returns whether filter input mode is currently active.
    ///
    /// When `true`, key presses should be delegated to `handle_filter_key`.
    ///
    /// # Arguments
    ///
    /// * `state` - The tree view state to check
    ///
    /// # Returns
    ///
    /// `true` if filter mode is active, `false` otherwise.
    #[must_use]
    pub fn is_filter_mode(&self, state: &TreeViewState) -> bool {
        state.is_filter_mode()
    }

    /// Returns the current filter text.
    ///
    /// # Arguments
    ///
    /// * `state` - The tree view state to check
    ///
    /// # Returns
    ///
    /// The current filter text, or `None` if no filter is active.
    #[must_use]
    pub fn filter_text<'s>(&self, state: &'s TreeViewState) -> Option<&'s str> {
        state.filter_text()
    }

    /// Clears the filter and exits filter mode.
    ///
    /// This removes any active filter, showing all items again.
    ///
    /// # Arguments
    ///
    /// * `state` - The tree view state to modify
    pub fn clear_filter(&self, state: &mut TreeViewState) {
        state.clear_filter();
    }
}
