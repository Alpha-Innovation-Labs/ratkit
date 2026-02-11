//! TreeNavigator::handle_key method.

use crossterm::event::KeyEvent;

use crate::primitives::tree_view::tree_navigator::TreeNavigator;
use crate::primitives::tree_view::tree_node::TreeNode;
use crate::primitives::tree_view::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Handles a key event and updates tree state.
    ///
    /// # Arguments
    ///
    /// * `key` - The key event to handle.
    /// * `nodes` - The tree nodes.
    /// * `state` - The tree view state to update.
    ///
    /// # Returns
    ///
    /// `true` if the key was handled, `false` otherwise.
    pub fn handle_key<T>(
        &self,
        key: KeyEvent,
        nodes: &[TreeNode<T>],
        state: &mut TreeViewState,
    ) -> bool {
        // Only handle key press events, not release
        if key.kind != crossterm::event::KeyEventKind::Press {
            return false;
        }

        let code = key.code;

        if self.keybindings.next.contains(&code) {
            self.select_next(nodes, state);
            true
        } else if self.keybindings.previous.contains(&code) {
            self.select_previous(nodes, state);
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
            self.goto_top(nodes, state);
            true
        } else if self.keybindings.goto_bottom.contains(&code) {
            self.goto_bottom(nodes, state);
            true
        } else {
            false
        }
    }
}
