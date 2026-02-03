use crossterm::event::KeyCode;

use crate::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for next item
    pub fn with_next(mut self, keys: Vec<KeyCode>) -> Self {
        self.next = keys;
        self
    }
}
