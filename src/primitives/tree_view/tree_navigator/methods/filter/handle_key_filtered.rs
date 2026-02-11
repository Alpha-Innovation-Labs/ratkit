//! TreeNavigator::handle_key_filtered method.

use crossterm::event::KeyEvent;

use crate::primitives::tree_view::tree_navigator::TreeNavigator;
use crate::primitives::tree_view::tree_node::TreeNode;
use crate::primitives::tree_view::tree_view_state::TreeViewState;

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
    pub fn handle_key_filtered<T, F>(
        &self,
        key: KeyEvent,
        nodes: &[TreeNode<T>],
        state: &mut TreeViewState,
        matcher: F,
    ) -> bool
    where
        F: Fn(&T, &Option<String>) -> bool,
    {
        // Only handle key press events, not release
        if key.kind != crossterm::event::KeyEventKind::Press {
            return false;
        }

        let code = key.code;

        if self.keybindings.next.contains(&code) {
            self.select_next_filtered(nodes, state, matcher);
            true
        } else if self.keybindings.previous.contains(&code) {
            self.select_previous_filtered(nodes, state, |data, filter| matcher(data, filter));
            true
        } else if self.keybindings.expand.contains(&code) {
            self.expand_selected(nodes, state);
            true
        } else if self.keybindings.collapse.contains(&code) {
            self.collapse_selected(nodes, state);
            true
        } else if self.keybindings.toggle.contains(&code) {
            self.toggle_selected(nodes, state);
            true
        } else if self.keybindings.goto_top.contains(&code) {
            self.goto_top_filtered(nodes, state, |data, filter| matcher(data, filter));
            true
        } else if self.keybindings.goto_bottom.contains(&code) {
            self.goto_bottom_filtered(nodes, state, |data, filter| matcher(data, filter));
            true
        } else {
            false
        }
    }
}
