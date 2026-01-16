//! is_navigate_left method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the navigate left key
    pub fn is_navigate_left(&self, key: &KeyEvent) -> bool {
        key.code == self.navigate_left.0 && key.modifiers == self.navigate_left.1
    }
}
