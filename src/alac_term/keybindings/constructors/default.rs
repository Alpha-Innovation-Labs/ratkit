use crossterm::event::{KeyCode, KeyModifiers};

use crate::alac_term::key_binding::KeyBinding;
use crate::alac_term::keybindings::AlacTermKeyBindings;

impl Default for AlacTermKeyBindings {
    fn default() -> Self {
        Self {
            copy_selection: KeyBinding::new(
                KeyCode::Char('c'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),
            clear_selection: KeyBinding::new(KeyCode::Esc, KeyModifiers::NONE),
            paste: KeyBinding::new(
                KeyCode::Char('v'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),
            scroll_up: KeyBinding::new(KeyCode::PageUp, KeyModifiers::SHIFT),
            scroll_down: KeyBinding::new(KeyCode::PageDown, KeyModifiers::SHIFT),
        }
    }
}
