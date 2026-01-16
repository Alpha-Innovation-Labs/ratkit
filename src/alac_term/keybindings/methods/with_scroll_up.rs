use crossterm::event::{KeyCode, KeyModifiers};

use crate::alac_term::key_binding::KeyBinding;
use crate::alac_term::keybindings::AlacTermKeyBindings;

impl AlacTermKeyBindings {
    /// Set the scroll up key binding
    pub fn with_scroll_up(mut self, code: KeyCode, modifiers: KeyModifiers) -> Self {
        self.scroll_up = KeyBinding::new(code, modifiers);
        self
    }
}
