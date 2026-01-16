//! VT100TermKeyBindings::with_copy_move_right builder method

use crate::vt100_term::keybindings::VT100TermKeyBindings;
use crossterm::event::KeyCode;

impl VT100TermKeyBindings {
    /// Set the keys to move right in copy mode
    pub fn with_copy_move_right(mut self, keys: Vec<KeyCode>) -> Self {
        self.copy_move_right = keys;
        self
    }
}
