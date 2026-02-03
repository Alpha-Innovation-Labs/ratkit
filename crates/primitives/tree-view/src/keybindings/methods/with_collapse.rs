use crossterm::event::KeyCode;

use crate::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for collapse
    pub fn with_collapse(mut self, keys: Vec<KeyCode>) -> Self {
        self.collapse = keys;
        self
    }
}
