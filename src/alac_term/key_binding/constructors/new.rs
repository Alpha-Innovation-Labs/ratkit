use crossterm::event::{KeyCode, KeyModifiers};

use crate::alac_term::key_binding::KeyBinding;

impl KeyBinding {
    /// Create a new key binding
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}
