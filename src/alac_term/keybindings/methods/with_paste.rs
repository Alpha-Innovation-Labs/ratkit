use crossterm::event::{KeyCode, KeyModifiers};

use crate::alac_term::key_binding::KeyBinding;
use crate::alac_term::keybindings::AlacTermKeyBindings;

impl AlacTermKeyBindings {
    /// Set the paste key binding
    pub fn with_paste(mut self, code: KeyCode, modifiers: KeyModifiers) -> Self {
        self.paste = KeyBinding::new(code, modifiers);
        self
    }
}
