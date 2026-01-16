//! VT100TermKeyBindings::with_copy_selection builder method

use crate::vt100_term::key_binding::KeyBinding;
use crate::vt100_term::keybindings::VT100TermKeyBindings;

impl VT100TermKeyBindings {
    /// Set the key to copy selection
    pub fn with_copy_selection(mut self, binding: KeyBinding) -> Self {
        self.copy_selection = binding;
        self
    }
}
