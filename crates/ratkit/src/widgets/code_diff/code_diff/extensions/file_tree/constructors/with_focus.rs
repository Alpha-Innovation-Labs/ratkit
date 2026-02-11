//! Builder method for setting focus state.

use super::super::super::diff_file_tree::DiffFileTree;

impl DiffFileTree {
    /// Sets the focus state and returns self for chaining.
    ///
    /// This is a builder method that can be chained after constructors.
    ///
    /// # Arguments
    ///
    /// * `focused` - Whether the tree should have focus
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::{DiffFileTree, FileStatus};
    ///
    /// let tree = DiffFileTree::from_paths(&[("src/lib.rs", FileStatus::Modified)])
    ///     .with_focus(true);
    /// ```
    #[must_use]
    pub fn with_focus(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}
