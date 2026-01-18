//! TreeNavigator::goto_bottom_filtered method.

use crate::tree_view::helpers::get_visible_paths_filtered;
use crate::tree_view::tree_navigator::TreeNavigator;
use crate::tree_view::tree_node::TreeNode;
use crate::tree_view::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Goes to the last visible item with filtering support.
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
    pub fn goto_bottom_filtered<T, F>(
        &self,
        nodes: &[TreeNode<T>],
        state: &mut TreeViewState,
        matcher: F,
    ) where
        F: Fn(&T, &Option<String>) -> bool,
    {
        let visible_paths = get_visible_paths_filtered(nodes, state, matcher);
        if !visible_paths.is_empty() {
            state.select(visible_paths[visible_paths.len() - 1].clone());
        }
    }
}
