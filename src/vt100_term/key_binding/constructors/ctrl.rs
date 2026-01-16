//! KeyBinding::ctrl constructor

use crate::vt100_term::key_binding::KeyBinding;
use crossterm::event::{KeyCode, KeyModifiers};

impl KeyBinding {
    /// Create a key binding with Ctrl modifier
    pub fn ctrl(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::CONTROL,
        }
    }
}
