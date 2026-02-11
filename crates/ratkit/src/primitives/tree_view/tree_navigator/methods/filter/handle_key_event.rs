//! TreeNavigator::handle_key_event method.

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
    ///
    /// # Example
    ///
    /// ```rust
    /// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// use ratatui_toolkit::tree_view::{TreeNavigator, TreeNode, TreeViewState};
    ///
    /// let navigator = TreeNavigator::new();
    /// let nodes = vec![TreeNode::new("Item")];
    /// let mut state = TreeViewState::new();
    /// let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
    /// let handled = navigator.handle_key_event(key, &nodes, &mut state);
    /// ```
    pub fn handle_key_event<T>(
        &self,
        key: KeyEvent,
        nodes: &[TreeNode<T>],
        state: &mut TreeViewState,
    ) -> bool {
        self.handle_key(key, nodes, state)
    }
}
