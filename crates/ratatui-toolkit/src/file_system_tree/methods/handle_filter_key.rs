//! Method to handle keyboard input while in filter mode.

use crossterm::event::KeyCode;

use crate::file_system_tree::FileSystemTree;
use crate::primitives::tree_view::{TreeNavigator, TreeViewState};

impl<'a> FileSystemTree<'a> {
    /// Handles a key press while in filter mode.
    ///
    /// Delegates to [`TreeNavigator::handle_filter_key`] for consistent
    /// filter handling across all tree components.
    ///
    /// # Key Bindings
    ///
    /// - `Esc` - Clear filter and exit filter mode
    /// - `Enter` - Exit filter mode (keep filter active)
    /// - `Backspace` - Delete last character from filter
    /// - Any character - Append to filter
    ///
    /// # Arguments
    ///
    /// * `key` - The key code that was pressed
    /// * `state` - The tree view state to modify
    ///
    /// # Returns
    ///
    /// `true` if the key was handled, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crossterm::event::KeyCode;
    /// use ratatui_toolkit::file_system_tree::FileSystemTree;
    /// use ratatui_toolkit::tree_view::TreeViewState;
    ///
    /// let tree = FileSystemTree::new(".");
    /// let mut state = TreeViewState::new();
    /// tree.enter_filter_mode(&mut state);
    ///
    /// // Type a character
    /// tree.handle_filter_key(KeyCode::Char('r'), &mut state);
    ///
    /// // Exit filter mode
    /// tree.handle_filter_key(KeyCode::Esc, &mut state);
    /// ```
    pub fn handle_filter_key(&self, key: KeyCode, state: &mut TreeViewState) -> bool {
        let navigator = TreeNavigator::default();
        navigator.handle_filter_key(key, state)
    }
}
