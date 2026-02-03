//! TreeNavigator::goto_top method.

use crate::helpers::get_visible_paths;
use crate::tree_navigator::TreeNavigator;
use crate::tree_node::TreeNode;
use crate::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Goes to the first visible item.
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
    /// state.select(vec![1]);
    /// navigator.goto_top(&nodes, &mut state);
    /// assert_eq!(state.selected_path, Some(vec![0]));
    /// ```
    pub fn goto_top<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        let visible_paths = get_visible_paths(nodes, state);
        if !visible_paths.is_empty() {
            state.select(visible_paths[0].clone());
        }
    }
}
