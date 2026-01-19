//! is_navigate_right method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the navigate right key
    pub fn is_navigate_right(&self, key: &KeyEvent) -> bool {
        key.code == self.navigate_right.0 && key.modifiers == self.navigate_right.1
    }
}
