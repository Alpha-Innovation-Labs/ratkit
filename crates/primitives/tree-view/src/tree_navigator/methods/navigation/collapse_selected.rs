//! TreeNavigator::collapse_selected method.

use crate::tree_navigator::TreeNavigator;
use crate::tree_node::TreeNode;
use crate::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Collapses the selected node or moves to parent.
    ///
    /// If the node is expanded, it collapses it.
    /// If the node is already collapsed, it moves to the parent.
    ///
    /// # Arguments
    ///
    /// * `_nodes` - The tree nodes (unused but kept for API consistency).
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
    /// state.expand(vec![0]);
    /// navigator.collapse_selected(&nodes, &mut state);
    /// assert!(!state.is_expanded(&[0]));
    /// ```
    pub fn collapse_selected<T>(&self, _nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        if let Some(path) = state.selected_path.clone() {
            if state.is_expanded(&path) {
                // Collapse current
                state.collapse(path);
            } else if path.len() > 1 {
                // Move to parent
                let parent = path[..path.len() - 1].to_vec();
                state.select(parent);
            }
        }
    }
}
