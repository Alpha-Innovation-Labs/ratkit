use crossterm::event::KeyCode;

use crate::tree_view::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for previous item
    pub fn with_previous(mut self, keys: Vec<KeyCode>) -> Self {
        self.previous = keys;
        self
    }
}
