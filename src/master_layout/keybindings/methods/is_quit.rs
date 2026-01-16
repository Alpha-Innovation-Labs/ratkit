//! is_quit method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches any of the quit keys
    pub fn is_quit(&self, key: &KeyEvent) -> bool {
        self.quit
            .iter()
            .any(|(code, mods)| key.code == *code && key.modifiers == *mods)
    }
}
