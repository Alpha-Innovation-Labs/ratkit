use crossterm::event::KeyCode;

use crate::tree_view::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for toggle
    pub fn with_toggle(mut self, keys: Vec<KeyCode>) -> Self {
        self.toggle = keys;
        self
    }
}
