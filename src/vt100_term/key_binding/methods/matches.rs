//! KeyBinding::matches method

use crate::vt100_term::key_binding::KeyBinding;

impl KeyBinding {
    /// Check if a key event matches this binding
    pub fn matches(&self, key: &crossterm::event::KeyEvent) -> bool {
        key.code == self.code && key.modifiers == self.modifiers
    }
}
