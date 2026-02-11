use crossterm::event::KeyCode;

use crate::primitives::tree_view::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for expand
    pub fn with_expand(mut self, keys: Vec<KeyCode>) -> Self {
        self.expand = keys;
        self
    }
}
