use crossterm::event::KeyCode;

use crate::tree_view::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for expand
    pub fn with_expand(mut self, keys: Vec<KeyCode>) -> Self {
        self.expand = keys;
        self
    }
}
