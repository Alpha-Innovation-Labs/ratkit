mod constructors;
mod methods;

use crossterm::event::{KeyCode, KeyModifiers};

/// A key binding definition consisting of a key code and modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyBinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}
