//! Method to handle keyboard input while in filter mode.

use crossterm::event::KeyCode;

use super::super::super::diff_file_tree::DiffFileTree;
use super::super::super::primitives::tree_view::TreeNavigator;

impl DiffFileTree {
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
    ///
    /// # Returns
    ///
    /// `true` if the key was handled, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crossterm::event::KeyCode;
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::DiffFileTree;
    ///
    /// let mut tree = DiffFileTree::new();
    /// tree.enter_filter_mode();
    ///
    /// // Type a character
    /// tree.handle_filter_key(KeyCode::Char('r'));
    ///
    /// // Exit filter mode
    /// tree.handle_filter_key(KeyCode::Esc);
    /// ```
    pub fn handle_filter_key(&mut self, key: KeyCode) -> bool {
        let navigator = TreeNavigator::default();
        navigator.handle_filter_key(key, &mut self.state)
    }
}
