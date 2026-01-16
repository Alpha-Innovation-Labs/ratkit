use crossterm::event::{KeyCode, KeyModifiers};

use crate::alac_term::key_binding::KeyBinding;
use crate::alac_term::keybindings::AlacTermKeyBindings;

impl AlacTermKeyBindings {
    /// Set the clear selection key binding
    pub fn with_clear_selection(mut self, code: KeyCode, modifiers: KeyModifiers) -> Self {
        self.clear_selection = KeyBinding::new(code, modifiers);
        self
    }
}
