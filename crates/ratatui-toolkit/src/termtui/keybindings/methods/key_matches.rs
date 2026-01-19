//! Key matching utility for keybindings

use crate::termtui::keybindings::TermTuiKeyBindings;
use crossterm::event::KeyEvent;

impl TermTuiKeyBindings {
    /// Check if a key event matches a keybinding (ignoring state field differences)
    pub fn key_matches(key: &KeyEvent, binding: &KeyEvent) -> bool {
        key.code == binding.code && key.modifiers == binding.modifiers
    }
}
