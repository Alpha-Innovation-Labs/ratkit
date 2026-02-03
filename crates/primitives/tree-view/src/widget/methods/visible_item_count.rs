//! TreeView::visible_item_count method.

use crate::tree_view_state::TreeViewState;
use crate::widget::TreeView;

impl<'a, T> TreeView<'a, T> {
    /// Gets total visible item count.
    ///
    /// # Arguments
    ///
    /// * `state` - The tree view state.
    ///
    /// # Returns
    ///
    /// The number of visible items in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView, TreeViewState};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeView::new(nodes);
    /// let state = TreeViewState::new();
    /// assert_eq!(tree.visible_item_count(&state), 1);
    /// ```
    pub fn visible_item_count(&self, state: &TreeViewState) -> usize {
        self.flatten_tree(state).len()
    }
}
