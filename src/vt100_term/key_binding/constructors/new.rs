//! KeyBinding::new constructor

use crate::vt100_term::key_binding::KeyBinding;
use crossterm::event::{KeyCode, KeyModifiers};

impl KeyBinding {
    /// Create a new key binding with modifiers
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}
