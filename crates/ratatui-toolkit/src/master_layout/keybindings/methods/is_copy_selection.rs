//! is_copy_selection method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the copy selection key
    pub fn is_copy_selection(&self, key: &KeyEvent) -> bool {
        key.code == self.copy_selection.0 && key.modifiers.contains(self.copy_selection.1)
    }
}
