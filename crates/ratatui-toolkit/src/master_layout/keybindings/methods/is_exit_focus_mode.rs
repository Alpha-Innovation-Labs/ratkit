//! is_exit_focus_mode method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the exit focus mode key
    pub fn is_exit_focus_mode(&self, key: &KeyEvent) -> bool {
        key.code == self.exit_focus_mode.0 && key.modifiers == self.exit_focus_mode.1
    }
}
