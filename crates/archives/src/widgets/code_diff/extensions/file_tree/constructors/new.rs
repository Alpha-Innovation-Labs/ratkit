//! Constructor for creating an empty DiffFileTree.

use crate::primitives::tree_view::TreeViewState;
use crate::services::theme::AppTheme;
use crate::widgets::code_diff::diff_file_tree::DiffFileTree;

impl DiffFileTree {
    /// Creates a new empty `DiffFileTree`.
    ///
    /// # Returns
    ///
    /// An empty tree with no files.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::DiffFileTree;
    ///
    /// let tree = DiffFileTree::new();
    /// assert!(tree.nodes.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            state: TreeViewState::new(),
            selected_index: 0,
            focused: false,
            theme: AppTheme::default(),
        }
    }
}

impl Default for DiffFileTree {
    fn default() -> Self {
        Self::new()
    }
}
