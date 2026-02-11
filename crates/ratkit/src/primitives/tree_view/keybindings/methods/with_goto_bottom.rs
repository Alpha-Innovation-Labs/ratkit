use crossterm::event::KeyCode;

use crate::primitives::tree_view::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for goto bottom
    pub fn with_goto_bottom(mut self, keys: Vec<KeyCode>) -> Self {
        self.goto_bottom = keys;
        self
    }
}
