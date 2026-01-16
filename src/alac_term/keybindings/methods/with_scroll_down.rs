use crossterm::event::{KeyCode, KeyModifiers};

use crate::alac_term::key_binding::KeyBinding;
use crate::alac_term::keybindings::AlacTermKeyBindings;

impl AlacTermKeyBindings {
    /// Set the scroll down key binding
    pub fn with_scroll_down(mut self, code: KeyCode, modifiers: KeyModifiers) -> Self {
        self.scroll_down = KeyBinding::new(code, modifiers);
        self
    }
}
