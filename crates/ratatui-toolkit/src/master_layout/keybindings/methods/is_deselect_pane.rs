//! is_deselect_pane method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the deselect pane key
    pub fn is_deselect_pane(&self, key: &KeyEvent) -> bool {
        key.code == self.deselect_pane.0 && key.modifiers == self.deselect_pane.1
    }
}
