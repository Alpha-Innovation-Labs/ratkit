//! TreeNavigator::select_previous_filtered method.

use crate::tree_view::helpers::get_visible_paths_filtered;
use crate::tree_view::tree_navigator::TreeNavigator;
use crate::tree_view::tree_node::TreeNode;
use crate::tree_view::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Selects the previous visible item with filtering support.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The node data type.
    /// * `F` - The filter matcher function type.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The tree nodes.
    /// * `state` - The tree view state to update.
    /// * `matcher` - A function that takes node data and filter, returns true if matches.
    pub fn select_previous_filtered<T, F>(
        &self,
        nodes: &[TreeNode<T>],
        state: &mut TreeViewState,
        matcher: F,
    ) where
        F: Fn(&T, &Option<String>) -> bool,
    {
        let visible_paths = get_visible_paths_filtered(nodes, state, matcher);
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
            state.select(visible_paths[0].clone());
        }
    }
}
