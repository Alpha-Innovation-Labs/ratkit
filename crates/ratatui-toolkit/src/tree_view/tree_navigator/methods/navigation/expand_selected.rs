//! TreeNavigator::expand_selected method.

use crate::tree_view::tree_navigator::TreeNavigator;
use crate::tree_view::tree_node::TreeNode;
use crate::tree_view::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Expands the selected node.
    ///
    /// Only expands if the node has children.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The tree nodes.
    /// * `state` - The tree view state to update.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNavigator, TreeNode, TreeViewState};
    ///
    /// let navigator = TreeNavigator::new();
    /// let child = TreeNode::new("Child");
    /// let nodes = vec![TreeNode::with_children("Parent", vec![child])];
    /// let mut state = TreeViewState::new();
    /// state.select(vec![0]);
    /// navigator.expand_selected(&nodes, &mut state);
    /// assert!(state.is_expanded(&[0]));
    /// ```
    pub fn expand_selected<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        if let Some(path) = state.selected_path.clone() {
            // Check if node has children
            if let Some(node) = self.get_node_at_path(nodes, &path) {
                if !node.children.is_empty() {
                    state.expand(path);
                }
            }
        }
    }
}
