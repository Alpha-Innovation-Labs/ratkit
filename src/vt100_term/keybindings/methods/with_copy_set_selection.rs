//! VT100TermKeyBindings::with_copy_set_selection builder method

use crate::vt100_term::keybindings::VT100TermKeyBindings;
use crossterm::event::KeyCode;

impl VT100TermKeyBindings {
    /// Set the keys to set selection in copy mode
    pub fn with_copy_set_selection(mut self, keys: Vec<KeyCode>) -> Self {
        self.copy_set_selection = keys;
        self
    }
}
