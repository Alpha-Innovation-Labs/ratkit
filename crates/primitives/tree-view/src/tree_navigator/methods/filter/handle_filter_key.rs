//! TreeNavigator::handle_filter_key method.

use crossterm::event::KeyCode;

use crate::tree_navigator::TreeNavigator;
use crate::tree_view_state::TreeViewState;

impl TreeNavigator {
    /// Handles a key code in filter mode.
    ///
    /// # Arguments
    ///
    /// * `key` - The key code to handle.
    /// * `state` - The tree view state to update.
    ///
    /// # Returns
    ///
    /// `true` if the key was handled, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crossterm::event::KeyCode;
    /// use ratatui_toolkit::tree_view::{TreeNavigator, TreeViewState};
    ///
    /// let navigator = TreeNavigator::new();
    /// let mut state = TreeViewState::new();
    /// state.enter_filter_mode();
    /// navigator.handle_filter_key(KeyCode::Char('t'), &mut state);
    /// assert_eq!(state.filter, Some("t".to_string()));
    /// ```
    pub fn handle_filter_key(&self, key: KeyCode, state: &mut TreeViewState) -> bool {
        match key {
            KeyCode::Esc => {
                state.exit_filter_mode();
                state.clear_filter();
                true
            }
            KeyCode::Enter => {
                state.exit_filter_mode();
                true
            }
            KeyCode::Backspace => {
                state.backspace_filter();
                true
            }
            KeyCode::Char(c) => {
                state.append_to_filter(c);
                true
            }
            _ => false,
        }
    }
}
