//! KeyBinding::key constructor

use crate::vt100_term::key_binding::KeyBinding;
use crossterm::event::{KeyCode, KeyModifiers};

impl KeyBinding {
    /// Create a key binding with no modifiers
    pub fn key(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }
}
