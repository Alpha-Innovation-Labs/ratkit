//! Key binding with optional modifiers

mod constructors;
mod methods;

use crossterm::event::{KeyCode, KeyModifiers};

/// A key binding with optional modifiers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}
