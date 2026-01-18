//! Constructor for creating an empty DiffFileTree.

use crate::diff_file_tree::DiffFileTree;
use crate::services::theme::AppTheme;
use crate::tree_view::TreeViewState;

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
    /// use ratatui_toolkit::diff_file_tree::DiffFileTree;
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
