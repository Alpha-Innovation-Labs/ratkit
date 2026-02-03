//! TreeNavigator::select_previous method.

use crate::primitives::tree_view::helpers::get_visible_paths;
use crate::primitives::tree_view::tree_navigator::TreeNavigator;
use crate::primitives::tree_view::tree_node::TreeNode;
use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Selects the previous visible item.
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
    /// navigator.select_previous(&nodes, &mut state);
    /// assert_eq!(state.selected_path, Some(vec![0]));
    /// ```
    pub fn select_previous<T>(&self, nodes: &[TreeNode<T>], state: &mut TreeViewState) {
        let visible_paths = get_visible_paths(nodes, state);
        if visible_paths.is_empty() {
            return;
        }

        if let Some(current_path) = &state.selected_path {
            if let Some(current_idx) = visible_paths.iter().position(|p| p == current_path) {
                if current_idx > 0 {
                    state.select(visible_paths[current_idx - 1].clone());
                }
            }
        } else {
            // Select first item
            state.select(visible_paths[0].clone());
        }
    }
}
