use crossterm::event::KeyCode;

use crate::primitives::tree_view::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for collapse
    pub fn with_collapse(mut self, keys: Vec<KeyCode>) -> Self {
        self.collapse = keys;
        self
    }
}
