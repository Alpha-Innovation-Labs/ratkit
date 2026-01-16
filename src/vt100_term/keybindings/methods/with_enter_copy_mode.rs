//! VT100TermKeyBindings::with_enter_copy_mode builder method

use crate::vt100_term::key_binding::KeyBinding;
use crate::vt100_term::keybindings::VT100TermKeyBindings;

impl VT100TermKeyBindings {
    /// Set the key to enter copy mode
    pub fn with_enter_copy_mode(mut self, binding: KeyBinding) -> Self {
        self.enter_copy_mode = binding;
        self
    }
}
