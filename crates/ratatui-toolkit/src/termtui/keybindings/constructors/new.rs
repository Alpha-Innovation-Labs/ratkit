//! Constructor for TermTuiKeyBindings

use crate::termtui::keybindings::TermTuiKeyBindings;

impl TermTuiKeyBindings {
    /// Create a new keybindings configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
}
