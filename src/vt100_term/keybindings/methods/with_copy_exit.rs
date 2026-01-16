//! VT100TermKeyBindings::with_copy_exit builder method

use crate::vt100_term::keybindings::VT100TermKeyBindings;
use crossterm::event::KeyCode;

impl VT100TermKeyBindings {
    /// Set the keys to exit copy mode
    pub fn with_copy_exit(mut self, keys: Vec<KeyCode>) -> Self {
        self.copy_exit = keys;
        self
    }
}
