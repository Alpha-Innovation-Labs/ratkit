//! TreeView::node_at_row method.

use crate::primitives::tree_view::tree_view::TreeView;
use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl<'a, T> TreeView<'a, T> {
    /// Gets the node at a specific row (considering scroll offset).
    ///
    /// # Arguments
    ///
    /// * `state` - The tree view state.
    /// * `row` - The row index (0-based, relative to visible area).
    ///
    /// # Returns
    ///
    /// The path to the node at the given row, or `None` if out of bounds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView, TreeViewState};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeView::new(nodes);
    /// let state = TreeViewState::new();
    /// let path = tree.node_at_row(&state, 0);
    /// ```
    pub fn node_at_row(&self, state: &TreeViewState, row: usize) -> Option<Vec<usize>> {
        let items = self.flatten_tree(state);
        items.get(row + state.offset).map(|(_, path)| path.clone())
    }
}
