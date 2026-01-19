//! is_clear_selection method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the clear selection key
    pub fn is_clear_selection(&self, key: &KeyEvent) -> bool {
        key.code == self.clear_selection.0 && key.modifiers == self.clear_selection.1
    }
}
