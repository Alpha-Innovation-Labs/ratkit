use crossterm::event::KeyCode;

use crate::keybindings::TreeKeyBindings;

impl TreeKeyBindings {
    /// Set custom keybindings for goto top
    pub fn with_goto_top(mut self, keys: Vec<KeyCode>) -> Self {
        self.goto_top = keys;
        self
    }
}
