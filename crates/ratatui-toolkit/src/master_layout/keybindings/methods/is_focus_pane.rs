//! is_focus_pane method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches the focus pane key
    pub fn is_focus_pane(&self, key: &KeyEvent) -> bool {
        key.code == self.focus_pane.0 && key.modifiers == self.focus_pane.1
    }
}
