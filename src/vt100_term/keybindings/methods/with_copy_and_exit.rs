//! VT100TermKeyBindings::with_copy_and_exit builder method

use crate::vt100_term::keybindings::VT100TermKeyBindings;
use crossterm::event::KeyCode;

impl VT100TermKeyBindings {
    /// Set the keys to copy and exit copy mode
    pub fn with_copy_and_exit(mut self, keys: Vec<KeyCode>) -> Self {
        self.copy_and_exit = keys;
        self
    }
}
