//! KeyBinding::ctrl_shift constructor

use crate::vt100_term::key_binding::KeyBinding;
use crossterm::event::{KeyCode, KeyModifiers};

impl KeyBinding {
    /// Create a key binding with Ctrl+Shift modifiers
    pub fn ctrl_shift(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        }
    }
}
