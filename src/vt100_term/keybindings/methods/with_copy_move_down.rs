//! VT100TermKeyBindings::with_copy_move_down builder method

use crate::vt100_term::keybindings::VT100TermKeyBindings;
use crossterm::event::KeyCode;

impl VT100TermKeyBindings {
    /// Set the keys to move down in copy mode
    pub fn with_copy_move_down(mut self, keys: Vec<KeyCode>) -> Self {
        self.copy_move_down = keys;
        self
    }
}
