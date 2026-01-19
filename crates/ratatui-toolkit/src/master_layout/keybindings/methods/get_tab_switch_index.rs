//! get_tab_switch_index method for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::KeyEvent;

impl MasterLayoutKeyBindings {
    /// Check if the given key event matches any tab switch key and return the tab index (0-based)
    pub fn get_tab_switch_index(&self, key: &KeyEvent) -> Option<usize> {
        for (i, (code, mods)) in self.switch_tabs.iter().enumerate() {
            if key.code == *code && key.modifiers == *mods {
                return Some(i);
            }
        }
        None
    }
}
