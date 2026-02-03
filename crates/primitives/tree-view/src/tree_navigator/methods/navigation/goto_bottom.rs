//! TreeNavigator::goto_bottom method.

use crate::helpers::get_visible_paths;
use crate::tree_navigator::TreeNavigator;
use crate::tree_node::TreeNode;
use crate::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Goes to the last visible item.
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
    /// let nodes = vec![TreeNode::new("Item1"), TreeNode::new("Item2")];
    /// let mut state = TreeViewState::new();
    /// navigator.goto_bottom(&nodes, &mut state);
    /// assert_eq!(state.selected_path, Some(vec![1]));
    /// ```
    pub fn goto_bottom<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        let visible_paths = get_visible_paths(nodes, state);
        if !visible_paths.is_empty() {
            state.select(visible_paths[visible_paths.len() - 1].clone());
        }
    }
}
