//! VT100TermKeyBindings::new constructor

use crate::vt100_term::keybindings::VT100TermKeyBindings;

impl VT100TermKeyBindings {
    /// Create new keybindings with defaults
    pub fn new() -> Self {
        Self::default()
    }
}
