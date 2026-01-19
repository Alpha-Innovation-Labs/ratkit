//! is_navigate_up method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the navigate up key
    pub fn is_navigate_up(&self, key: &KeyEvent) -> bool {
        key.code == self.navigate_up.0 && key.modifiers == self.navigate_up.1
    }
}
