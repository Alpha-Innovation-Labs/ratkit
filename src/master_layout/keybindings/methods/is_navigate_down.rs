//! is_navigate_down method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the navigate down key
    pub fn is_navigate_down(&self, key: &KeyEvent) -> bool {
        key.code == self.navigate_down.0 && key.modifiers == self.navigate_down.1
    }
}
