//! TreeNavigator::handle_key_event_filtered method.

use crossterm::event::KeyEvent;

use crate::tree_view::tree_navigator::TreeNavigator;
use crate::tree_view::tree_node::TreeNode;
use crate::tree_view::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Handles a key event with filtering support.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The node data type.
    /// * `F` - The filter matcher function type.
    ///
    /// # Arguments
    ///
    /// * `key` - The key event to handle.
    /// * `nodes` - The tree nodes.
    /// * `state` - The tree view state to update.
    /// * `matcher` - A function that takes node data and filter, returns true if matches.
    ///
    /// # Returns
    ///
    /// `true` if the key was handled, `false` otherwise.
    pub fn handle_key_event_filtered<T, F>(
        &self,
        key: KeyEvent,
        nodes: &[TreeNode<T>],
        state: &mut TreeViewState,
        matcher: F,
    ) -> bool
    where
        F: Fn(&T, &Option<String>) -> bool,
    {
        self.handle_key_filtered(key, nodes, state, matcher)
    }
}
